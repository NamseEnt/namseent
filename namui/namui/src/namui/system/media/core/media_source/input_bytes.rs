use crate::media::core::input_context::InputBytes;
use ffmpeg_next::{
    ffi::{
        av_free, av_malloc, avformat_alloc_context, avformat_close_input,
        avformat_find_stream_info, avformat_open_input, avio_alloc_context, AVERROR_INVALIDDATA,
        AVFMT_FLAG_CUSTOM_IO, AVSEEK_SIZE, SEEK_CUR, SEEK_END, SEEK_SET,
    },
    format::context,
    Error,
};
use std::{
    ffi::{c_void, CString},
    io::{Cursor, Read, Seek, SeekFrom},
};

const INPUT_BUFFER_SIZE: usize = 32_768;

pub fn input_bytes(bytes: *mut Cursor<InputBytes>) -> Result<context::Input, Error> {
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
        };

        match avformat_open_input(
            &mut ps,
            file_name.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ) {
            0 => match avformat_find_stream_info(ps, std::ptr::null_mut()) {
                r if r >= 0 => Ok(context::Input::wrap(ps)),
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
