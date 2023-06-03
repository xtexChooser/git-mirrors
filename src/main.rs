use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use notify::{Event, EventKind, RecommendedWatcher, Watcher};
use podman_api::Podman;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::info;

mod image;
mod macros;
mod rc;

#[derive(Debug, Parser, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path for the podman remote socket
    #[clap(short, long, default_value = "unix:///run/podman/podman.sock")]
    podman: String,

    /// Directory with configuration files
    #[clap(short, long, default_value = "/etc/podrc")]
    base: PathBuf,

    /// Keep exists resources
    #[clap(short, long, default_value_t = false)]
    keep: bool,

    #[clap(subcommand)]
    command: Subcommand,
}

#[derive(Debug, clap::Subcommand, Serialize, Deserialize)]
enum Subcommand {
    /// Apply the configuration at once
    Apply,
    /// Apply and watch for future changes
    Daemon,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("init podman socket");
    let podman = Podman::new(&args.podman)?;
    info!(
        api_version = podman.ping().await?.api_version,
        "podman socket connected"
    );

    if !args.base.exists() {
        bail!("base directory {} does not exist", args.base.display())
    }

    match args.command {
        Subcommand::Apply => rc::apply(&args.base, &podman, args.keep).await?,
        Subcommand::Daemon => {
            let (tx, mut rx) = mpsc::channel(1);
            tx.send(()).await?;
            let mut watcher = RecommendedWatcher::new(
                move |res: Result<_, _>| {
                    let res: Event = res.unwrap();
                    if matches!(
                        res.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    ) {
                        futures::executor::block_on(async {
                            tx.send(()).await.unwrap();
                        });
                    }
                },
                notify::Config::default(),
            )?;
            watcher.watch(&args.base, notify::RecursiveMode::Recursive)?;

            while let Some(_) = rx.recv().await {
                info!("some files changed, reloading configuration");
                rc::apply(&args.base, &podman, args.keep).await?;
            }
        }
    }

    Ok(())
}
