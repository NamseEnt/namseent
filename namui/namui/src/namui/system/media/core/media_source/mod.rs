mod input_bytes;

use super::input_context::{InputBytes, InputContext, InputContextMode};
use input_bytes::input_bytes;
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub enum MediaSource {
    Path(std::path::PathBuf),
    Bytes(Arc<Vec<u8>>),
}
impl MediaSource {
    pub(crate) fn create_input_context(
        &self,
    ) -> std::result::Result<InputContext, ffmpeg_next::Error> {
        match self {
            MediaSource::Path(path) => {
                let context = ffmpeg_next::format::input(path)?;
                Ok(InputContext::wrap(context, InputContextMode::Path))
            }
            MediaSource::Bytes(bytes) => {
                let bytes: *mut Cursor<InputBytes> =
                    Box::into_raw(Box::new(Cursor::new(InputBytes(bytes.clone()))));
                let context = input_bytes(bytes)?;
                Ok(InputContext::wrap(context, InputContextMode::Bytes(bytes)))
            }
        }
    }
}

impl From<&Path> for MediaSource {
    fn from(value: &Path) -> Self {
        MediaSource::Path(value.to_path_buf())
    }
}
impl From<&PathBuf> for MediaSource {
    fn from(value: &PathBuf) -> Self {
        MediaSource::Path(value.to_path_buf())
    }
}
impl From<PathBuf> for MediaSource {
    fn from(value: PathBuf) -> Self {
        MediaSource::Path(value)
    }
}
impl From<&str> for MediaSource {
    fn from(value: &str) -> Self {
        MediaSource::Path(Path::new(value).to_path_buf())
    }
}
impl From<&String> for MediaSource {
    fn from(value: &String) -> Self {
        MediaSource::Path(Path::new(value).to_path_buf())
    }
}
impl From<String> for MediaSource {
    fn from(value: String) -> Self {
        MediaSource::Path(Path::new(&value).to_path_buf())
    }
}
impl From<Vec<u8>> for MediaSource {
    fn from(value: Vec<u8>) -> Self {
        MediaSource::Bytes(Arc::new(value))
    }
}
impl From<&Vec<u8>> for MediaSource {
    fn from(value: &Vec<u8>) -> Self {
        MediaSource::Bytes(Arc::new(value.clone()))
    }
}
impl From<&[u8]> for MediaSource {
    fn from(value: &[u8]) -> Self {
        MediaSource::Bytes(Arc::new(value.to_vec()))
    }
}
impl From<Arc<Vec<u8>>> for MediaSource {
    fn from(value: Arc<Vec<u8>>) -> Self {
        MediaSource::Bytes(value)
    }
}
