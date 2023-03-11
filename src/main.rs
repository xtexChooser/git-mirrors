#![feature(let_chains)]
#![feature(async_closure)]

use std::time::Duration;

use anyhow::Result;
use rc::draw_rc;
use text::draw_text;

mod rc;
mod text;
mod unifont;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt().finish())?;
    unifont::download().await?;
    unsafe {
        unifont::FONT = unifont::read().await?;
    }
    tokio::spawn(draw_branding());
    tokio::spawn(draw_zhwiki());
    tokio::spawn(draw_enwiki());
    tokio::spawn(draw_metawiki());
    loop {}
}

async fn draw_branding() {
    loop {
        draw_text("By XTEX-VNET AS4242420361", 400, 300)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
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
    draw_rc("WP meta", "https://meta.wikimedia.org/w/api.php", 400, 450)
        .await
        .unwrap();
}
