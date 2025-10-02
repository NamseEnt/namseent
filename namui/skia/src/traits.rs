use crate::*;
pub use anyhow::Result;
use namui_type::*;
use std::sync::Arc;

pub type JoinHandle<T> = tokio::task::JoinHandle<T>;

pub trait SkImage {
    fn info(&self) -> ImageInfo;
}
