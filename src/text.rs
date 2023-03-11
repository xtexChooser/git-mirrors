use anyhow::Result;
use tracing::info;

pub async fn draw_text(text: &str, base_x: u32, base_y: u32) -> Result<()> {
    info!(text, x = base_x, y = base_y, "drawing text");
    Ok(())
}
