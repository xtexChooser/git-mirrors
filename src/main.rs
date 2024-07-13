#![windows_subsystem = "windows"]

use std::{fs, io::Read, path::PathBuf};

use anyhow::Result;
use egui::{FontData, FontFamily};

#[tokio::main]
async fn main() -> Result<()> {
    eframe::run_native(
        "My egui App",
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

struct MainApp {}

impl MainApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        Self::configure_fonts(&cc.egui_ctx)?;
        Ok(Self {})
    }

    pub fn configure_fonts(ctx: &eframe::egui::Context) -> Result<()> {
        ctx.set_zoom_factor(1.2);
        let mut fonts = egui::FontDefinitions::default();
        {
            // load msyh
            let path = PathBuf::from(std::env::var("SystemRoot")?).join("Fonts/msyh.ttc");
            if path.exists() {
                fonts
                    .font_data
                    .insert("msyh".to_owned(), FontData::from_owned(fs::read(path)?));
                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "msyh".to_owned());
                fonts
                    .families
                    .get_mut(&FontFamily::Monospace)
                    .unwrap()
                    .push("msyh".to_owned());
            }
        }

        {
            // load Cubic-11
            let mut buf = Vec::new();
            xz2::read::XzDecoder::new(&include_bytes!("assets/Cubic_11_1.300_R.ttf.xz")[..])
                .read_to_end(&mut buf)?;
            fonts
                .font_data
                .insert("cubic11".to_owned(), FontData::from_owned(buf));
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "cubic11".to_owned());
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "cubic11".to_owned());
        }

        ctx.set_fonts(fonts);
        Ok(())
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("YJYZ Tools - {}", env!("CARGO_PKG_VERSION")));
            if ui.link("源代码").clicked() {
                ctx.open_url(egui::OpenUrl::new_tab(
                    "https://codeberg.org/xtex/yjyz-tools",
                ));
            }

            egui::menu::bar(ui, |ui| {
                if ui.button("Open").clicked() {
                    // …
                }
            });
            egui::Window::new("My Window")
                .fade_in(false)
                .show(ctx, |ui| {
                    ui.label("Hello World!");
                });
        });
    }
}
