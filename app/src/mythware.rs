use std::{
    ffi::{c_void, OsString},
    os::windows::ffi::OsStringExt,
    path::PathBuf,
    process::Command,
    str::FromStr,
    sync::{LazyLock, RwLock},
};

use anyhow::{bail, Result};
use cached::proc_macro::once;
use educe::Educe;
use egui::{RichText, WidgetText};
use log::info;
use sysinfo::ProcessesToUpdate;
use windows::{
    core::{HRESULT, PCWSTR},
    Win32::{
        Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            ClipCursor, EnumWindows, GetClassNameA, GetWindowLongW, GetWindowTextLengthW,
            GetWindowTextW, PostMessageA, SetWindowsHookExW, UnhookWindowsHookEx, BM_CLICK,
            GWL_STYLE, HHOOK, WH_KEYBOARD_LL, WH_MOUSE_LL, WINDOW_STYLE, WM_COMMAND, WS_SYSMENU,
        },
    },
};
use windows_registry::LOCAL_MACHINE;
use yjyz_tools_license::FeatureFlags;

use crate::{licenser, utils, worker::WorkerStateRef};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub enum SetupType {
    Teacher,
    Student,
}

impl FromStr for SetupType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Teacher" => Ok(Self::Teacher),
            "Student" => Ok(Self::Student),
            _ => bail!("unparsable SetupType {}", s),
        }
    }
}

