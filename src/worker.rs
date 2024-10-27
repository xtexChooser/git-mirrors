use std::{
    cmp,
    ffi::c_void,
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::Result;
use educe::Educe;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE},
};
use yjyz_tools::license::{self, LicenseFeatures};

use crate::{mythware, sec};

#[derive(Educe)]
#[educe(Default)]
pub struct WorkerState {
    pub always_on_top: Option<usize>,

    pub mythware_auto_unlock_keyboard: bool,
}

pub type WorkerStateRef = Arc<RwLock<WorkerState>>;

pub async fn run(state: WorkerStateRef) -> Result<()> {
    let mut delay = 0;
    let mut runtime_sec_counter = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(delay)).await;
        delay = 300;
        runtime_sec_counter += 1;

        let state = state.read().unwrap();

        if let Some(hwnd) = state.always_on_top {
            delay = cmp::min(delay, 100);
            unsafe {
                let hwnd = HWND(hwnd as *mut c_void);
                SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)?;
            }
        }

        if license::is_set(LicenseFeatures::MYTHWARE_WINDOWING) {
            if state.mythware_auto_unlock_keyboard && mythware::is_broadcast_on().unwrap_or(false) {
                delay = cmp::min(delay, 25);
                mythware::unlock_keyboard()?;
            }
        }

        if runtime_sec_counter >= (1000 / delay) {
            runtime_sec_counter = 0;

            if !license::is_set(LicenseFeatures::NO_SECURITY_CHECK)
                && !license::is_set(LicenseFeatures::ALLOW_INSECURE)
            {
                if let Err(err) = sec::check_environment() {
                    panic!("unavailable: 5865bf4b-80ff-48ac-b5b0-280caaf267af: {}", err)
                }
            }
        }
    }
}
