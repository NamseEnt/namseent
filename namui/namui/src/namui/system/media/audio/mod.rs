mod atomic_floating;
mod audio_buffer;
mod audio_config;
mod audio_context;
mod audio_resampling;
mod full_load_once_audio;

pub(crate) use audio_buffer::*;
pub(crate) use audio_config::*;
pub(crate) use audio_context::*;
pub(crate) use audio_resampling::*;
pub use full_load_once_audio::*;
use std::fmt::Debug;

pub(crate) trait AudioConsume: Debug + Send {
    fn consume(&mut self, output: &mut [f32]);
    fn is_end(&self) -> bool;
}
