use std::{
    ffi::c_void,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE},
};

#[derive(Default)]
pub struct WorkerState {
    pub always_on_top: Option<usize>,
}

pub async fn run(state: Arc<RwLock<WorkerState>>) -> Result<()> {
    loop {
        let mut delay = 40;

        let state = state.read().unwrap();

        if let Some(hwnd) = state.always_on_top {
            unsafe {
                let hwnd = HWND(hwnd as *mut c_void);
                SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE)?;
            }
        }
    }
    Ok(())
}
