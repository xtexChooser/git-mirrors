use std::{path::PathBuf, sync::LazyLock};

use anyhow::{bail, Result};
use egui::RichText;
use log::info;
use windows_registry::LOCAL_MACHINE;

fn open_eclass_standard() -> Result<windows_registry::Key> {
    Ok(LOCAL_MACHINE
        .open(r"SOFTWARE\TopDomain\e-Learning Class Standard\1.00")
        .or_else(|_| {
            LOCAL_MACHINE.open(r"SOFTWARE\WOW6432Node\TopDomain\e-Learning Class Standard\1.00")
        })?)
}

fn open_eclass_student() -> Result<windows_registry::Key> {
    Ok(LOCAL_MACHINE
        .open(r"SOFTWARE\TopDomain\e-Learning Class\Student")
        .or_else(|_| {
            LOCAL_MACHINE.open(r"SOFTWARE\WOW6432Node\TopDomain\e-Learning Class\Student")
        })?)
}

pub fn read_installation_dir() -> Result<PathBuf> {
    Ok(open_eclass_standard()?
        .get_string("TargetDirectory")?
        .into())
}

pub static INSTALLATION_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    read_installation_dir()
        .inspect_err(|err| info!("Mythware e-Learning Class not found: {err}"))
        .ok()
});

pub fn read_password() -> Result<String> {
    // https://github.com/MuliMuri/Mythware/blob/master/Test/Program.cs
    let knock = open_eclass_student()?.get_bytes("Knock1")?;
    if knock.len() % 4 != 0 {
        bail!("length of Knock1 is not multiplies of 4");
    }
    if knock.len() < 4 {
        bail!("length of Knock1 is below 4");
    }
    let knock = knock
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .map(|val| val ^ 0x50434c45 ^ 0x454c4350)
        .flat_map(|val| u32::to_be_bytes(val))
        .collect::<Vec<u8>>();
    if knock[0] != *knock.last().unwrap() {
        bail!("the first byte of Knock1 is not equal to the last byte")
    }
    let knock = &knock.as_slice()[knock[0] as usize..knock.len() - 1];
    let mut len = 0;
    for chunk in knock.chunks_exact(2) {
        if chunk == [0, 0] {
            break;
        } else {
            len += 1;
        }
    }
    let knock = &knock[0..(len * 2)];
    Ok(String::from_utf8(knock.to_vec())?)
}

pub fn read_password_legacy() -> Result<String> {
    let passwd = open_eclass_standard()?.get_string("UninstallPasswd")?;
    if passwd.starts_with("Passwd") {
        Ok(passwd[6..].to_owned())
    } else {
        Ok(passwd)
    }
}

pub static PASSWORD: LazyLock<Option<String>> = LazyLock::new(|| {
    read_password()
        .inspect_err(|err| info!("Failed to read new kind of mythware password: {err}"))
        .or_else(|_| read_password_legacy())
        .inspect_err(|err| info!("Failed to read legacy kind of mythware password: {err}"))
        .ok()
});

pub fn show_window(ui: &mut egui::Ui) {
    if let Some(path) = INSTALLATION_PATH.as_ref() {
        ui.horizontal_wrapped(|ui| {
            ui.label("安装位置：");
            ui.label(RichText::new(path.to_str().unwrap_or_default()).italics());
        });
    }
    ui.horizontal_wrapped(|ui| {
        ui.label("密码：");
        if let Some(password) = PASSWORD.as_ref() {
            if password.is_empty() {
                ui.label(RichText::new("（空）").italics());
            } else {
                ui.label(RichText::new(password).italics());
            }
        } else {
            ui.label(RichText::new("（读取密码失败）").italics());
        }
    });
    ui.label("超级密码：mythware_super_password");
}
