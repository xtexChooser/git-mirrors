// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(let_chains)]
#![feature(path_add_extension)]
use std::{
    cmp, fs,
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::Result;
use educe::Educe;
use egui::Id;
use licenser::LicenserWindow;
use log::{error, info, warn};
use mythware::MythwareWindow;
use powershadow::PowerShadowWindow;
use raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle};
use updater::Updater;
use windowsadj::WindowsAdjWindow;
use worker::WorkerState;
use yjyz_tools::license::{self, LicenseFeatures};

mod assets;
mod licenser;
mod mythware;
mod powershadow;
mod sec;
mod updater;
mod utils;
mod windowsadj;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    if std::env::var("YJYZTOOLS_DEBUG").is_ok() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        std::env::set_var("RUST_BACKTRACE", "true");
    }
    env_logger::init();
    log_panics::init();

    if *mythware::SETUP_TYPE == Some(mythware::SetupType::Teacher)
        && !license::is_set(LicenseFeatures::MYTHWARE_ALLOW_TEACHER)
    {
        fs::remove_file(std::env::current_exe()?)?;
        panic!("unavailable: 4f82b84f-481a-426d-8361-795008110549")
    }

    if !license::is_set(LicenseFeatures::NO_SECURITY_CHECK) {
        info!("Invoking runtime safety subsystem");
        if let Err(err) = sec::check_environment() {
            if license::is_set(LicenseFeatures::ALLOW_INSECURE) {
                warn!("ins. env.: {}", err.to_string());
            } else {
                panic!("unavailable: e1c62299-ae2c-4261-88a2-4c2211caedeb: {}", err)
            }
        } else {
            info!("Runtime environment is safe")
        }
    } else {
        info!("Bypassing safety check")
    }

    if *license::IS_SUDOER {
        warn!("Sudoer mode is set");
    }

    if !license::is_set(LicenseFeatures::NO_UPDATE) {
        tokio::spawn(async {
            if let Err(err) = updater::check().await {
                let _ = ASYNC_ERROR.write().unwrap().insert(err);
            }
        });
    }

    let worker_state = Arc::new(RwLock::new(WorkerState::default()));

    {
        let worker_state = worker_state.clone();
        tokio::spawn(async move {
            if let Err(err) = worker::run(worker_state).await {
                let _ = ASYNC_ERROR.write().unwrap().insert(err);
            }
        });
    }

    eframe::run_native(
        "YJYZ Toolkit",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 450.0])
                .with_always_on_top(),
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(MainApp::new(cc, worker_state)?))),
    )
    .unwrap();

    Ok(())
}

static ASYNC_ERROR: RwLock<Option<anyhow::Error>> = RwLock::new(None);

#[derive(Educe)]
#[educe(Default)]
struct MainApp {
    #[educe(Default = None)]
    error: Option<anyhow::Error>,
    #[educe(Default = false)]
    double_error: bool,
    show_licenses: bool,

    worker: Arc<RwLock<WorkerState>>,

    #[educe(Default = true)]
    always_on_top: bool,
    #[educe(Default = false)]
    prevent_screenshot: bool,

    update: Updater,

    #[educe(Default = false)]
    mythware_open: bool,
    mythware: MythwareWindow,

    #[educe(Default = false)]
    windows_adj_open: bool,
    windows_adj: WindowsAdjWindow,

    #[educe(Default = false)]
    powershadow_open: bool,
    powershadow: PowerShadowWindow,

    #[educe(Default = false)]
    licenser_open: bool,
    licenser: LicenserWindow,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>, worker: Arc<RwLock<WorkerState>>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Self {
            worker,
            ..Default::default()
        })
    }
}

const DATA_WINDOW_HWND: u64 = 0xc4dbc123bb779f78;

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Err(err) = self.show(ctx, frame) {
            error!("ui error: {err:?}");
            if self.error.is_none() {
                self.error = Some(err);
                ctx.request_repaint();
            } else if !self.double_error {
                self.double_error = true;
                ctx.request_repaint();
            } else {
                panic!("triple error");
            }
        }
    }
}

