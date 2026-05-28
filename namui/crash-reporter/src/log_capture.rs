//! Captures the host process's stdout + stderr into a fixed-size ring buffer
//! (mmap-backed shared file) so the crash child can read it back as
//! `CrashContext::log_tail` (SPEC §2 caps at 64 KiB).
//!
//! Layout: `[ 8 bytes magic | 8 bytes LE u64 total_bytes_written | 64 KiB ring ]`
//!
//! Writer (parent): a dedicated thread `read()`s the pipe end that we duped
//! over `STDOUT_FILENO` / `STDERR_FILENO` (Unix) or routed via
//! `SetStdHandle` (Windows), mirrors each chunk to the saved real console so
//! the user still sees output, then writes to the ring and bumps the total
//! counter (release fence so the child reads a consistent picture).
//!
//! Reader (child): opens the same file via the path the parent stashed in
//! `NAMUI_CRASH_LOG_RING`, validates the magic, and returns the last 64 KiB
//! of valid bytes (wrapping if `total > capacity`).

use crate::{Error, queue};
use memmap2::{Mmap, MmapMut};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::atomic::{Ordering, fence},
    thread,
};

const MAGIC: &[u8; 8] = b"NAMSHLOG";
const HEADER_LEN: usize = 16;
const RING_CAPACITY: usize = 64 * 1024;
const FILE_SIZE: u64 = (HEADER_LEN + RING_CAPACITY) as u64;
const ENV_LOG_RING: &str = "NAMUI_CRASH_LOG_RING";

pub struct LogCapture {
    _restore: PlatformRestore,
}

pub fn start(app_name: &str) -> Result<LogCapture, Error> {
    let path = queue::root_dir(app_name)?.join("log_ring");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(&path)?;
    file.set_len(FILE_SIZE)?;
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };
    mmap[0..8].copy_from_slice(MAGIC);
    mmap[8..16].copy_from_slice(&0u64.to_le_bytes());
    mmap.flush()?;

    // SAFETY: caller invokes `start` early in `main`, before spawning any
    // other threads — std::env access is single-threaded at this point.
    unsafe {
        std::env::set_var(ENV_LOG_RING, &path);
    }

    let (restore, pipe_reader, mirror) = redirect_stdio()?;
    thread::Builder::new()
        .name("crash-reporter-log-tail".into())
        .spawn(move || drain(pipe_reader, mmap, mirror))?;

    Ok(LogCapture { _restore: restore })
}

pub fn read_tail() -> Option<String> {
    let path = std::env::var_os(ENV_LOG_RING)?;
    let bytes = read_ring(Path::new(&path)).ok().flatten()?;
    if bytes.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn read_ring(path: &Path) -> std::io::Result<Option<Vec<u8>>> {
    let file = File::open(path)?;
    if file.metadata()?.len() < FILE_SIZE {
        return Ok(None);
    }
    let mmap = unsafe { Mmap::map(&file)? };
    if &mmap[0..8] != MAGIC {
        return Ok(None);
    }
    fence(Ordering::Acquire);
    let total = u64::from_le_bytes(mmap[8..16].try_into().unwrap()) as usize;
    let valid = total.min(RING_CAPACITY);
    let ring = &mmap[HEADER_LEN..HEADER_LEN + RING_CAPACITY];
    let mut buf = vec![0u8; valid];
    if total <= RING_CAPACITY {
        buf.copy_from_slice(&ring[..valid]);
    } else {
        let start = total % RING_CAPACITY;
        let tail_len = RING_CAPACITY - start;
        buf[..tail_len].copy_from_slice(&ring[start..]);
        buf[tail_len..].copy_from_slice(&ring[..start]);
    }
    Ok(Some(buf))
}

fn drain<R: Read>(mut pipe_reader: R, mut mmap: MmapMut, mut mirror: Mirror) {
    let mut chunk = [0u8; 4096];
    let mut total: u64 = 0;
    loop {
        let n = match pipe_reader.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        let bytes = &chunk[..n];
        let _ = mirror.write_all(bytes);
        write_ring(&mut mmap, total, bytes);
        total = total.saturating_add(n as u64);
        fence(Ordering::Release);
        mmap[8..16].copy_from_slice(&total.to_le_bytes());
    }
}

fn write_ring(mmap: &mut MmapMut, total: u64, bytes: &[u8]) {
    let ring = &mut mmap[HEADER_LEN..HEADER_LEN + RING_CAPACITY];
    let pos = (total as usize) % RING_CAPACITY;
    let first_chunk = bytes.len().min(RING_CAPACITY - pos);
    ring[pos..pos + first_chunk].copy_from_slice(&bytes[..first_chunk]);
    if first_chunk < bytes.len() {
        let rest = &bytes[first_chunk..];
        let rest_len = rest.len().min(RING_CAPACITY);
        ring[..rest_len].copy_from_slice(&rest[..rest_len]);
    }
}

// ---------- Platform-specific stdio redirect ----------

#[cfg(unix)]
struct PlatformRestore {
    saved_stdout: libc::c_int,
    saved_stderr: libc::c_int,
}

#[cfg(unix)]
impl Drop for PlatformRestore {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.saved_stdout, libc::STDOUT_FILENO);
            libc::dup2(self.saved_stderr, libc::STDERR_FILENO);
            libc::close(self.saved_stdout);
            libc::close(self.saved_stderr);
        }
    }
}

