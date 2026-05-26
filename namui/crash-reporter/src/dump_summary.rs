//! Parses a minidump and extracts the two pieces of information we feed back
//! into `CrashContext` / `intake_crash`:
//!
//! - `stack_hash` — SPEC §4 recommended algorithm: SHA-256 hex over the
//!   crashing thread's top-N frames in `module_basename!0x<offset_from_base>`
//!   form. Symbol-free; uses `minidump-unwind` so we get more than just the
//!   IP register.
//! - `error_message` — a human-readable description of *why* the process
//!   crashed (SIGSEGV / EXCEPTION_ACCESS_VIOLATION / etc.), derived from
//!   the minidump's exception code via `MinidumpException::get_crash_reason`.

use crate::Error;
use minidump::{
    Minidump, MinidumpException, MinidumpModuleList, MinidumpSystemInfo, MinidumpThreadList,
    Module,
};
use minidump_unwind::{
    CallStack, SystemInfo as UnwindSystemInfo, Symbolizer, simple_symbol_supplier, walk_stack,
};
use sha2::{Digest, Sha256};
use std::path::Path;

const TOP_N: usize = 10;

pub struct DumpSummary {
    pub stack_hash: String,
    pub error_message: Option<String>,
}

pub fn parse(dump_path: &Path) -> Result<DumpSummary, Error> {
    let dump = Minidump::read_path(dump_path)?;
    let exception: MinidumpException = dump.get_stream()?;
    let threads: MinidumpThreadList = dump.get_stream()?;
    let modules: MinidumpModuleList = dump.get_stream()?;
    let mdsi: MinidumpSystemInfo = dump.get_stream()?;
    let memory_list = dump.get_memory().unwrap_or_default();

    let crashing_tid = exception.get_crashing_thread_id();
    let thread = threads
        .threads
        .iter()
        .find(|t| t.raw.thread_id == crashing_tid)
        .ok_or(Error::MissingCrashingContext)?;
    let context = exception
        .context(&mdsi, None)
        .or_else(|| thread.context(&mdsi, None))
        .ok_or(Error::MissingCrashingContext)?
        .into_owned();

    let (os_version, os_build) = mdsi.os_parts();
    let unwind_system_info = UnwindSystemInfo {
        os: mdsi.os,
        os_version: Some(os_version),
        os_build,
        cpu: mdsi.cpu,
        cpu_info: mdsi.cpu_info().map(|c| c.into_owned()),
        cpu_microcode_version: None,
        cpu_count: 1,
    };

    let mut stack = CallStack::with_context(context);
    let symbolizer = Symbolizer::new(simple_symbol_supplier(vec![]));

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(walk_stack(
        0,
        (),
        &mut stack,
        thread.stack_memory(&memory_list),
        &modules,
        &unwind_system_info,
        &symbolizer,
    ));

    let mut lines = Vec::with_capacity(TOP_N);
    for frame in stack.frames.iter().take(TOP_N) {
        match &frame.module {
            Some(module) => {
                let raw = module.code_file();
                let name = Path::new(raw.as_ref())
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| raw.clone().into_owned());
                let offset = frame.instruction.saturating_sub(module.base_address());
                lines.push(format!("{name}!0x{offset:x}"));
            }
            None => lines.push(format!("0x{:x}", frame.instruction)),
        }
    }
    let stack_hash = hex::encode(Sha256::digest(lines.join("\n").as_bytes()));

    let error_message = Some(format!(
        "{}",
        exception.get_crash_reason(mdsi.os, mdsi.cpu)
    ));

    Ok(DumpSummary {
        stack_hash,
        error_message,
    })
}
