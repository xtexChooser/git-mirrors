#![windows_subsystem = "windows"]
use anyhow::Result;
use egui::RichText;

mod assets;
mod mythware;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("YJYZTOOLS_DEBUG").is_ok() || cfg!(debug_assertions) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        unsafe {
            windows::Win32::System::Console::AllocConsole()?;
        }
    }
    env_logger::init();

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

struct MainApp {
    show_licenses: bool,
    show_mythware: bool,
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Self {
            show_licenses: false,
            show_mythware: false,
        })
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
                    self.show_mythware = true;
                }
            });

            if let Some(password) = mythware::PASSWORD.as_ref() {
                ui.horizontal_wrapped(|ui| {
                    ui.label("极域密码：");
                    if password.is_empty() {
                        ui.label(RichText::new("（空）").italics());
                    } else {
                        ui.label(RichText::new(password).italics());
                    }
                });
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
                .open(&mut self.show_mythware)
                .vscroll(true)
                .default_size((250.0, 200.0))
                .show(ctx, mythware::show_window);
        });
    }
}
