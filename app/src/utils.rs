use std::ffi::c_void;

use anyhow::Result;
use egui::Id;
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE, HWND, NTSTATUS, RECT},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        Threading::{
            OpenProcess, OpenThread, TerminateThread, PROCESS_SUSPEND_RESUME, THREAD_TERMINATE,
        },
    },
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
        SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOSIZE)?;
        SetWindowPos(hwnd, HWND_TOPMOST, rect.left, rect.top, 0, 0, SWP_NOSIZE)?;
    }
    Ok(())
}

pub fn suspend_process(pid: u32) -> Result<()> {
    windows_targets::link!("ntdll.dll" "system" fn NtSuspendProcess(handle : HANDLE) -> NTSTATUS);
    unsafe {
        let handle = OpenProcess(PROCESS_SUSPEND_RESUME, BOOL(0), pid)?;
        NtSuspendProcess(handle).ok()?;
        CloseHandle(handle)?;
    }
    Ok(())
}

pub fn resume_process(pid: u32) -> Result<()> {
    windows_targets::link!("ntdll.dll" "system" fn NtResumeProcess(handle : HANDLE) -> NTSTATUS);
    unsafe {
        let handle = OpenProcess(PROCESS_SUSPEND_RESUME, BOOL(0), pid)?;
        NtResumeProcess(handle).ok()?;
        CloseHandle(handle)?;
    }
    Ok(())
}

pub fn force_kill_process(pid: u32) -> Result<()> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, pid)?;
        let mut entry = THREADENTRY32 {
            dwSize: std::mem::size_of::<THREADENTRY32>() as u32,
            ..Default::default()
        };
        Thread32First(snapshot, &mut entry)?;
        loop {
            if entry.th32OwnerProcessID == pid {
                let thread = OpenThread(THREAD_TERMINATE, BOOL(0), entry.th32ThreadID)?;
                TerminateThread(thread, 0)?;
                CloseHandle(thread)?;
            }
            if Thread32Next(snapshot, &mut entry).is_err() {
                break;
            }
        }
        CloseHandle(snapshot)?;
    }
    Ok(())
}
