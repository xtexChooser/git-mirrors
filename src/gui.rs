use std::{
    env::{self, set_var, var},
    path::Path,
};

use anyhow::Result;
use egui::{Button, Color32, DragValue, Slider};
use tracing::info;

pub async fn run_gui() -> Result<()> {
    info!("run gui");

    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Lumberjack Solver",
        options,
        Box::new(|_| Box::<App>::default()),
    )
    .unwrap();

    Ok(())
}
struct App {
    game_url: String,
    webdriver_url: String,
    ff_addon: String,
    target_record: u32,
    step_score: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_url: var("LUMBERJACK_GAME_URL").unwrap(),
            webdriver_url: var("LUMBERJACK_WEBDRIVER_URL").unwrap(),
            ff_addon: var("LUMBERJACK_FIREFOX_ADDON").unwrap(),
            target_record: var("LUMBERJACK_TARGET_SCORES").unwrap().parse().unwrap(),
            step_score: var("LUMBERJACK_STEP_SCORES").unwrap().parse().unwrap(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lumberjack Solver");
            ui.label(concat!("Version ", env!("CARGO_PKG_VERSION"), " by xtex"));
            let mut can_run = true;

            ui.horizontal(|ui| {
                let label = ui.label("Game URL: ");
                if ui
                    .text_edit_singleline(&mut self.game_url)
                    .labelled_by(label.id)
                    .on_hover_text("The whole URL starts with https://tbot.xyz/lumber/")
                    .changed()
                {
                    set_var("LUMBERJACK_GAME_URL", &self.game_url);
                }
            });
            if !self.game_url.starts_with("https://tbot.xyz/lumber/") {
                ui.colored_label(Color32::RED, "The given game URL does not starts with https://tbot.xyz/lumber/, it may be wrong!");
            }

            ui.horizontal(|ui| {
                let label = ui.label("WebDriver URL: ");
                if ui
                    .text_edit_singleline(&mut self.webdriver_url)
                    .labelled_by(label.id)
                    .on_hover_text("The URL for WebDriver to use.\nOnly FireFox (geckodriver) is supported.")
                    .changed()
                {
                    set_var("LUMBERJACK_WEBDRIVER_URL", &self.webdriver_url);
                }
                if ui.link("Get Geckodriver").clicked() {
                    ui.output_mut(|o| o.open_url("https://github.com/mozilla/geckodriver/releases"));
                }
            });

            ui.horizontal(|ui| {
                let label = ui.label("FF Addon: ");
                if ui
                    .text_edit_singleline(&mut self.ff_addon)
                    .labelled_by(label.id)
                    .on_hover_text("The whole URL starts with https://tbot.xyz/lumber/")
                    .changed()
                {
                    set_var("LUMBERJACK_FIREFOX_ADDON", &self.ff_addon);
                }
            });
            if self.ff_addon.is_empty() || !Path::new(&self.ff_addon).exists() {
                ui.colored_label(Color32::RED, "Please specify a correct FF Addon path. The file can be built by running 'make -C ff_addon' in the source.");
                can_run = false;
            }

            ui.horizontal(|ui| {
                let label = ui.label("Target Scores: ");
                if ui.add(DragValue::new(&mut self.target_record))
                    .labelled_by(label.id)
                    .changed()
                {
                    set_var("LUMBERJACK_TARGET_SCORES", &self.target_record.to_string());
                }
            });

            ui.horizontal(|ui| {
                let label = ui.label("Step Scores: ");
                if ui.add(Slider::new(&mut self.step_score, 1..=200))
                    .labelled_by(label.id)
                    .on_hover_text("DO NOT change this unless you know what are you doing")
                    .changed()
                {
                    set_var("LUMBERJACK_STEP_SCORES", &self.step_score.to_string());
                }
            });

            ui.group(|ui| {
                ui.heading("All Options");
                let mut vars = env::vars();
                let mut text = String::new();
                for (k, v ) in &mut vars {
                    if k.starts_with("LUMBERJACK_") {
                        text.push_str(&format!("{}={}\n", k, v));
                    }
                }
                ui.code(text);
            });

            if ui.add_enabled(can_run, Button::new("Run")).clicked() {
                frame.close();
            }
        });
    }
}
