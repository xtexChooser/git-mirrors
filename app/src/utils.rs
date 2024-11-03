use std::{
    ffi::c_void,
    future::Future,
    sync::{Arc, RwLock},
};

use anyhow::Result;
use egui::Id;
use tokio::task::JoinHandle;
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

#[derive(Clone)]
pub enum AsyncTaskState<T> {
    NotStarted,
    Running,
    Done(Arc<T>),
}

impl<T> AsyncTaskState<T> {
    pub fn started(&self) -> bool {
        !matches!(self, Self::NotStarted)
    }

    pub fn running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn done(&self) -> bool {
        matches!(self, Self::Done(_))
    }

    pub fn try_result(&self) -> Option<Arc<T>> {
        if let Self::Done(result) = self {
            Some(result.to_owned())
        } else {
            None
        }
    }

    pub fn result(&self) -> Arc<T> {
        if let Self::Done(result) = self {
            result.to_owned()
        } else {
            panic!("calling AsyncTaskState::result on incomplete task")
        }
    }
}

enum AsyncTaskStateInner<T> {
    NotStarted,
    Running(JoinHandle<()>),
    Done(Arc<T>),
}

impl<T> From<&AsyncTaskStateInner<T>> for AsyncTaskState<T> {
    fn from(value: &AsyncTaskStateInner<T>) -> Self {
        match value {
            AsyncTaskStateInner::NotStarted => Self::NotStarted,
            AsyncTaskStateInner::Running(_) => Self::Running,
            AsyncTaskStateInner::Done(v) => Self::Done(v.clone()),
        }
    }
}

#[derive(Clone)]
pub struct AsyncTask<T>(Arc<RwLock<AsyncTaskStateInner<T>>>)
where
    T: Send + Sync + 'static;

impl<T: Send + Sync + 'static> AsyncTask<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(AsyncTaskStateInner::NotStarted)))
    }

    pub fn start<F>(&self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        let mut state = self.0.write().unwrap();
        if let AsyncTaskStateInner::Running(handle) = &*state {
            handle.abort();
        }
        let state_container = self.0.clone();
        let handle = tokio::spawn(async move {
            let result = future.await;
            *state_container.write().unwrap() = AsyncTaskStateInner::Done(Arc::new(result));
        });
        *state = AsyncTaskStateInner::Running(handle)
    }

    pub fn abort(&self) {
        let mut state = self.0.write().unwrap();
        if let AsyncTaskStateInner::Running(handle) = &*state {
            handle.abort();
        }
        *state = AsyncTaskStateInner::NotStarted
    }

    pub fn reset(&self) {
        let mut state = self.0.write().unwrap();
        *state = AsyncTaskStateInner::NotStarted
    }

    pub fn state(&self) -> AsyncTaskState<T> {
        (&*self.0.read().unwrap()).into()
    }
}

impl<T: Send + Sync + 'static> Default for AsyncTask<T> {
    fn default() -> Self {
        Self::new()
    }
}
