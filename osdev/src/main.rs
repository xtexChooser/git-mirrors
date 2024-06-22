use anyhow::Result;
use categorize_redir::categorize_redirects;
use clap::Parser;
use uncatredir::mark_uncategorized_redirects;

pub mod categorize_redir;
pub mod consts;
pub mod uncatredir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Args {
	#[command(about = "Mark uncategorized redirects")]
	MarkUncatRedirs,
	#[command(about = "Categorize redirects")]
	CategorizeRedirects,
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv()?;
	let args = Args::parse();
	tracing_subscriber::fmt::init();

	match args {
		Args::MarkUncatRedirs => mark_uncategorized_redirects().await?,
		Args::CategorizeRedirects => categorize_redirects().await?,
	}

	Ok(())
}
