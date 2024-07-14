#![windows_subsystem = "windows"]
#![feature(let_chains)]
use std::ffi::c_void;

use anyhow::Result;
use educe::Educe;
use mythware::MythwareWindow;
use raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE},
};

mod assets;
mod mythware;

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

    eframe::run_native(
        "YJYZ Toolkit",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([640.0, 480.0])
                .with_always_on_top(),
            default_theme: eframe::Theme::Dark,
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
    show_licenses: bool,
    mythware_open: bool,
    mythware: MythwareWindow,
    #[educe(Default = true)]
    always_on_top: bool,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Default::default())
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(0.3);

        if self.always_on_top {
            ctx.request_repaint_after_secs(0.04);
            unsafe {
                if let RawWindowHandle::Win32(Win32WindowHandle { hwnd, .. }) =
                    frame.window_handle().unwrap().as_raw()
                {
                    let hwnd = HWND(hwnd.get() as *mut c_void);
                    SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE).unwrap();
                }
            }
        }

        if self.mythware.auto_windowing_broadcast
            && mythware::check_broadcast_fullscreen().unwrap_or(false)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            mythware::toggle_broadcast_window().unwrap();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("YJYZ Tools - {}", env!("CARGO_PKG_VERSION")));
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
            });

            ui.checkbox(&mut self.always_on_top, "自动置顶");

            if mythware::PASSWORD.read().unwrap().is_some() {
                self.mythware.show_password(ui, "极域密码：");
            }

            egui::Window::new("开源许可证")
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
                .show(ctx, |ui| self.mythware.show(ui));
        });
    }
}
