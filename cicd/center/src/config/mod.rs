pub(crate) mod namui;
pub(crate) mod rust;

pub(crate) use namui::*;
pub(crate) use rust::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CicdConfig {
    version: u8,
    rust: Option<RustConfig>,
    namui: Option<NamuiConfig>,
}
