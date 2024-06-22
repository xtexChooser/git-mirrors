use anyhow::Result;
use clap::Parser;
use uncatredir::mark_uncategorized_redirects;

pub mod consts;
pub mod uncatredir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Args {
	#[command(about = "Mark uncategorized redirects")]
	MarkUncatRedirs,
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv()?;
	let args = Args::parse();
	tracing_subscriber::fmt::init();

	match args {
		Args::MarkUncatRedirs => mark_uncategorized_redirects().await?,
	}

	Ok(())
}