impl TryFrom<String> for SetupType {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

pub static SETUP_TYPE: LazyLock<Option<SetupType>> = LazyLock::new(|| {
    read_setup_type()
        .inspect_err(|err| info!("cannot read mythware setup type: {err}"))
        .ok()
});

pub fn read_setup_type() -> Result<SetupType> {
    open_eclass_standard()?.get_string("SetupType")?.try_into()
}

pub fn read_password() -> Result<String> {
    // https://github.com/MuliMuri/Mythware/blob/master/Test/Program.cs
    let knock = open_eclass_student()?.get_value("Knock1")?;
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
        .flat_map(u32::to_be_bytes)
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
    if let Some(passwd) = passwd.strip_prefix("Passwd") {
        Ok(passwd.to_owned())
    } else {
        Ok(passwd)
    }
}

pub static PASSWORD: LazyLock<RwLock<Option<String>>> = LazyLock::new(|| {
    read_password()
        .inspect_err(|err| info!("Failed to read new kind of mythware password: {err}"))
        .or_else(|_| read_password_legacy())
        .inspect_err(|err| info!("Failed to read legacy kind of mythware password: {err}"))
        .ok()
        .map(|s| {
            if licenser::is_set(FeatureFlags::MYTHWARE_PASSWORD) {
                s
            } else {
                "（不支持）".to_string()
            }
        })
        .into()
});

pub fn set_password(password: &str) -> Result<()> {
    let mut knock = Vec::new();
    knock.push(1);
    for char in password.as_bytes() {
        knock.push(*char);
        knock.push(0);
    }
    knock.append(&mut vec![0, 0]);
    while knock.len() % 4 != 3 {
        knock.push(0);
    }
    knock.push(1);
    let knock = knock
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
        .map(|val| val ^ 0x454c4350 ^ 0x50434c45)
        .flat_map(u32::to_be_bytes)
        .collect::<Vec<u8>>();
    open_eclass_student()?.create("")?.set_bytes(
        "Knock1",
        windows_registry::Type::Bytes,
        &knock,
    )?;
    *PASSWORD.write().unwrap() = Some(password.to_owned());
    Ok(())
}

pub fn find_broadcast_window() -> Result<Option<HWND>> {
    static mut WINDOW: Option<HWND> = None;
    unsafe {
        WINDOW = None;
        unsafe extern "system" fn enum_callback(hwnd: HWND, _: LPARAM) -> BOOL {
            let mut name = [0u8; 20];
            assert!(GetClassNameA(hwnd, &mut name) >= 0);
            // check "Afx:" prefix
            if name[0..4] == [0x41, 0x66, 0x78, 0x3a] {
                let mut title = vec![0; GetWindowTextLengthW(hwnd) as usize + 1];
                assert!(GetWindowTextW(hwnd, &mut title) >= 0);
                let title = OsString::from_wide(&title).to_string_lossy().to_string();
                if title.starts_with("屏幕广播") || title.ends_with(" 正在共享屏幕") {
                    WINDOW = Some(hwnd);
                    return false.into();
                }
            }
            true.into()
        }
        if let Err(err) = EnumWindows(Some(enum_callback), LPARAM::default())
            && err.code() != HRESULT(0)
        {
            return Err(err.into());
        }
        if let Some(hwnd) = WINDOW {
            return Ok(Some(hwnd));
        }
    }
    Ok(None)
}

#[once(time = 1, result = true)]
pub fn is_broadcast_on() -> Result<bool> {
    Ok(find_broadcast_window()?.is_some())
}

#[once(time = 1, result = true)]
pub fn is_broadcast_fullscreen() -> Result<bool> {
    if let Some(hwnd) = find_broadcast_window()? {
        unsafe {
            if (WINDOW_STYLE(GetWindowLongW(hwnd, GWL_STYLE) as u32) & WS_SYSMENU).0 == 0 {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn toggle_broadcast_window() -> Result<()> {
    if !licenser::is_set(FeatureFlags::MYTHWARE_WINDOWING) {
        return Ok(());
    }
    if let Some(hwnd) = find_broadcast_window()? {
        unsafe {
            PostMessageA(
                hwnd,
                WM_COMMAND,
                WPARAM(((BM_CLICK << 16) | 1004) as usize),
                LPARAM(0),
            )?;
        }
        *IS_BROADCAST_FULLSCREEN.write().unwrap() = None;
    }
    Ok(())
}

struct UnlockingHook(usize);

impl Drop for UnlockingHook {
    fn drop(&mut self) {
        unsafe { UnhookWindowsHookEx(HHOOK(self.0 as *mut c_void)).unwrap() };
    }
}

impl From<HHOOK> for UnlockingHook {
    fn from(value: HHOOK) -> Self {
        Self(value.0 as usize)
    }
}

static UNLOCKING_HOOKS: RwLock<Vec<UnlockingHook>> = RwLock::new(Vec::new());

pub fn unlock_keyboard() -> Result<()> {
    if !licenser::is_set(FeatureFlags::MYTHWARE_WINDOWING) {
        return Ok(());
    }
    unsafe {
        unsafe extern "system" fn hook_proc(_: i32, _: WPARAM, _: LPARAM) -> LRESULT {
            LRESULT(0)
        }

        let module = GetModuleHandleW(PCWSTR::null())?;
        *UNLOCKING_HOOKS.write().unwrap() = vec![
            SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), module, 0)?.into(),
            SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), module, 0)?.into(),
        ];
        ClipCursor(None)?;
    }
    Ok(())
}

#[once(time = 1)]
pub fn find_studentmain_pid() -> Option<u32> {
    let mut sys = sysinfo::System::new();
    sys.refresh_processes(ProcessesToUpdate::All);
    for (pid, process) in sys.processes() {
        if process.name().to_ascii_lowercase() == "studentmain.exe" {
            return Some(pid.as_u32());
        }
    }
    None
}

#[derive(Educe)]
#[educe(Default)]
pub struct MythwareWindow {
    set_password_buf: Option<String>,
    #[educe(Default = true)]
    pub auto_windowing_broadcast: bool,
    #[educe(Default = true)]
    pub auto_unlock_keyboard: bool,
    #[educe(Default = false)]
    pub stumain_suspended: bool,
}

impl MythwareWindow {
    pub fn show(&mut self, ui: &mut egui::Ui, worker: &WorkerStateRef) -> Result<()> {
        if licenser::is_set(FeatureFlags::MYTHWARE_STOPPING) {
            ui.horizontal_wrapped(|ui| {
                if let Some(pid) = find_studentmain_pid() {
                    if ui.button("关闭极域").clicked() {
                        utils::force_kill_process(pid)?;
                        *FIND_STUDENTMAIN_PID.write().unwrap() = None;
                    }
                } else if let Some(path) = INSTALLATION_PATH.as_ref() {
                    if ui.button("启动极域").clicked() {
                        Command::new(path.join("StudentMain.exe")).spawn()?;
                        ui.ctx().request_repaint();
                        *FIND_STUDENTMAIN_PID.write().unwrap() = None;
                    }
                }
                Ok::<(), anyhow::Error>(())
            })
            .inner?;
        }
        if let Some(path) = INSTALLATION_PATH.as_ref() {
            ui.horizontal_wrapped(|ui| {
                let label = ui.label(RichText::new("安装位置：").strong());
                ui.label(RichText::new(path.to_str().unwrap_or_default()).italics())
                    .labelled_by(label.id);
            });
        }
        if let Some(ty) = SETUP_TYPE.as_ref() {
            ui.horizontal_wrapped(|ui| {
                let label = ui.label(RichText::new("安装类型：").strong());
                let text = match ty {
                    SetupType::Teacher => "教师端",
                    SetupType::Student => "学生端",
                };
                ui.label(RichText::new(text).italics())
                    .labelled_by(label.id);
            });
        }
        self.show_password(ui, RichText::new("密码：").strong())?;

        if licenser::is_set(FeatureFlags::MYTHWARE_PASSWORD) {
            ui.label("超级密码：mythware_super_password");
        }

        ui.horizontal_wrapped(|ui| {
            let label = ui.label(RichText::new("屏幕广播：").strong());
            if is_broadcast_on()? {
                if licenser::is_set(FeatureFlags::MYTHWARE_WINDOWING) {
                    if ui
                        .button(if is_broadcast_fullscreen()? {
                            "广播窗口化"
                        } else {
                            "广播全屏化"
                        })
                        .clicked()
                    {
                        if !is_broadcast_fullscreen()? {
                            // toggle into fullscreen
                            self.auto_windowing_broadcast = false;
                        }
                        toggle_broadcast_window()?;
                    }
                } else {
                    ui.label("当前正在广播").labelled_by(label.id);
                }
            } else {
                ui.label("当前无广播").labelled_by(label.id);
            }
            ui.checkbox(&mut self.auto_windowing_broadcast, "自动窗口化");
            if ui
                .checkbox(&mut self.auto_unlock_keyboard, "自动解除键盘锁")
                .changed()
            {
                worker.write().unwrap().mythware_auto_unlock_keyboard = self.auto_unlock_keyboard;
            }
            Ok::<(), anyhow::Error>(())
        })
        .inner?;

        if licenser::is_set(FeatureFlags::MYTHWARE_SUSPENDING) {
            if let Some(pid) = find_studentmain_pid() {
                ui.horizontal_wrapped(|ui| {
                    let label = ui.label(RichText::new("挂起：").strong());
                    if self.stumain_suspended {
                        if ui.button("取消挂起").labelled_by(label.id).clicked() {
                            utils::resume_process(pid)?;
                            self.stumain_suspended = false;
                        }
                    } else if ui.button("挂起").labelled_by(label.id).clicked() {
                        utils::suspend_process(pid)?;
                        self.stumain_suspended = true;
                    }
                    Ok::<(), anyhow::Error>(())
                })
                .inner?;
            }
        }
        Ok(())
    }

