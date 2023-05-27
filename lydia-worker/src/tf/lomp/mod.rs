use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{
    task::JoinHandle,
    time::{interval_at, sleep},
};

use crate::{report::update_report, Bot};

pub fn start_lomp_worker(bot: Arc<Bot>) -> Result<JoinHandle<()>> {
    Ok(tokio::spawn(async move {
        //let interval = interval_at(start, period)
        loop {
            // run every day
            sleep(Duration::from_secs(60 * 60 * 24)).await;
        }
    }))
}
