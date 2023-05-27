use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, bail, Result};
use env::Env;
use secrets::Secrets;
use tokio::time::sleep;
use tracing::{info, Level};
use tracing_subscriber::{fmt::writer::MakeWriterExt, FmtSubscriber};
use webhook::client::WebhookClient;

pub mod env;
pub mod secrets;
pub mod tf;

pub struct Bot {
    pub env: Env,
    pub secrets: Secrets,
    pub discord: WebhookClient,
}

impl Bot {
    pub async fn new() -> Result<Bot> {
        let env = env::detect_env()?;
        let secrets = Secrets::new()?;
        let discord = WebhookClient::new(&secrets.dc.url);
        Ok(Bot {
            env,
            secrets,
            discord,
        })
    }

    pub async fn send_discord<F>(&self, f: F) -> Result<()>
    where
        F: Fn(&mut webhook::models::Message) -> &mut webhook::models::Message,
    {
        assert!(self
            .discord
            .send(f)
            .await
            .map_err(|e| anyhow!("failed to delivery notification message: {e:?}"))?);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    let file_appender = tracing_appender::rolling::daily("../logs", "worker.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(std::io::stdout.and(non_blocking_file))
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // construct bot
    info!("lydia-worker started");
    let bot = Arc::new(Bot::new().await?);

    // run bot
    info!("run bot");

    let jobs = match bot.env {
        Env::Exo => vec![],
        Env::TF => vec![],
        Env::Dev => std::env::var("LYDIA_DEV_JOBS")?
            .split(',')
            .map(String::from)
            .collect::<Vec<_>>(),
    };
    let mut handles = vec![];
    for job in &jobs {
        handles.push(match job.as_str() {
            "wm:lomp" => tf::lomp::start_lomp_worker()?,
            _ => bail!("unknown job: {}", job),
        });
    }

    bot.send_discord(|msg| {
        msg.content(&format!(
            "lydia-worker started
            env: {:?}
            enabled jobs({}): {:#?}",
            bot.env,
            handles.len(),
            &jobs
        ))
    })
    .await?;

    loop {
        // check workers
        let mut index = 0;
        let mut err = false;
        for handle in &handles {
            if handle.is_finished() {
                err = true;
                break;
            }
            index += 1;
        }
        if !err {
            sleep(Duration::from_secs(5)).await;
            continue;
        }
        // error exits
        let handle = handles.swap_remove(index);
        let job = &jobs[index];
        bot.send_discord(|msg| msg.content(&format!("{job} stopped unexpectedly")))
            .await?;
        handle.await?;
        bail!("job {job} finished successfully")
    }
}
