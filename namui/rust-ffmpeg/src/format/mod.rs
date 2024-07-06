use libc::c_void;
pub use util::format::{pixel, Pixel};
pub use util::format::{sample, Sample};
use util::interrupt;

pub mod stream;

pub mod chapter;

pub mod context;
pub use self::context::Context;

pub mod format;
#[cfg(not(feature = "ffmpeg_5_0"))]
pub use self::format::list;
pub use self::format::{flag, Flags};
pub use self::format::{Input, Output};

pub mod network;

use std::ffi::{CStr, CString};
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;
use std::ptr::{self};
use std::str::from_utf8_unchecked;
use std::sync::Arc;

use ffi::*;
use {Dictionary, Error, Format};

const INPUT_BUFFER_SIZE: usize = 32_768;

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register_all() {
    unsafe {
        av_register_all();
    }
}

#[cfg(not(feature = "ffmpeg_5_0"))]
pub fn register(format: &Format) {
    match *format {
        Format::Input(ref format) => unsafe {
            av_register_input_format(format.as_ptr() as *mut _);
        },

        Format::Output(ref format) => unsafe {
            av_register_output_format(format.as_ptr() as *mut _);
        },
    }
}

pub fn version() -> u32 {
    unsafe { avformat_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avformat_license()).to_bytes()) }
}

// XXX: use to_cstring when stable
fn from_path<P: AsRef<Path>>(path: &P) -> CString {
    CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap()
}

// NOTE: this will be better with specialization or anonymous return types
pub fn open<P: AsRef<Path>>(path: &P, format: &Format) -> Result<Context, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match *format {
            Format::Input(ref format) => match avformat_open_input(
                &mut ps,
                path.as_ptr(),
                format.as_ptr() as *mut _,
                ptr::null_mut(),
            ) {
                0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                    r if r >= 0 => Ok(Context::Input(context::Input::wrap(
                        ps,
                        context::input::Mode::Path,
                    ))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },

            Format::Output(ref format) => match avformat_alloc_output_context2(
                &mut ps,
                format.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
            ) {
                0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                    0 => Ok(Context::Output(context::Output::wrap(ps))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn open_with<P: AsRef<Path>>(
    path: &P,
    format: &Format,
    options: Dictionary,
) -> Result<Context, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();

        match *format {
            Format::Input(ref format) => {
                let res = avformat_open_input(
                    &mut ps,
                    path.as_ptr(),
                    format.as_ptr() as *mut _,
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                        r if r >= 0 => Ok(Context::Input(context::Input::wrap(
                            ps,
                            context::input::Mode::Path,
                        ))),
                        e => Err(Error::from(e)),
                    },

                    e => Err(Error::from(e)),
                }
            }

            Format::Output(ref format) => match avformat_alloc_output_context2(
                &mut ps,
                format.as_ptr() as *mut _,
                ptr::null(),
                path.as_ptr(),
            ) {
                0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                    0 => Ok(Context::Output(context::Output::wrap(ps))),
                    e => Err(Error::from(e)),
                },

                e => Err(Error::from(e)),
            },
        }
    }
}

