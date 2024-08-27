use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    /// libavif operation failed with result `code`
    Code(u32),
    /// The image pixel format isn't supported or the specified `width` and `height` don't
    /// match the pixel buffer size
    UnsupportedImageType,
    StdIo(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Code(code) => write!(f, "libavif error: {}", code),
            Error::UnsupportedImageType => f.write_str("unsupported image type"),
            Error::StdIo(err) => write!(f, "std::io error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdIo(err)
    }
}