    pub fn show_password(&mut self, ui: &mut egui::Ui, label: impl Into<WidgetText>) -> Result<()> {
        ui.horizontal_wrapped(|ui| {
            let label = ui.label(label);
            match &mut self.set_password_buf {
                None => {
                    if let Some(password) = PASSWORD.read().unwrap().as_ref() {
                        if password.is_empty() {
                            ui.label(RichText::new("（空）").italics())
                                .labelled_by(label.id);
                        } else {
                            ui.label(RichText::new(password).italics())
                                .labelled_by(label.id);
                        }
                    } else {
                        ui.label(RichText::new("（读取密码失败）").italics())
                            .labelled_by(label.id);
                    }
                    if ui.button("修改").clicked() {
                        self.set_password_buf =
                            Some(PASSWORD.read().unwrap().clone().unwrap_or_default());
                    }
                }
                Some(buf) => {
                    ui.text_edit_singleline(buf).labelled_by(label.id);
                    if ui.button("保存").clicked() {
                        set_password(buf.as_str())?;
                        self.set_password_buf = None;
                    }
                    if ui.button("取消").clicked() {
                        self.set_password_buf = None;
                    }
                }
            };
            Ok::<(), anyhow::Error>(())
        })
        .inner?;
        Ok(())
    }
}