#[cfg(unix)]
struct Mirror {
    fd: libc::c_int,
}

#[cfg(unix)]
impl Write for Mirror {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = unsafe { libc::write(self.fd, buf.as_ptr().cast(), buf.len()) };
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(n as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(unix)]
fn redirect_stdio() -> Result<(PlatformRestore, File, Mirror), Error> {
    use std::os::fd::FromRawFd;
    unsafe {
        let mut pipe_fds = [0i32; 2];
        if libc::pipe(pipe_fds.as_mut_ptr()) != 0 {
            return Err(Error::Io(std::io::Error::last_os_error()));
        }
        let saved_stdout = libc::dup(libc::STDOUT_FILENO);
        let saved_stderr = libc::dup(libc::STDERR_FILENO);
        if saved_stdout < 0 || saved_stderr < 0 {
            return Err(Error::Io(std::io::Error::last_os_error()));
        }
        if libc::dup2(pipe_fds[1], libc::STDOUT_FILENO) < 0
            || libc::dup2(pipe_fds[1], libc::STDERR_FILENO) < 0
        {
            return Err(Error::Io(std::io::Error::last_os_error()));
        }
        libc::close(pipe_fds[1]);
        let pipe_reader = File::from_raw_fd(pipe_fds[0]);
        Ok((
            PlatformRestore {
                saved_stdout,
                saved_stderr,
            },
            pipe_reader,
            Mirror { fd: saved_stdout },
        ))
    }
}

#[cfg(windows)]
struct PlatformRestore {
    saved_stdout: windows::Win32::Foundation::HANDLE,
    saved_stderr: windows::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl Drop for PlatformRestore {
    fn drop(&mut self) {
        use windows::Win32::System::Console::{
            STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle,
        };
        unsafe {
            let _ = SetStdHandle(STD_OUTPUT_HANDLE, self.saved_stdout);
            let _ = SetStdHandle(STD_ERROR_HANDLE, self.saved_stderr);
        }
    }
}

#[cfg(windows)]
struct Mirror {
    handle: windows::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
unsafe impl Send for Mirror {}

#[cfg(windows)]
impl Write for Mirror {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        use windows::Win32::Storage::FileSystem::WriteFile;
        let mut written: u32 = 0;
        let result = unsafe { WriteFile(self.handle, Some(buf), Some(&mut written), None) };
        if result.is_err() {
            return Err(std::io::Error::last_os_error());
        }
        Ok(written as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(windows)]
fn redirect_stdio() -> Result<(PlatformRestore, File, Mirror), Error> {
    use std::os::windows::io::FromRawHandle;
    use windows::Win32::Security::SECURITY_ATTRIBUTES;
    use windows::Win32::System::Console::{
        GetStdHandle, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetStdHandle,
    };
    use windows::Win32::System::Pipes::CreatePipe;
    unsafe {
        let saved_stdout = GetStdHandle(STD_OUTPUT_HANDLE)
            .map_err(|e| Error::Io(std::io::Error::other(e.to_string())))?;
        let saved_stderr = GetStdHandle(STD_ERROR_HANDLE)
            .map_err(|e| Error::Io(std::io::Error::other(e.to_string())))?;

        let mut read_handle = windows::Win32::Foundation::HANDLE::default();
        let mut write_handle = windows::Win32::Foundation::HANDLE::default();
        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: std::ptr::null_mut(),
            bInheritHandle: windows::core::BOOL(1),
        };
        CreatePipe(&mut read_handle, &mut write_handle, Some(&sa), 0)
            .map_err(|e| Error::Io(std::io::Error::other(e.to_string())))?;

        SetStdHandle(STD_OUTPUT_HANDLE, write_handle)
            .map_err(|e| Error::Io(std::io::Error::other(e.to_string())))?;
        SetStdHandle(STD_ERROR_HANDLE, write_handle)
            .map_err(|e| Error::Io(std::io::Error::other(e.to_string())))?;

        let pipe_reader = File::from_raw_handle(read_handle.0 as _);
        Ok((
            PlatformRestore {
                saved_stdout,
                saved_stderr,
            },
            pipe_reader,
            Mirror {
                handle: saved_stdout,
            },
        ))
    }
}

#[cfg(not(any(unix, windows)))]
struct PlatformRestore;

#[cfg(not(any(unix, windows)))]
struct Mirror;

#[cfg(not(any(unix, windows)))]
impl Write for Mirror {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(not(any(unix, windows)))]
fn redirect_stdio() -> Result<(PlatformRestore, File, Mirror), Error> {
    Err(Error::Io(std::io::Error::other(
        "log_capture not supported on this platform",
    )))
}
