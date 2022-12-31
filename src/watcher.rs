use anyhow::Result;
use etcd_client::{Event, EventType, WatchOptions};

use crate::{etcd::get_etcd_client, zone::Zone};

pub async fn watch_zone(zone: &mut Zone) -> Result<()> {
    let (_watcher, mut stream) = {
        get_etcd_client()
            .await?
            .watch(
                zone.conf.etcd_prefix.as_str(),
                Some(WatchOptions::new().with_prefix()),
            )
            .await?
    };
    info!("watching for changes in zone {}", zone.conf.name);
    while let Some(resp) = stream.message().await? {
        for event in resp.events() {
            info!(
                "etcd watcher event, msg id: {}, type: {:?}, key: {:?}, prev key: {:?}",
                resp.watch_id(),
                event.event_type(),
                event.kv().and_then(|kv| kv.key_str().ok()),
                event.prev_kv().and_then(|kv| kv.key_str().ok()),
            );

            if let Err(err) = handle_event(zone, event).await {
                error!(
                    "failed to handle etcd watcher event in {}: {}",
                    resp.watch_id(),
                    err
                )
            }
        }
    }
    Ok(())
}

pub async fn handle_event(zone: &mut Zone, event: &Event) -> Result<()> {
    let prefix = &zone.conf.etcd_prefix;
    match event.event_type() {
        EventType::Put => {
            if let Some(kv) = event.kv() {
                let mut key = kv.key_str()?.to_string();
                key.drain(0..prefix.len());
                zone.handle_peer(key.as_str()).await?;
            }
        }
        EventType::Delete => {
            if let Some(kv) = event.kv() {
                let mut key = kv.key_str()?.to_string();
                key.drain(0..prefix.len());
                zone.remove_peer(key.as_str()).await?;
            }
        }
    }
    Ok(())
}
