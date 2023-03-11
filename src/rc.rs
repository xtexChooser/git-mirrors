use std::{collections::HashMap, time::Duration};

use anyhow::{Context, Error, Result};
use image::Rgb;
use tracing::{error, trace};

use crate::{
    draw::{fill_rect, new_socket},
    text::draw_text,
    unifont,
};

pub async fn draw_rc(title: &str, api: &str, base_x: u16, base_y: u16) -> Result<()> {
    fill_rect(
        &mut new_socket()?,
        base_x,
        base_y,
        250,
        56,
        &Rgb([0xcc, 0xcc, 0xcc]),
    )
    .await?;
    let client = reqwest::Client::builder()
        .https_only(true)
        .timeout(Duration::from_secs(5))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let rc_params = HashMap::from([
        ("action", "query"),
        ("format", "json"),
        ("list", "recentchanges"),
        ("formatversion", "2"),
        ("rcprop", "title|user|sizes"),
        ("rcshow", "!bot"),
        ("rclimit", "2"),
        ("rctoponly", "1"),
    ]);

    let mut last_resp = String::new();

    loop {
        match (async || -> Result<String> {
            let resp_str = client
                .post(api)
                .form(&rc_params)
                .send()
                .await?
                .text()
                .await?;

            if resp_str == last_resp {
                trace!("nothing changed, skipping updating");
                return Ok(resp_str);
            } else {
                draw_text(title, base_x, base_y).await?;
            }

            let resp: serde_json::Value =
                serde_json::from_str(&resp_str).with_context(|| resp_str.to_owned())?;

            let rcs = resp["query"]["recentchanges"]
                .as_array()
                .ok_or(Error::msg("$.query.recentchanges is not array"))?;
            let mut y = base_y;
            for rc in rcs.iter() {
                y += unifont::GLYPH_HEIGHT;
                let rc = rc.as_object().ok_or(Error::msg("rc is not obj"))?;
                let title = &rc["title"]
                    .as_str()
                    .ok_or(Error::msg("rc.title is not str"))?;
                let user = &rc["user"]
                    .as_str()
                    .ok_or(Error::msg("rc.user is not str"))?;
                let oldlen = &rc["oldlen"].as_i64();
                let newlen = &rc["newlen"].as_i64();
                let len = if let Some(oldlen) = oldlen && let Some(newlen) = newlen {
                                      Some(newlen - oldlen)
                                  } else {
                                      None
                                  };
                let lenstr = if let Some(len) = len {
                    format!(" {len:+}")
                } else {
                    "".to_string()
                };
                let msg = format!("{} (User: {}{})", title, user, lenstr);
                draw_text(&msg, base_x, y).await?;
            }

            tokio::time::sleep(Duration::from_secs(3)).await;

            Ok(resp_str)
        })()
        .await
        {
            Err(e) => error!(err = e.to_string(), "{}", title),
            Ok(resp) => last_resp = resp,
        }
    }
}
