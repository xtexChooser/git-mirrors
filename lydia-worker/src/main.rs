use std::{sync::Arc, time::Duration};

use anyhow::{bail, Result};
use env::Env;
use parking_lot::RwLock;
use secrets::Secrets;
use tokio::time::sleep;
use tracing::{info, Level};
use tracing_subscriber::{fmt::writer::MakeWriterExt, FmtSubscriber};
use webhook::client::WebhookClient;

use crate::discord::send_discord;

pub mod discord;
pub mod env;
pub mod report;
pub mod secrets;
pub mod tf;
pub mod util;

pub const USER_AGENT: &str = concat!(
    "lydia-bot/",
    env!("CARGO_PKG_VERSION"),
    " (https://codeberg.org/Lydia)"
);

pub struct Bot {
    pub env: Env,
    pub secrets: Secrets,
    pub http: reqwest::Client,
    pub discord: WebhookClient,
    pub zhwp: Option<Arc<mwbot::Bot>>,
}

impl Bot {
    pub async fn new() -> Result<Bot> {
        let env = env::detect_env()?;
        let secrets = Secrets::new()?;

        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        let discord = WebhookClient::new(&secrets.dc.url);
        let wmf_bot = if let Some(s) = secrets.wmf.clone() {
            Some(Arc::new(
                mwbot::Bot::builder(
                    "https://zh.wikipedia.org/w/api.php".to_string(),
                    "https://zh.wikipedia.org/api/rest_v1".to_string(),
                )
                .set_botpassword(s.user, s.passwd)
                .set_user_agent(USER_AGENT.to_string())
                .build()
                .await?,
            ))
        } else {
            None
        };
        Ok(Bot {
            env,
            secrets,
            http,
            discord,
            zhwp: wmf_bot,
        })
    }

    pub fn is_dev(&self) -> bool {
        self.env.is_dev()
    }
}

#[macro_export]
macro_rules! if_dev {
    ($bot:ident, $prod:expr, $dev:expr) => {
        if $bot.is_dev() {
            $dev
        } else {
            $prod
        }
    };
}

pub static BOT: RwLock<Option<Arc<Bot>>> = RwLock::new(None);

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    let file_appender = tracing_appender::rolling::daily("../logs", "worker.log");
    let (non_blocking_file, _) = tracing_appender::non_blocking(file_appender);
    let err_file_appender = tracing_appender::rolling::daily("../logs", "worker-error.log");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(
            std::io::stdout
                .and(err_file_appender.with_max_level(Level::WARN))
                .and(non_blocking_file),
        )
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    // construct bot
    info!(user_agent = USER_AGENT, "lydia-worker started");
    let bot = Arc::new(Bot::new().await?);
    *BOT.write() = Some(bot.clone());

    // run bot
    info!("run bot");

    let jobs = match bot.env {
        Env::Exo => vec![],
        Env::TF => vec!["wm:lomp".to_string()],
        Env::Dev => std::env::var("LYDIA_DEV_JOBS")?
            .split(',')
            .map(String::from)
            .collect::<Vec<_>>(),
    };
    let mut handles = vec![];
    for job in &jobs {
        let bot = bot.clone();
        handles.push(match job.as_str() {
            "wm:lomp" => tf::lomp::start_lomp_worker(bot)?,
            _ => bail!("unknown job: {}", job),
        });
    }

    send_discord(&bot, |msg| {
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
            sleep(Duration::from_secs(if_dev!(bot, 30, 5))).await;
            continue;
        }
        // error exits
        let handle = handles.swap_remove(index);
        let job = &jobs[index];
        handle.await?;
        bail!("job {job} finished without error")
    }
}
