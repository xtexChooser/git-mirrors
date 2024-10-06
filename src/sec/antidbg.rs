use anyhow::{bail, Result};
use windows::Win32::System::{
    Diagnostics::Debug::{CheckRemoteDebuggerPresent, IsDebuggerPresent},
    Threading::GetCurrentProcess,
};

use super::c;

pub fn detect_debuggers() -> Result<()> {
    // IsDebuggerPresent
    if unsafe { IsDebuggerPresent().as_bool() } {
        bail!("1a03f32f-36fd-4394-bbd3-3ff94a7d6769")
    }

    // PEB.BeingDebugged
    if unsafe { c::check_peb_debugged() } {
        bail!("165b64db-8224-41b0-84a3-7ff9cea6fadb")
    }

    // CheckRemoteDebuggerPresent
    check_remote_dbg_present()?;

    // int3
    if unsafe { c::check_intr3() } {
        bail!("cb8b6d06-5d03-4871-8229-5ccf55ab327c")
    }

    // DR0-DR3 registers
    if unsafe { c::check_dr_regs() } {
        bail!("0a7a14ef-a105-4dfd-a8b0-7956e348c3fe")
    }

    // trap
    if unsafe { c::check_trap() } {
        bail!("b9a91540-ea79-4fed-9f3b-60b7f8a7f75d")
    }

    Ok(())
}

fn check_remote_dbg_present() -> Result<()> {
    unsafe {
        let mut ret = Default::default();
        CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut ret)?;
        if ret.as_bool() {
            bail!("6db9321b-53b1-418d-9da6-49cae5bc2a13")
        }
    }
    Ok(())
}
