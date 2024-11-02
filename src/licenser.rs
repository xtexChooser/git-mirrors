use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        LazyLock, RwLock,
    },
};

use anyhow::{bail, Result};
use ed25519_dalek::{pkcs8::DecodePrivateKey, SigningKey};
use educe::Educe;
use log::{error, info, warn};
use tokio::sync::oneshot;
use yjyz_tools::license::{self, FeatureFlags, LatestLicenseClaims, License};

#[derive(Educe)]
#[educe(Default)]
pub struct LicensesWindow {
    licenser: LicenserWindow,
    activation_result: Option<String>,
    activation_task: Option<oneshot::Receiver<String>>,
    activation_code: String,
}

impl LicensesWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        ui.vertical(|ui| {
            ui.label("已加载的许可文件：");
            for claims in LICENSES.read().unwrap().iter() {
                let text = if claims.features.contains(FeatureFlags::SUDOER) {
                    format!("— {}（超级用户）", claims.id)
                } else if claims.features.contains(FeatureFlags::SUDOER) {
                    format!("— {}（试用）", claims.id)
                } else {
                    format!("— {}", claims.id)
                };

                let mut hover = Vec::new();
                for (flag, _) in claims.features.iter_names() {
                    hover.push(flag);
                }
                ui.label(text).on_hover_text(hover.join("\n"));
            }
        });
        if is_set(FeatureFlags::SUDOER) {
            ui.collapsing("创建授权", |ui| self.licenser.show(ui))
                .body_returned
                .unwrap_or(Ok(()))?;
        }
        Ok(())
    }

    pub fn show_no_license(&mut self, ui: &mut egui::Ui) -> Result<()> {
        ui.heading("找不到许可文件");
        ui.label("在此设备上找不到有效的许可文件。");
        ui.horizontal(|ui| {
            if ui.button("免费试用").clicked() {
                start_trail();
            }
            Ok(()) as anyhow::Result<()>
        })
        .inner?;
        ui.collapsing("使用激活码", |ui| {
            if let Some(rx) = &mut self.activation_task {
                ui.label("激活中…");
                match rx.try_recv() {
                    Err(oneshot::error::TryRecvError::Empty) => {}
                    Err(err) => return Err(err.into()),
                    Ok(result) => {
                        info!("Got activation result: {}", result);
                        self.activation_result = Some(result);
                        self.activation_task = None;
                        ui.ctx().request_repaint();
                    }
                }
            } else {
                ui.text_edit_singleline(&mut self.activation_code);
                if ui.button("激活").clicked() {
                    let (tx, rx) = oneshot::channel();
                    self.activation_task = Some(rx);

                    let key = self.activation_code.clone();
                    tokio::spawn(async {
                        match do_activation(key).await {
                            Ok(result) => tx.send(result),
                            Err(err) => {
                                error!("Failed to resolve activation code: {:?}", err);
                                tx.send(err.to_string())
                            }
                        }
                    });
                }
                if let Some(result) = &self.activation_result {
                    ui.label(result);
                }
            }
            Ok(()) as anyhow::Result<()>
        })
        .body_returned
        .unwrap_or(Ok(()))?;
        Ok(())
    }
}

#[derive(Educe)]
#[educe(Default)]
pub struct LicenserWindow {
    #[educe(Default = FeatureFlags::empty())]
    features: FeatureFlags,
}

impl LicenserWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        for (name, flag) in FeatureFlags::all().iter_names() {
            let mut state = self.features.contains(flag);
            ui.checkbox(&mut state, name);
            self.features.set(flag, state);
        }
        if ui.button("创建").clicked() {
            let mut sign_key = SigningKey::from_pkcs8_pem(&fs::read_to_string("yjyz-tools.pem")?)?;

            let claims = LatestLicenseClaims::from(self.features);
            let license = License::V1(claims.sign(&mut sign_key)?);
            fs::write(".yjyz-tools.lic", license.to_bytes()?)?;
        }
        Ok(())
    }
}

pub fn find_licenses() -> Vec<PathBuf> {
    const LICENSE_NAMES: &[&str] = &[
        ".yjyz-tools.lic",
        ".license.key",
        "yjyz-tools.lic",
        "yjyz-tools.key",
        "yjyz/yjyz-tools.lic",
        "yjyz-tools/yjyz-tools.lic",
        "license.key",
        "license",
        "license.bin",
        "maint/keys/sudo",
    ];

    let mut dirs = Vec::from([PathBuf::from("."), PathBuf::from("C:\\")]);
    for disk in sysinfo::Disks::new_with_refreshed_list().list() {
        dirs.push(disk.mount_point().to_path_buf());
    }
    let mut files = Vec::new();
    for dir in dirs {
        for name in LICENSE_NAMES {
            let path = dir.join(name);
            if path.exists() {
                files.push(path);
            }
        }
    }
    files
}

pub static LICENSES: LazyLock<RwLock<Vec<LatestLicenseClaims>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
pub static IS_SUDOER: AtomicBool = AtomicBool::new(false);
pub static HAS_TRAIL: AtomicBool = AtomicBool::new(false);

pub fn load_licenses() {
    for path in find_licenses() {
        info!("Loading license from: {}", path.display());
        match License::from_file(&path) {
            Ok(license) => {
                if let Err(err) = register_license(license) {
                    error!(
                        "Failed to register license from {}: {:?}",
                        path.display(),
                        err
                    );
                }
            }
            Err(err) => {
                warn!("Malformed license at {}: {:?}", path.display(), err)
            }
        }
    }
}

pub fn register_license(license: License) -> Result<()> {
    if license.verify() {
        match license.to_latest_claims() {
            Ok(claims) => {
                info!("Loaded license {}", claims.id);
                if claims.features.contains(FeatureFlags::SUDOER) {
                    warn!("Superuser flag from {}", claims.id);
                    IS_SUDOER.store(true, Ordering::Relaxed);
                }
                if claims.features.contains(FeatureFlags::TRAIL) {
                    info!("Trail flag from {}", claims.id);
                    HAS_TRAIL.store(true, Ordering::Relaxed);
                }
                LICENSES.write().unwrap().push(claims);
            }
            Err(err) => warn!("Failed to upgrade license: {:?}", err),
        }
    } else {
        bail!("unsigned license")
    }
    Ok(())
}

pub fn is_set(flags: FeatureFlags) -> bool {
    if IS_SUDOER.load(Ordering::Relaxed) {
        return license::v1::SUDOER_RIGHTS().contains(flags);
    }
    if HAS_TRAIL.load(Ordering::Relaxed) {
        if license::v1::TRAIL_RIGHTS().contains(flags) {
            return true;
        }
    }
    LICENSES
        .read()
        .unwrap()
        .iter()
        .any(|claims| claims.features.contains(flags))
}

pub async fn do_activation(key: String) -> Result<String> {
    Ok("已激活".to_string())
}

pub fn start_trail() {}
