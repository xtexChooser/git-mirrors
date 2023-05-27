use anyhow::Result;
use tokio::task::JoinHandle;

pub fn start_lomp_worker() -> Result<JoinHandle<()>> {
    Ok(tokio::spawn(async {
        //
    }))
}
