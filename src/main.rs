#![feature(slice_group_by)]
#![feature(exact_size_is_empty)]
#![feature(async_closure)]

use std::{cmp::max, collections::BTreeMap, env, path::PathBuf, sync::Arc};

use anyhow::Result;
use build_clean::{
    db::{self, info::CacheTypeRef, LUA},
    search,
};
use cursive::{
    reexports::ahash::{HashMap, HashMapExt},
    view::{Nameable, Resizable, Scrollable},
    views::{
        Checkbox, Dialog, EditView, LinearLayout, ListView, Panel, SelectView, SliderView, TextView,
    },
    Cursive, With,
};
use cursive_async_view::{AsyncState, AsyncView};
use mlua::Lua;
use owo_colors::{
    colors::{css::LightGreen, Green, Red, Yellow},
    AnsiColors, OwoColorize,
};
use parking_lot::{Mutex, RwLock};
use tokio::{
    sync::oneshot::{self, error::TryRecvError},
    task::JoinSet,
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct CleanOptions {
    target: Option<HashMap<&'static PathBuf, CacheTypeRef>>,
    clean_type: bool,
    workers: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let (tx, mut rx) = oneshot::channel::<CleanOptions>();
    siv.set_user_data(tx);

    show_init_lua(&mut siv)?;

    siv.run();

    if let Ok(mut opts) = rx.try_recv() {
        let target = opts.target.take().unwrap();
        println!(
            "Clean starting, total {}/{} selected caches",
            target.len(),
            unsafe { ACTION.as_ref() }.unwrap().len()
        );
        println!(
            "- Clean Type: {}",
            if opts.clean_type { "Fast" } else { "Safe" }
        );

        let mut handles = JoinSet::new();
        let target = Arc::new(Mutex::new(
            target
                .iter()
                .map(|(path, type_ref)| ((*path).clone(), type_ref.clone()))
                .collect::<HashMap<_, _>>()
                .into_iter(),
        ));

        let clean_type = opts.clean_type;
        let error_count = Arc::new(RwLock::new(0usize));
        for _ in 0..opts.workers {
            let target = target.clone();
            let error_count = error_count.clone();
            handles.spawn(async move || -> Result<()> {
                let lua = Lua::new();
                db::init_lua(&lua)?;
                while let Some((path, type_ref)) = target.lock().next() {
                    let resolved = type_ref.resolve(&lua)?;
                    let t0 = Instant::now();
                    println!(
                        "{}{}{}{}{}",
                        "- ".fg::<Green>(),
                        "Cleaning ".fg::<Green>().bold(),
                        path.display(),
                        " as ".fg::<Green>(),
                        resolved.get_name()?.fg::<Yellow>(),
                    );
                    let result = if clean_type {
                        resolved.fast_clean(&path)
                    } else {
                        resolved.clean(&path)
                    };
                    match result {
                        Ok(_) => {
                            let time = t0.elapsed();
                            println!(
                                "{} {} {} {} {}",
                                "-".fg::<Green>(),
                                "Cleaned".fg::<Green>().bold(),
                                path.display(),
                                "in".fg::<Green>(),
                                if time.as_secs() == 0 {
                                    format!("{}ms", time.as_millis())
                                } else {
                                    format!("{}s", time.as_secs())
                                }
                                .color(match time.as_millis() {
                                    ..=200 => AnsiColors::Green,
                                    201..=1000 => AnsiColors::Yellow,
                                    _ => AnsiColors::Red,
                                },)
                            );
                        }
                        Err(err) => {
                            println!(
                                "{}",
                                format!("- Error in {}\n{}", path.display(), err.to_string())
                                    .fg::<Red>()
                                    .bold()
                            );
                            *error_count.write() += 1;
                        }
                    }
                }
                Ok(())
            }());
        }

        while let Some(val) = handles.join_next().await {
            let _ = val?;
        }

        let error_count = *error_count.read();
        if error_count == 0 {
            println!("{}", "CLEAN SUCCESSFUL!".fg::<LightGreen>().bold());
        } else {
            println!(
                "{}",
                format!("CLEAN FAILED! {} errors occurred", error_count)
                    .fg::<Red>()
                    .bold()
            );
        }
    }

    Ok(())
}

fn show_init_lua(siv: &mut Cursive) -> Result<()> {
    siv.add_layer(TextView::new("Loading Lua scripts"));
    let cb = siv.cb_sink().clone();
    tokio::spawn(async move {
        let lua = LUA.lock();
        db::init_lua(&lua).unwrap();
        cb.send(Box::new(|siv| {
            siv.pop_layer();
            show_main_menu(siv).unwrap();
        }))
        .unwrap();
    });
    Ok(())
}

fn show_main_menu(siv: &mut Cursive) -> Result<()> {
    let max_workers = num_cpus::get() * 2;
    let workers = max(num_cpus::get() as isize - 2, 2) as usize;
    siv.add_layer(
        Dialog::new()
            .title("Build Clean")
            .content(
                ListView::new()
                    .child(
                        "Base Path",
                        EditView::new()
                            .content(env::current_dir().unwrap().to_str().unwrap())
                            .with_name("base_path"),
                    )
                    .child(
                        "Worker Threads",
                        LinearLayout::horizontal()
                            .child(
                                SliderView::horizontal(max_workers)
                                    .value(workers)
                                    .on_change(|siv, val| {
                                        siv.call_on_name(
                                            "worker_count_text",
                                            |view: &mut TextView| {
                                                view.set_content((val + 1).to_string())
                                            },
                                        );
                                    })
                                    .with_name("worker_count"),
                            )
                            .child(
                                TextView::new((workers + 1).to_string())
                                    .with_name("worker_count_text"),
                            ),
                    )
                    .child(
                        "Balancer Depth",
                        LinearLayout::horizontal()
                            .child(
                                SliderView::horizontal(16)
                                    .value(6)
                                    .on_change(|siv, val| {
                                        siv.call_on_name(
                                            "queue_depth_text",
                                            |view: &mut TextView| {
                                                view.set_content((val + 1).to_string())
                                            },
                                        );
                                    })
                                    .with_name("queue_depth"),
                            )
                            .child(TextView::new("6").with_name("queue_depth_text")),
                    )
                    .child(
                        "Default Action",
                        Checkbox::new().checked().with_name("default_action"),
                    )
                    .child(
                        "Clean Type",
                        SelectView::new()
                            .popup()
                            .item("Fast (delete files directly)", true)
                            .item("Safe (try cleaning with builder)", false)
                            .selected(1)
                            .with_name("clean_type"),
                    )
                    .child(
                        "Cleaning Threads",
                        LinearLayout::horizontal()
                            .child(
                                SliderView::horizontal(max_workers)
                                    .value(workers)
                                    .on_change(|siv, val| {
                                        siv.call_on_name(
                                            "clean_workers_text",
                                            |view: &mut TextView| {
                                                view.set_content((val + 1).to_string())
                                            },
                                        );
                                    })
                                    .with_name("clean_workers"),
                            )
                            .child(
                                TextView::new((workers + 1).to_string())
                                    .with_name("clean_workers_text"),
                            ),
                    )
                    .full_width()
                    .scrollable(),
            )
            .button("Run", |siv| {
                run(siv).unwrap();
            })
            .button("Quit", |siv| {
                siv.quit();
            }),
    );
    Ok(())
}

fn run(siv: &mut Cursive) -> Result<()> {
    let search_base = siv
        .call_on_name("base_path", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string();
    let worker_count = siv
        .call_on_name("worker_count", |view: &mut SliderView| view.get_value())
        .unwrap()
        + 1;
    let queue_depth = siv
        .call_on_name("queue_depth", |view: &mut SliderView| view.get_value())
        .unwrap()
        + 1;
    let default_action = siv
        .call_on_name("default_action", |view: &mut Checkbox| view.is_checked())
        .unwrap();
    let clean_type = *siv
        .call_on_name("clean_type", |view: &mut SelectView<bool>| view.selection())
        .unwrap()
        .unwrap();
    let clean_workers = siv
        .call_on_name("clean_workers", |view: &mut SliderView| view.get_value())
        .unwrap()
        + 1;
    siv.pop_layer();

    let (tx, mut rx) = oneshot::channel();
    let t0 = Instant::now();
    tokio::spawn(async move {
        tx.send(
            search::search(search_base.into(), worker_count, queue_depth)
                .await
                .map(|mut result| {
                    result.sort_by(|(p1, _), (p2, _)| p1.cmp(p2));
                    let mut result1 = vec![];
                    for result in result.into_iter() {
                        match result1.last() {
                            Some((path, _)) => {
                                if !result.0.starts_with(path) {
                                    result1.push(result)
                                }
                            }
                            None => result1.push(result),
                        }
                    }
                    let mut result = result1;
                    result.sort_by(|(_, r1), (_, r2)| r1.cmp(r2));
                    result
                }),
        )
        .unwrap();
    });

    let cb = siv.cb_sink().clone();
    let async_view = AsyncView::new(siv, move || match rx.try_recv() {
        Ok(result) => {
            let result = Arc::new(result.unwrap());
            AsyncState::Available(
                Dialog::text(format!("Found {} results", result.len()))
                    .title("Result")
                    .button("Continue", move |siv| {
                        siv.pop_layer();
                        show_result(
                            siv,
                            result.clone(),
                            default_action,
                            clean_type,
                            clean_workers,
                        )
                        .unwrap()
                    }),
            )
        }
        Err(TryRecvError::Empty) => {
            cb.send(Box::new(move |siv| {
                siv.call_on_name("scan_timer", |view: &mut TextView| {
                    let elapsed = t0.elapsed();
                    if elapsed.as_secs() != 0 {
                        view.set_content(format!("Took {}s", elapsed.as_secs()))
                    } else {
                        view.set_content(format!("Took {}ms", elapsed.as_millis()))
                    }
                });
            }))
            .unwrap();
            AsyncState::Pending
        }
        Err(TryRecvError::Closed) => panic!(),
    });

    siv.add_layer(
        Dialog::new().title("Scanning").content(
            LinearLayout::vertical()
                .child(async_view)
                .child(TextView::new("Waiting").with_name("scan_timer")),
        ),
    );
    Ok(())
}

static mut ACTION: Option<BTreeMap<PathBuf, bool>> = None;

fn show_result(
    siv: &mut Cursive,
    result: Arc<Vec<(PathBuf, CacheTypeRef)>>,
    default_action: bool,
    clean_type: bool,
    workers: usize,
) -> Result<()> {
    siv.pop_layer();
    let lua = LUA.lock();
    let result0 = result;
    let mut list = LinearLayout::vertical();
    unsafe {
        ACTION = Some(
            result0
                .iter()
                .map(|(path, _)| (path.clone(), default_action))
                .collect::<BTreeMap<_, _>>(),
        );
    }
    let result = result0.group_by(|(_, r1), (_, r2)| r1 == r2);

    for result in result {
        let type_ref = &result[0].1;
        let resolved = type_ref.resolve(&lua).unwrap();
        let mut group = ListView::new();

        for (path, _) in result {
            let path = path.clone();
            group.add_child(
                &resolved.to_display(&path).unwrap().display().to_string(),
                Checkbox::new()
                    .with(|view| {
                        view.set_checked(default_action);
                    })
                    .on_change(move |_, checked| unsafe {
                        ACTION.as_mut().unwrap().insert(path.clone(), checked);
                    }),
            );
        }

        list.add_child(
            Panel::new(group)
                .title(resolved.get_name().unwrap())
                .full_width(),
        );
    }
    siv.add_layer(
        Dialog::new()
            .title("Result")
            .content(list.scrollable())
            .button("Clean", move |siv| {
                run_clean(siv, result0.clone(), clean_type, workers).unwrap();
            }),
    );
    Ok(())
}

fn run_clean(
    siv: &mut Cursive,
    result: Arc<Vec<(PathBuf, CacheTypeRef)>>,
    clean_type: bool,
    workers: usize,
) -> Result<()> {
    siv.quit();

    let mut types = HashMap::new();
    for (path, type_ref) in result.iter() {
        types.insert(path, type_ref);
    }
    let selected = unsafe { ACTION.as_ref() }
        .unwrap()
        .iter()
        .filter_map(|(k, v)| {
            if *v {
                Some((k, (*types.get(k).unwrap()).clone()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let tx = siv
        .take_user_data::<oneshot::Sender<CleanOptions>>()
        .unwrap();
    tx.send(CleanOptions {
        target: Some(selected),
        clean_type,
        workers,
    })
    .unwrap();

    Ok(())
}
