#![feature(slice_group_by)]

use std::{cmp::max, collections::BTreeMap, env, path::PathBuf, sync::Arc};

use anyhow::Result;
use build_clean::{
    db::{self, info::CacheTypeRef, LUA},
    search,
};
use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Checkbox, Dialog, EditView, LinearLayout, ListView, Panel, SliderView, TextView},
    Cursive, With,
};
use cursive_async_view::{AsyncState, AsyncView};
use tokio::{
    sync::oneshot::{self, error::TryRecvError},
    time::Instant,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    init_lua(&mut siv)?;
    siv.run();

    Ok(())
}

fn init_lua(siv: &mut Cursive) -> Result<()> {
    siv.add_layer(TextView::new("Loading Lua scripts"));
    let cb = siv.cb_sink().clone();
    tokio::spawn(async move {
        db::init_lua().await.unwrap();
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
                                TextView::new(workers.to_string()).with_name("worker_count_text"),
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
                        show_result(siv, result.clone(), default_action).unwrap()
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
) -> Result<()> {
    siv.pop_layer();
    let lua = LUA.lock();
    let result = result.clone();
    let mut list = LinearLayout::vertical();
    unsafe {
        ACTION = Some(
            result
                .iter()
                .map(|(path, _)| (path.clone(), default_action))
                .collect::<BTreeMap<_, _>>(),
        );
    }
    let result = result.group_by(|(_, r1), (_, r2)| r1 == r2);

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
    siv.add_layer(Dialog::new().title("Result").content(list.scrollable()));
    Ok(())
}
