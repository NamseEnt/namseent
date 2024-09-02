mod atomic_floating;
mod audio_buffer;
mod audio_config;
mod audio_context;
mod audio_resampling;
mod full_load_once_audio;
mod full_load_repeat_audio;

pub(crate) use audio_buffer::*;
pub(crate) use audio_config::*;
pub(crate) use audio_context::*;
pub(crate) use audio_resampling::*;
pub use full_load_once_audio::*;
pub use full_load_repeat_audio::*;
use std::fmt::Debug;

pub trait AudioConsume: Debug + Send {
    fn consume(&mut self, output: &mut [f32]);
    fn is_end(&self) -> bool;
}
