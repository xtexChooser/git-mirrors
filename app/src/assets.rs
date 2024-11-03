use std::{fs, io::Read, path::PathBuf};

use anyhow::Result;
use egui::{FontData, FontFamily};

pub const CUBIC11_FONT: &[u8] = include_bytes!("assets/Cubic_11.ttf.xz");
pub const CUBIC11_LICENSE: &str = include_str!("assets/Cubic_11-LICENSE.txt");

pub const LICENSE_STR: &str = include_str!("../../LICENSE");

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
        xz2::read::XzDecoder::new(CUBIC11_FONT).read_to_end(&mut buf)?;
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

pub const MAS_SCRIPT: &str = include_str!("assets/MAS_AIO.cmd");
