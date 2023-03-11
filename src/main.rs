use anyhow::Result;
use draw::{draw_rc, draw_text};

mod draw;
mod unifont;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt().finish())?;
    unifont::download().await?;
    unsafe {
        unifont::FONT = unifont::read().await?;
    }
    tokio::join!(
        draw_branding(),
        draw_zhwiki(),
        draw_enwiki(),
        draw_metawiki()
    );
    Ok(())
}

async fn draw_branding() {
    loop {
        draw_text("By XTEX-VNET AS4242420361", 400, 300)
            .await
            .unwrap();
    }
}

async fn draw_zhwiki() {
    draw_rc("WP zh", "https://zh.wikipedia.org/w/api.php", 400, 350)
        .await
        .unwrap();
}

async fn draw_enwiki() {
    draw_rc("WP en", "https://en.wikipedia.org/w/api.php", 400, 400)
        .await
        .unwrap();
}

async fn draw_metawiki() {
    draw_rc("WP meta", "https://meta.wikipedia.org/w/api.php", 400, 450)
        .await
        .unwrap();
}
