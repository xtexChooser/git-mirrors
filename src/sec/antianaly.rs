use std::hash::{DefaultHasher, Hash, Hasher};

use anyhow::{bail, Result};
use phf::phf_set;
use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System, UpdateKind};

const FORBIDDEN_PROCS: phf::Set<&str> = phf_set![
    "ollydbg.exe",                      // OllyDebug debugger
    "ollyice.exe",                      // OllyDebug debugger
    "ProcessHacker.exe",                // Process Hacker
    "tcpview.exe",                      // Part of Sysinternals Suite
    "autoruns.exe",                     // Part of Sysinternals Suite
    "autorunsc.exe",                    // Part of Sysinternals Suite
    "filemon.exe",                      // Part of Sysinternals Suite
    "procmon.exe",                      // Part of Sysinternals Suite
    "regmon.exe",                       // Part of Sysinternals Suite
    "procexp.exe",                      // Part of Sysinternals Suite
    "idaq.exe",                         // IDA Pro Interactive Disassembler
    "idaq64.exe",                       // IDA Pro Interactive Disassembler
    "ImmunityDebugger.exe",             // ImmunityDebugger
    "Wireshark.exe",                    // Wireshark packet sniffer
    "dumpcap.exe",                      // Network traffic dump tool
    "HookExplorer.exe",                 // Find various types of runtime hooks
    "ImportREC.exe",                    // Import Reconstructor
    "PETools.exe",                      // PE Tool
    "LordPE.exe",                       // LordPE
    "SysInspector.exe",                 // ESET SysInspector
    "proc_analyzer.exe",                // Part of SysAnalyzer iDefense
    "sysAnalyzer.exe",                  // Part of SysAnalyzer iDefense
    "sniff_hit.exe",                    // Part of SysAnalyzer iDefense
    "windbg.exe",                       // Microsoft WinDbg
    "joeboxcontrol.exe",                // Part of Joe Sandbox
    "joeboxserver.exe",                 // Part of Joe Sandbox
    "ResourceHacker.exe",               // Resource Hacker
    "x32dbg.exe",                       // x32dbg
    "x64dbg.exe",                       // x64dbg
    "Fiddler.exe",                      // Fiddler
    "httpdebugger.exe",                 // Http Debugger
    "cheatengine-i386.exe",             // Cheat Engine
    "cheatengine-x86_64.exe",           // Cheat Engine
    "cheatengine-x86_64-SSE4-AVX2.exe", // Cheat Engine
    "frida-helper-32.exe",              // Frida
    "frida-helper-64.exe",              // Frida
];

pub fn detect_analysis_tools() -> Result<()> {
    let sysinfo = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_exe(UpdateKind::Always)),
    );
    for exe in &FORBIDDEN_PROCS {
        let proc = sysinfo
            .processes()
            .values()
            .filter(move |val: &&Process| val.name().eq_ignore_ascii_case(exe))
            .next();
        if let Some(proc) = proc {
            let mut hasher = DefaultHasher::new();
            proc.exe()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .hash(&mut hasher);
            bail!("a7f0f209-04bf-4107-82d3-8ae5a1f11a6e: {}", hasher.finish())
        }
    }
    Ok(())
}
