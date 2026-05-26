//! Computes a stable hash for the crashing thread's top frame so namsh can group
//! recurrences of the same crash. Symbol-free: derives `module_name+0xoffset`
//! purely from the minidump's `MinidumpModuleList`, then SHA-256s the result.

use crate::Error;
use minidump::{
    Minidump, MinidumpException, MinidumpModule, MinidumpModuleList, MinidumpSystemInfo,
    MinidumpThreadList, Module,
};
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn compute(dump_path: &Path) -> Result<String, Error> {
    let dump = Minidump::read_path(dump_path)?;
    let exception: MinidumpException = dump.get_stream()?;
    let threads: MinidumpThreadList = dump.get_stream()?;
    let modules: MinidumpModuleList = dump.get_stream()?;
    let sys_info: MinidumpSystemInfo = dump.get_stream()?;

    let crashing_tid = exception.get_crashing_thread_id();
    let context = match exception.context(&sys_info, None) {
        Some(c) => c,
        None => {
            let thread = threads
                .threads
                .iter()
                .find(|t| t.raw.thread_id == crashing_tid)
                .ok_or(Error::MissingCrashingContext)?;
            thread
                .context(&sys_info, None)
                .ok_or(Error::MissingCrashingContext)?
        }
    };

    let ip = context.get_instruction_pointer();
    let frame = match modules.module_at_address(ip) {
        Some(module) => format!(
            "{}!0x{:x}",
            module_basename(module),
            ip.saturating_sub(module.base_address())
        ),
        None => format!("0x{ip:x}"),
    };

    let digest = Sha256::digest(frame.as_bytes());
    Ok(hex::encode(digest))
}

fn module_basename(module: &MinidumpModule) -> String {
    let name = module.name.clone();
    name.rsplit(['/', '\\'])
        .next()
        .map(str::to_string)
        .unwrap_or(name)
}
