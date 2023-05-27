use anyhow::{anyhow, Result};
use webhook::models::Message;

use crate::Bot;

pub async fn send_discord<F>(bot: &Bot, f: F) -> Result<()>
where
    F: Fn(&mut Message) -> &mut Message,
{
    assert!(bot
        .discord
        .send(f)
        .await
        .map_err(|e| anyhow!("failed to delivery notification message: {e:?}"))?);
    Ok(())
}
