// #![windows_subsystem = "windows"]
#![feature(let_chains)]
use std::ffi::c_void;

use anyhow::Result;
use educe::Educe;
use egui::Id;
use licenser::LicenserWindow;
use log::{error, warn};
use mythware::MythwareWindow;
use powershadow::PowerShadowWindow;
use raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE},
};
use windowsadj::WindowsAdjWindow;
use yjyz_tools::license::{self, LicenseFeatures};

mod assets;
mod licenser;
mod mythware;
mod powershadow;
mod utils;
mod windowsadj;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("YJYZTOOLS_DEBUG").is_ok() || cfg!(debug_assertions) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        std::env::set_var("RUST_BACKTRACE", "true");
    }
    env_logger::init();
    log_panics::init();

    if *license::IS_SUDOER {
        warn!("Sudoer mode is set");
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
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)?))),
    )
    .unwrap();

    Ok(())
}

#[derive(Educe)]
#[educe(Default)]
struct MainApp {
    #[educe(Default = None)]
    error: Option<anyhow::Error>,
    #[educe(Default = false)]
    double_error: bool,
    show_licenses: bool,
    mythware_open: bool,
    mythware: MythwareWindow,
    #[educe(Default = true)]
    always_on_top: bool,
    #[educe(Default = false)]
    prevent_screenshot: bool,
    windows_adj_open: bool,
    windows_adj: WindowsAdjWindow,
    powershadow_open: bool,
    powershadow: PowerShadowWindow,
    licenser_open: bool,
    licenser: LicenserWindow,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Default::default())
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
        ctx.request_repaint_after_secs(0.3);
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

        if self.always_on_top {
            ctx.request_repaint_after_secs(0.04);
            unsafe {
                let hwnd = HWND(
                    ctx.data(|data| data.get_temp::<usize>(Id::new(DATA_WINDOW_HWND)).unwrap())
                        as *mut c_void,
                );
                SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)?;
            }
        }

        if license::is_set(LicenseFeatures::MYTHWARE_WINDOWING) {
            if self.mythware.auto_windowing_broadcast
                && mythware::is_broadcast_fullscreen().unwrap_or(false)
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                mythware::toggle_broadcast_window()?;
            }

            if self.mythware.auto_unlock_keyboard && mythware::is_broadcast_on().unwrap_or(false) {
                ctx.request_repaint_after_secs(0.025);
                mythware::unlock_keyboard()?;
            }
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

                ui.horizontal(|ui| {
                    ui.hyperlink_to("源代码", "https://codeberg.org/xtex/yjyz-tools");
                    if ui.link("开源许可证").clicked() {
                        self.show_licenses = true;
                    }
                });

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
                    if *license::IS_SUDOER {
                        if ui.button("创建许可").clicked() {
                            self.licenser_open = true;
                        }
                    }
                });

                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut self.always_on_top, "自动置顶");
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

                Ok::<(), anyhow::Error>(())
            })
            .inner?;

        Ok(())
    }
}
