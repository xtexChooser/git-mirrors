use std::{fs, path::PathBuf, sync::LazyLock};

use anyhow::Result;
use educe::Educe;
use egui::{RichText, WidgetText};
use log::info;
use yjyz_tools::license::{self, LicenseFeatures};

#[derive(Educe)]
#[educe(Default)]
pub struct PowerShadowConfig {
    pub normal_mode_md5: Option<String>,
    pub sys_mode_md5: Option<String>,
}

pub static PSCONFIG: LazyLock<Option<PowerShadowConfig>> = LazyLock::new(|| {
    read_psconfig()
        .inspect_err(|err| info!("cannot read PsConfig.set: {err}"))
        .ok()
        .and_then(|s| s)
});

const PATTERN_NORMAL_PASSWD: [u8; 13] = [
    0x00, 0x6E, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x6D, 0x00, 0x61, 0x00, 0x6C, 0x00,
];

const PATTERN_SYSMODE_PASSWD: [u8; 15] = [
    0x00, 0x73, 0x00, 0x79, 0x00, 0x73, 0x00, 0x6D, 0x00, 0x6F, 0x00, 0x64, 0x00, 0x65, 0x00,
];

const EMPTY_PASSWORD_MD5: &str = "93b885adfe0da089cdf634904fd59f71";
// md5("abc123@@")，电子阅览室使用
const ABC123ATAT_MD5: &str = "6bbe5eaa9b787ca6ae1730ef700eebe7";

pub fn read_psconfig() -> Result<Option<PowerShadowConfig>> {
    if license::is_set(LicenseFeatures::POWERSHADOW_PASSWORD) {
        return Ok(None);
    }
    let path = PathBuf::from(r"C:\WINDOWS\system32\PsConfig.set");
    if !path.exists() {
        Ok(None)
    } else {
        let psconfig = fs::read(path)?;
        let config = PowerShadowConfig {
            normal_mode_md5: psconfig
                .windows(PATTERN_NORMAL_PASSWD.len() + 16)
                .find(|s| s[0..PATTERN_NORMAL_PASSWD.len()] == PATTERN_NORMAL_PASSWD)
                .map(|s| &s[PATTERN_NORMAL_PASSWD.len()..])
                .map(hex::encode),
            sys_mode_md5: psconfig
                .windows(PATTERN_SYSMODE_PASSWD.len() + 16)
                .find(|s| s[0..PATTERN_SYSMODE_PASSWD.len()] == PATTERN_SYSMODE_PASSWD)
                .map(|s| &s[PATTERN_SYSMODE_PASSWD.len()..])
                .map(hex::encode),
        };
        Ok(Some(config))
    }
}

#[derive(Educe)]
#[educe(Default)]
pub struct PowerShadowWindow {}

impl PowerShadowWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        if let Some(config) = PSCONFIG.as_ref() {
            if license::is_set(LicenseFeatures::POWERSHADOW_PASSWORD) {
                #[inline]
                fn show_password(
                    ui: &mut egui::Ui,
                    label: impl Into<WidgetText>,
                    passwd: Option<&String>,
                ) {
                    ui.horizontal_wrapped(|ui| {
                        let label = ui.label(label.into().strong());
                        if let Some(passwd) = passwd {
                            ui.label(passwd).labelled_by(label.id);
                            if passwd == EMPTY_PASSWORD_MD5 {
                                ui.label(RichText::new("（密码为空）").strong());
                            } else if passwd == ABC123ATAT_MD5 {
                                ui.label(RichText::new("（abc123@@）").strong());
                            }
                        } else {
                            ui.label(RichText::new("（未找到）").strong())
                                .labelled_by(label.id);
                        }
                    });
                }
                show_password(
                    ui,
                    RichText::new("普通模式密码 MD5："),
                    config.normal_mode_md5.as_ref(),
                );
                show_password(
                    ui,
                    RichText::new("单一影子系统模式密码 MD5："),
                    config.sys_mode_md5.as_ref(),
                );
            } else {
                ui.label("影子系统密码：（不支持）");
            }
        } else {
            ui.label("找不到影子系统配置文件");
        }
        Ok(())
    }
}
