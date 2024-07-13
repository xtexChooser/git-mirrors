#![windows_subsystem = "windows"]
use anyhow::Result;
use mythware::MythwareWindow;

mod assets;
mod mythware;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("YJYZTOOLS_DEBUG").is_ok() || cfg!(debug_assertions) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    log_panics::init();

    eframe::run_native(
        "YJYZ Toolkit",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
            default_theme: eframe::Theme::Dark,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)?))),
    )
    .unwrap();

    Ok(())
}

#[derive(Default)]
struct MainApp {
    show_licenses: bool,
    mythware_open: bool,
    mythware: MythwareWindow,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Default::default())
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                .default_size((250.0, 200.0))
                .show(ctx, |ui| self.mythware.show(ui));
        });
    }
}
