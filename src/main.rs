#![windows_subsystem = "windows"]
use anyhow::Result;

mod assets;

#[tokio::main]
async fn main() -> Result<()> {
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
}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        assets::configure_fonts(&cc.egui_ctx)?;
        Ok(Self {
            show_licenses: false,
        })
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("YJYZ Tools - {}", env!("CARGO_PKG_VERSION")));
            ui.horizontal(|ui| {
                if ui.link("源代码").clicked() {
                    ctx.open_url(egui::OpenUrl::new_tab(
                        "https://codeberg.org/xtex/yjyz-tools",
                    ));
                }
                if ui.link("开源许可证").clicked() {
                    self.show_licenses = true;
                }
            });

            egui::menu::bar(ui, |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            egui::Window::new("开源许可证")
                .open(&mut self.show_licenses)
                .vscroll(true)
                .default_size((320.0, 200.0))
                .show(ctx, |ui| {
                    ui.label(assets::LICENSE_STR);
                    ui.heading("Cubic-11");
                    ui.label(assets::CUBIC11_LICENSE);
                });
        });
    }
}
