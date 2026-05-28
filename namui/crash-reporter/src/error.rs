use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("crash-handler: {0}")]
    CrashHandler(#[from] crash_handler::Error),

    #[error("minidumper: {0:?}")]
    Minidumper(minidumper::Error),

    #[error("minidump parse: {0}")]
    MinidumpParse(#[from] minidump::Error),

    #[error("hex decode: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("invalid hmac key length")]
    HmacKey,

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("http: {0}")]
    Http(#[from] reqwest::Error),

    #[error("namsh intake rejected: {0}")]
    NamshRejected(String),

    #[error("child process failed to bind socket within timeout")]
    ChildConnectTimeout,

    #[error("user data dir unavailable")]
    NoUserDataDir,

    #[error("crashing thread context missing")]
    MissingCrashingContext,
}

impl From<minidumper::Error> for Error {
    fn from(e: minidumper::Error) -> Self {
        Error::Minidumper(e)
    }
}
