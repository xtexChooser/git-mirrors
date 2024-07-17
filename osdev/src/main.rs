use anyhow::Result;
use categorize_redir::categorize_redirects;
use clap::Parser;

pub mod categorize_redir;
pub mod consts;

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Args {
	#[command(about = "Categorize redirects")]
	CategorizeRedirects,
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv()?;
	let args = Args::parse();
	tracing_subscriber::fmt::init();

	match args {
		Args::CategorizeRedirects => categorize_redirects().await?,
	}

	Ok(())
}