impl MainApp {
    fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Result<()> {
        let mut repaint_after = 300;
        ctx.style_mut(|style| style.url_in_tooltip = true);
        ctx.data_mut(|data| {
            data.get_temp_mut_or_insert_with(Id::new(DATA_WINDOW_HWND), || {
                if let RawWindowHandle::Win32(Win32WindowHandle { hwnd, .. }) =
                    frame.window_handle().unwrap().as_raw()
                {
                    hwnd.get() as usize
                } else {
                    panic!("not win32 window")
                }
            });
        });

        if license::LICENSES.is_empty() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(format!("YJYZ Tools - {}", env!("CARGO_PKG_VERSION")));
                ui.heading("找不到许可文件");
                ui.label("在此设备上找不到有效的许可文件。");
            });
            return Ok(());
        }

        if license::is_set(LicenseFeatures::MYTHWARE_WINDOWING) {
            if self.mythware.auto_windowing_broadcast
                && mythware::is_broadcast_fullscreen().unwrap_or(false)
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                mythware::toggle_broadcast_window()?;
            }

            if self.mythware.auto_unlock_keyboard && mythware::is_broadcast_on().unwrap_or(false) {
                repaint_after = cmp::min(repaint_after, 25);
                mythware::unlock_keyboard()?;
            }
        }

        ctx.request_repaint_after(Duration::from_millis(repaint_after));
        if let Some(err) = ASYNC_ERROR.write().unwrap().take() {
            return Err(err);
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading(format!("YJYZ Tools - {}", env!("CARGO_PKG_VERSION")));
                let mut clear_error = false;
                if let Some(err) = &self.error {
                    egui::Window::new("错误")
                        .collapsible(false)
                        .scroll(true)
                        .default_size((600.0, 450.0))
                        .default_pos((0.0, 40.0))
                        .show(ctx, |ui| {
                            if self.double_error {
                                ui.heading("存在双重错误");
                            }
                            if !self.double_error && ui.button("忽略").clicked() {
                                clear_error = true;
                            }
                            ui.code(format!("{:?}", err));
                        });
                    if self.double_error {
                        return Ok(());
                    }
                }
                if clear_error {
                    self.error = None;
                    ui.ctx().request_repaint();
                }

                if license::is_set(LicenseFeatures::SHOW_SOURCE_LINK) {
                    ui.horizontal(|ui| {
                        ui.hyperlink_to("源代码", "https://codeberg.org/xtex/yjyz-tools");
                        if ui.link("开源许可证").clicked() {
                            self.show_licenses = true;
                        }
                    });
                }

                if license::is_set(LicenseFeatures::MUST_UPDATE) && self.update.should_show() {
                    self.update.show(ui)?;
                    return Ok(());
                }

                egui::menu::bar(ui, |ui| {
                    if ui.button("极域").clicked() {
                        self.mythware_open = true;
                    }
                    if ui.button("系统工具").clicked() {
                        self.windows_adj_open = true;
                    }
                    if ui.button("影子系统").clicked() {
                        self.powershadow_open = true;
                    }
                    if *license::IS_SUDOER && ui.button("创建许可").clicked() {
                        self.licenser_open = true;
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    if ui.checkbox(&mut self.always_on_top, "自动置顶").changed() {
                        if self.always_on_top {
                            self.worker.write().unwrap().always_on_top = Some(ctx.data(|data| {
                                data.get_temp::<usize>(Id::new(DATA_WINDOW_HWND)).unwrap()
                            }));
                        } else {
                            self.worker.write().unwrap().always_on_top = None;
                        }
                    }
                    if ui
                        .checkbox(&mut self.prevent_screenshot, "防止截屏")
                        .changed()
                    {
                        utils::prevent_screenshot(ui.ctx(), self.prevent_screenshot)?;
                    }
                    Ok::<(), anyhow::Error>(())
                })
                .inner?;

                if mythware::PASSWORD.read().unwrap().is_some() {
                    self.mythware.show_password(ui, "极域密码：")?;
                }

                ui.vertical(|ui| {
                    ui.label("已加载许可文件：");
                    for claims in license::LICENSES.iter() {
                        ui.label(format!("- {}", claims.id));
                    }
                });

                egui::Window::new("开源许可")
                    .open(&mut self.show_licenses)
                    .vscroll(true)
                    .default_size((320.0, 200.0))
                    .show(ctx, |ui| {
                        ui.label(assets::LICENSE_STR);
                        ui.heading("Cubic-11");
                        ui.label(assets::CUBIC11_LICENSE);
                    });

                egui::Window::new("极域")
                    .open(&mut self.mythware_open)
                    .vscroll(true)
                    .default_size((300.0, 200.0))
                    .show(ctx, |ui| self.mythware.show(ui))
                    .map(|o| o.inner)
                    .unwrap_or_default()
                    .unwrap_or(Ok(()))?;

                egui::Window::new("Windows工具")
                    .open(&mut self.windows_adj_open)
                    .vscroll(true)
                    .default_size((300.0, 300.0))
                    .show(ctx, |ui| self.windows_adj.show(ui))
                    .map(|o| o.inner)
                    .unwrap_or_default()
                    .unwrap_or(Ok(()))?;

                egui::Window::new("影子系统")
                    .open(&mut self.powershadow_open)
                    .vscroll(true)
                    .default_size((170.0, 130.0))
                    .show(ctx, |ui| self.powershadow.show(ui))
                    .map(|o| o.inner)
                    .unwrap_or_default()
                    .unwrap_or(Ok(()))?;

                egui::Window::new("许可生成")
                    .open(&mut self.licenser_open)
                    .vscroll(true)
                    .default_size((250.0, 300.0))
                    .show(ctx, |ui| self.licenser.show(ui))
                    .map(|o| o.inner)
                    .unwrap_or_default()
                    .unwrap_or(Ok(()))?;

                if self.update.should_show() {
                    egui::Window::new("更新")
                        .vscroll(true)
                        .default_size((250.0, 300.0))
                        .show(ctx, |ui| self.update.show(ui))
                        .map(|o| o.inner)
                        .unwrap_or_default()
                        .unwrap_or(Ok(()))?;
                }

                Ok::<(), anyhow::Error>(())
            })
            .inner?;

        Ok(())
    }
}