pub fn input<P: AsRef<Path>>(path: &P) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps, context::input::Mode::Path)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn input_bytes(bytes: Arc<Vec<u8>>) -> Result<context::Input, Error> {
    unsafe {
        unsafe extern "C" fn read_packet(
            opaque: *mut c_void,
            buffer: *mut u8,
            _buffer_size: i32,
        ) -> i32 {
            let cursor = &mut *(opaque as *mut Cursor<InputBytes>);
            let buffer = &mut *(buffer as *mut [u8; INPUT_BUFFER_SIZE]);
            match cursor.read(buffer) {
                Ok(read) => read as i32,
                Err(_) => AVERROR_INVALIDDATA,
            }
        }
        unsafe extern "C" fn seek(opaque: *mut c_void, offset: i64, whence: i32) -> i64 {
            let cursor = &mut *(opaque as *mut Cursor<InputBytes>);
            let seek_from = match whence {
                SEEK_SET => SeekFrom::Start(offset as u64),
                SEEK_CUR => SeekFrom::Current(offset),
                SEEK_END => SeekFrom::End(offset),
                AVSEEK_SIZE => return cursor.get_ref().0.len() as i64,
                _ => return -1,
            };
            match cursor.seek(seek_from) {
                Ok(position) => position as i64,
                Err(_error) => -1,
            }
        }
        let buffer: *mut c_void = av_malloc(INPUT_BUFFER_SIZE);
        let bytes: *mut Cursor<InputBytes> =
            Box::into_raw(Box::new(Cursor::new(InputBytes(bytes))));
        let file_name = CString::new("").unwrap();

        let mut ps = avformat_alloc_context();
        let avio = avio_alloc_context(
            buffer as *mut u8,
            INPUT_BUFFER_SIZE as i32,
            0,
            bytes as *mut _,
            Some(read_packet),
            None,
            Some(seek),
        );

        (*ps).pb = avio;
        (*ps).flags = AVFMT_FLAG_CUSTOM_IO;

        let free = || {
            av_free(buffer);
            bytes.drop_in_place();
        };

        match avformat_open_input(
            &mut ps,
            file_name.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(
                    ps,
                    context::input::Mode::Bytes { input_bytes: bytes },
                )),
                e => {
                    avformat_close_input(&mut ps);
                    free();
                    Err(Error::from(e))
                }
            },
            e => {
                free();
                Err(Error::from(e))
            }
        }
    }
}

pub fn input_with_dictionary<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Input, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();
        let res = avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), &mut opts);

        Dictionary::own(opts);

        match res {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps, context::input::Mode::Path)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn input_with_interrupt<P: AsRef<Path>, F>(
    path: &P,
    closure: F,
) -> Result<context::Input, Error>
where
    F: FnMut() -> bool,
{
    unsafe {
        let mut ps = avformat_alloc_context();
        let path = from_path(path);
        (*ps).interrupt_callback = interrupt::new(Box::new(closure)).interrupt;

        match avformat_open_input(&mut ps, path.as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(ps, ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps, context::input::Mode::Path)),
                e => {
                    avformat_close_input(&mut ps);
                    Err(Error::from(e))
                }
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output<P: AsRef<Path>>(path: &P) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);

        match avformat_alloc_output_context2(&mut ps, ptr::null_mut(), ptr::null(), path.as_ptr()) {
            0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                0 => Ok(context::Output::wrap(ps)),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_with<P: AsRef<Path>>(
    path: &P,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let mut opts = options.disown();

        match avformat_alloc_output_context2(&mut ps, ptr::null_mut(), ptr::null(), path.as_ptr()) {
            0 => {
                let res = avio_open2(
                    &mut (*ps).pb,
                    path.as_ptr(),
                    AVIO_FLAG_WRITE,
                    ptr::null(),
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ps)),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as<P: AsRef<Path>>(path: &P, format: &str) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let format = CString::new(format).unwrap();

        match avformat_alloc_output_context2(
            &mut ps,
            ptr::null_mut(),
            format.as_ptr(),
            path.as_ptr(),
        ) {
            0 => match avio_open(&mut (*ps).pb, path.as_ptr(), AVIO_FLAG_WRITE) {
                0 => Ok(context::Output::wrap(ps)),
                e => Err(Error::from(e)),
            },

            e => Err(Error::from(e)),
        }
    }
}

pub fn output_as_with<P: AsRef<Path>>(
    path: &P,
    format: &str,
    options: Dictionary,
) -> Result<context::Output, Error> {
    unsafe {
        let mut ps = ptr::null_mut();
        let path = from_path(path);
        let format = CString::new(format).unwrap();
        let mut opts = options.disown();

        match avformat_alloc_output_context2(
            &mut ps,
            ptr::null_mut(),
            format.as_ptr(),
            path.as_ptr(),
        ) {
            0 => {
                let res = avio_open2(
                    &mut (*ps).pb,
                    path.as_ptr(),
                    AVIO_FLAG_WRITE,
                    ptr::null(),
                    &mut opts,
                );

                Dictionary::own(opts);

                match res {
                    0 => Ok(context::Output::wrap(ps)),
                    e => Err(Error::from(e)),
                }
            }

            e => Err(Error::from(e)),
        }
    }
}

pub struct InputBytes(Arc<Vec<u8>>);
impl AsRef<[u8]> for InputBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
