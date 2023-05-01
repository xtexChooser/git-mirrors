use anyhow::Result;
use build_clean::{db::init_lua, search};
use cursive::views::TextView;

#[tokio::main]
async fn main() -> Result<()> {
    init_lua().await?;

    /*let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));

    siv.run();*/

    search::search("/mnt/src/".into(), 8, 6).await?;

    Ok(())
}
