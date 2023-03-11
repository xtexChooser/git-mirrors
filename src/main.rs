#![feature(let_chains)]
#![feature(async_closure)]
#![feature(array_try_map)]

use std::{env, time::Duration};

use anyhow::Result;
use draw::QPS_COUNTER;
use rc::draw_rc;
use text::draw_text;

mod draw;
mod rc;
mod text;
mod unifont;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt().finish())?;
    unifont::download().await?;
    unsafe {
        unifont::FONT = unifont::read().await?;
        draw::PREFIX = env::var("KB_CANVAS_PREFIX").unwrap_or("2a09:b280:ff82:4242".to_string());
    }
    
    tokio::spawn(draw_branding());
    tokio::spawn(draw_zhwiki());
    tokio::spawn(draw_enwiki());
    //tokio::spawn(draw_metawiki());
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        unsafe {
            QPS_COUNTER = 0;
        }
    }
}

const BASE_X: u16 = 310;
const BASE_Y: u16 = 140;

async fn draw_branding() {
    loop {
        draw_text("By XTEX-VNET AS4242420361", BASE_X, BASE_Y)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn draw_zhwiki() {
    draw_rc(
        "WP zh",
        "https://zh.wikipedia.org/w/api.php",
        BASE_X,
        BASE_Y + 56,
    )
    .await
    .unwrap();
}

async fn draw_enwiki() {
    draw_rc(
        "WP en",
        "https://en.wikipedia.org/w/api.php",
        BASE_X,
        BASE_Y + 112,
    )
    .await
    .unwrap();
}

/*async fn draw_metawiki() {
    draw_rc("WP meta", "https://meta.wikimedia.org/w/api.php", 310, 200)
        .await
        .unwrap();
}*/
