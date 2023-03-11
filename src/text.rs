use anyhow::Result;
use image::Rgb;
use tracing::{info, warn};

use crate::{
    draw::{draw_pixel, new_socket},
    unifont::{Glyph, FONT},
};

pub async fn draw_text(text: &str, base_x: u16, base_y: u16) -> Result<()> {
    info!(text, x = base_x, y = base_y, "drawing text");
    let mut socket = new_socket()?;
    let mut x = base_x;
    for chr in text.chars() {
        let char_code = u32::from(chr);
        match unsafe { &FONT }.get(&char_code) {
            Some(glyph) => {
                let glyph = Glyph::new(&glyph)?;
                for gx in 0..glyph.width() {
                    for gy in 0..16 {
                        let color = glyph.get_pixel(gx, gy);
                        let gray = if color { 0 } else { 255 };
                        let color = Rgb([gray, gray, gray]);
                        draw_pixel(&mut socket, x, base_y + gy as u16, &color, false).await?;
                    }
                    x += 1;
                }
            }
            None => warn!(
                char = chr.to_string(),
                char_code, "skipping character without glyph"
            ),
        }
        if x >= 512 {
            break;
        }
    }
    Ok(())
}
