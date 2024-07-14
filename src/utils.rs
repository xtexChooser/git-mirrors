use std::ffi::c_void;

use anyhow::Result;
use egui::Id;
use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging::{
        GetWindowRect, SetWindowDisplayAffinity, SetWindowPos, HWND_TOPMOST, SWP_NOSIZE,
        WDA_EXCLUDEFROMCAPTURE, WDA_NONE,
    },
};

use crate::DATA_WINDOW_HWND;

pub fn prevent_screenshot(ctx: &egui::Context, prevent: bool) -> Result<()> {
    unsafe {
        let hwnd = HWND(
            ctx.data(|data| data.get_temp::<usize>(Id::new(DATA_WINDOW_HWND)).unwrap())
                as *mut c_void,
        );
        SetWindowDisplayAffinity(
            hwnd,
            if prevent {
                WDA_EXCLUDEFROMCAPTURE
            } else {
                WDA_NONE
            },
        )?;
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect)?;
        SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOSIZE).unwrap();
        SetWindowPos(hwnd, HWND_TOPMOST, rect.left, rect.top, 0, 0, SWP_NOSIZE).unwrap();
    }
    Ok(())
}
