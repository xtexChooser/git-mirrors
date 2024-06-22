use anyhow::Result;
use clap::Parser;
use mwbot::Bot;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let bot = Bot::from_default_config().await?;

	Ok(())
}
