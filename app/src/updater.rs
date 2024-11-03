use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
};

use anyhow::{bail, Result};
use educe::Educe;
use egui::OpenUrl;
use log::info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::process::Command;
use yjyz_tools_license::FeatureFlags;

use crate::{licenser, ASYNC_ERROR};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionJson {
    pub version: String,
    #[serde(default)]
    pub message: Option<String>,
    pub download: String,
    pub sha256sum: String,
}

static CHECK_COMPLETED: AtomicBool = AtomicBool::new(false);
static UPDATE: RwLock<Option<VersionJson>> = RwLock::new(None);

pub async fn check() -> Result<()> {
    let mut path = std::path::absolute(std::env::current_exe()?)?;
    path.add_extension("upd.bat");
    if path.exists() {
        info!("Removing update bat ...");
        fs::remove_file(path)?;
        info!("Removed update bat, skipping update checking ...");
        CHECK_COMPLETED.store(true, Ordering::Relaxed);
        return Ok(());
    }

    info!("Checking for updates ...");
    let resp = reqwest::get("https://xtex.envs.net/yjyz-tools/version_v1.json")
        .await?
        .error_for_status()?
        .json::<VersionJson>()
        .await?;
    if resp.version != env!("CARGO_PKG_VERSION") {
        info!("Found update: {}", resp.version);
        let _ = UPDATE.write().unwrap().insert(resp);
    }
    CHECK_COMPLETED.store(true, Ordering::Relaxed);
    Ok(())
}

pub async fn do_update() -> Result<()> {
    let update = UPDATE.read().unwrap().to_owned();
    if let Some(update) = update {
        info!("Doing update ...");
        let resp = Box::new(
            reqwest::get(&update.download)
                .await?
                .error_for_status()?
                .bytes()
                .await?,
        );
        info!("Update file downloaded ...");

        let mut hasher = Sha256::new();
        hasher.update(&*resp);
        let digest = hasher.finalize();
        let expected = hex::decode(&update.sha256sum)?;
        if &digest[0..] != expected.as_slice() {
            bail!(
                "Broken checksum: {}, expected {}",
                hex::encode(digest),
                update.sha256sum
            )
        }

        let path = std::path::absolute(std::env::current_exe()?)?;
        fs::write(path.with_added_extension("updtmp"), *resp)?;

        let bat_path = path.with_added_extension("upd.bat");
        let exe_path = path.to_str().unwrap().to_string();
        let proc_name = std::env::current_exe()?
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        fs::write(
            &bat_path,
            format!(
                "TASKKILL /F /IM {proc_name}\nPING -1 3 127.0.0.1 > NUL\nMOVE {exe_path}.updtmp {exe_path}\nSTART {exe_path}"
            ),
        )?;
        _ = Command::new(bat_path)
            .current_dir(std::env::current_dir()?)
            .kill_on_drop(false)
            .spawn()?;
    }
    Ok(())
}

#[derive(Educe)]
#[educe(Default)]
pub struct Updater {
    updating: bool,
    dismissed: bool,
}

impl Updater {
    pub fn should_show(&self) -> bool {
        if licenser::is_set(FeatureFlags::NO_UPDATE) || self.dismissed {
            return false;
        }
        if UPDATE.read().unwrap().is_some() {
            return true;
        }
        licenser::is_set(FeatureFlags::MUST_UPDATE)
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        if self.updating {
            ui.heading("正在更新，请稍候……");
        } else if CHECK_COMPLETED.load(Ordering::Relaxed) {
            if let Some(update) = &*UPDATE.read().unwrap() {
                ui.vertical(|ui| {
                    ui.heading(format!("新版本可用：{}", update.version));
                    if let Some(message) = &update.message {
                        if !message.is_empty() {
                            ui.label(message);
                        }
                    }
                    if ui.button("更新").clicked() {
                        tokio::spawn(async {
                            if let Err(err) = do_update().await {
                                let _ = ASYNC_ERROR.write().unwrap().insert(err);
                            }
                        });
                        self.updating = true;
                    }
                    if ui.button("使用浏览器下载").clicked() {
                        ui.ctx().open_url(OpenUrl::new_tab(&update.download));
                    }

                    if !licenser::is_set(FeatureFlags::MUST_UPDATE)
                        && ui.button("不更新").clicked()
                    {
                        self.dismissed = true;
                    }
                });
            } else {
                self.dismissed = true;
                ui.ctx().request_repaint();
            }
        } else {
            ui.heading("正在检查更新，请稍候……");
        }
        Ok(())
    }
}
