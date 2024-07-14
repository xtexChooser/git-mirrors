use anyhow::Result;
use educe::Educe;
use egui::{RichText, WidgetText};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[derive(Educe)]
#[educe(Default)]
pub struct WindowsAdjWindow {}

impl WindowsAdjWindow {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        #[inline]
        fn show_button(
            ui: &mut egui::Ui,
            text: impl Into<WidgetText>,
            callback: impl Fn() -> Result<()>,
        ) {
            if ui.button(text).clicked() {
                callback().unwrap();
            }
        }

        egui::Grid::new("windows_adj").show(ui, |ui| {
            show_button(ui, "启用全部工具", enable_all);
            ui.end_row();

            ui.label(RichText::new("命令提示符").strong());
            show_button(ui, "启用", enable_cmd);
            show_button(ui, "禁用", disable_cmd);
            ui.end_row();

            ui.label(RichText::new("任务管理器").strong());
            show_button(ui, "启用", enable_taskmgr);
            show_button(ui, "禁用", disable_taskmgr);
            ui.end_row();

            ui.label(RichText::new("注册表编辑器").strong());
            show_button(ui, "启用", enable_regedit);
            show_button(ui, "禁用", disable_regedit);
            ui.end_row();

            ui.label(RichText::new("Win+R 运行").strong());
            show_button(ui, "启用", enable_run);
            show_button(ui, "禁用", disable_run);
            ui.end_row();

            ui.label(RichText::new("移除 IFEO 调试器").strong());
            show_button(ui, "全部", remove_all_debuggers);
            show_button(ui, "ntsd.exe", || remove_debugger("ntsd.exe"));
            show_button(ui, "taskkill.exe", || remove_debugger("taskkill.exe"));
            ui.end_row();

            ui.label(RichText::new("注销").strong());
            show_button(ui, "启用", enable_logout);
            show_button(ui, "禁用", disable_logout);
            ui.end_row();

            ui.label(RichText::new("chrome://dino").strong());
            show_button(ui, "启用", enable_chrome_dino);
            show_button(ui, "禁用", disable_chrome_dino);
            ui.end_row();

            ui.label(RichText::new("edge://surf").strong());
            show_button(ui, "启用", enable_edge_surf);
            show_button(ui, "禁用", disable_edge_surf);
            ui.end_row();
        });
    }
}

pub fn enable_all() -> Result<()> {
    enable_cmd()?;
    enable_taskmgr()?;
    enable_regedit()?;
    enable_run()?;
    let _ = remove_all_debuggers();
    enable_logout()?;
    enable_chrome_dino()?;
    enable_edge_surf()?;
    Ok(())
}

pub fn enable_cmd() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Policies\Microsoft\Windows\System")?
        .set_u32("DisableCMD", 0)?)
}

pub fn disable_cmd() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Policies\Microsoft\Windows\System")?
        .set_u32("DisableCMD", 1)?)
}

pub fn enable_taskmgr() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableTaskMgr", 0)?)
}

pub fn disable_taskmgr() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableTaskMgr", 1)?)
}

pub fn enable_regedit() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableRegistryTools", 0)?)
}

pub fn disable_regedit() -> Result<()> {
    Ok(CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableRegistryTools", 1)?)
}

pub fn enable_run() -> Result<()> {
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("NoRun", 0)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("RestrictRun", 0)?;
    Ok(())
}

pub fn disable_run() -> Result<()> {
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("NoRun", 1)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("RestrictRun", 1)?;
    Ok(())
}

pub fn remove_debugger(name: &str) -> Result<()> {
    let key = LOCAL_MACHINE.create(format!(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{}",
        name
    ))?;
    if key.get_value("debugger").is_err() {
        return Ok(());
    }
    Ok(key.remove_value("debugger")?)
}

pub fn remove_all_debuggers() -> Result<()> {
    for name in LOCAL_MACHINE
        .create(format!(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options",
        ))?
        .keys()?
    {
        remove_debugger(&name)?;
    }
    Ok(())
}

pub fn enable_logout() -> Result<()> {
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer")?
        .set_u32("NoLogOff", 0)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer")?
        .set_u32("StartMenuLogOff", 0)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableLockWorkstation", 0)?;
    Ok(())
}

pub fn disable_logout() -> Result<()> {
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer")?
        .set_u32("NoLogOff", 1)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer")?
        .set_u32("StartMenuLogOff", 1)?;
    CURRENT_USER
        .create(r"Software\Microsoft\Windows\CurrentVersion\Policies\System")?
        .set_u32("DisableLockWorkstation", 1)?;
    Ok(())
}

pub fn enable_chrome_dino() -> Result<()> {
    Ok(LOCAL_MACHINE
        .create(r"SOFTWARE\Policies\Google\Chrome")?
        .set_u32("AllowDinosaurEasterEgg", 1)?)
}

pub fn disable_chrome_dino() -> Result<()> {
    Ok(LOCAL_MACHINE
        .create(r"SOFTWARE\Policies\Google\Chrome")?
        .set_u32("AllowDinosaurEasterEgg", 0)?)
}

pub fn enable_edge_surf() -> Result<()> {
    Ok(LOCAL_MACHINE
        .create(r"SOFTWARE\Policies\Microsoft\Edge")?
        .set_u32("AllowSurfGame", 1)?)
}

pub fn disable_edge_surf() -> Result<()> {
    Ok(LOCAL_MACHINE
        .create(r"SOFTWARE\Policies\Microsoft\Edge")?
        .set_u32("AllowSurfGame", 0)?)
}
