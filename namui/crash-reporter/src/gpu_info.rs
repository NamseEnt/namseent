//! Detects the host machine's primary GPU adapter so the child can populate
//! `CrashContext::gpu_adapter`. macOS uses Metal's system-default device name,
//! Windows enumerates DXGI hardware adapters and takes the first non-software
//! one. Other platforms return `None`.
//!
//! Note: `gpu_driver` is currently NOT populated. Metal does not expose a
//! driver-version string, and DXGI's `DXGI_ADAPTER_DESC1` does not carry one
//! either (driver versions live in the Windows registry / `D3DKMTQueryAdapterInfo`,
//! and format varies per vendor).

const ENV_GPU_ADAPTER: &str = "NAMUI_CRASH_GPU_ADAPTER";

pub fn export_to_env() {
    if let Some(adapter) = detect() {
        // SAFETY: callers invoke this early in `main`, before spawning any
        // other threads. std::env access is single-threaded at this point.
        unsafe {
            std::env::set_var(ENV_GPU_ADAPTER, adapter);
        }
    }
}

pub(crate) fn read_from_env() -> Option<String> {
    std::env::var(ENV_GPU_ADAPTER)
        .ok()
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "macos")]
fn detect() -> Option<String> {
    let device = metal_rs::Device::system_default()?;
    Some(device.name().to_string())
}

#[cfg(windows)]
fn detect() -> Option<String> {
    use windows::Win32::Graphics::Dxgi::{
        CreateDXGIFactory2, DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_NONE, DXGI_ADAPTER_FLAG_SOFTWARE,
        IDXGIFactory4,
    };
    let factory: IDXGIFactory4 = unsafe { CreateDXGIFactory2(0).ok()? };
    for i in 0.. {
        let adapter = unsafe { factory.EnumAdapters1(i).ok()? };
        let desc = unsafe { adapter.GetDesc1().ok()? };
        if (DXGI_ADAPTER_FLAG(desc.Flags as i32) & DXGI_ADAPTER_FLAG_SOFTWARE)
            != DXGI_ADAPTER_FLAG_NONE
        {
            continue;
        }
        let len = desc
            .Description
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(desc.Description.len());
        return Some(String::from_utf16_lossy(&desc.Description[..len]));
    }
    None
}

#[cfg(not(any(target_os = "macos", windows)))]
fn detect() -> Option<String> {
    None
}
