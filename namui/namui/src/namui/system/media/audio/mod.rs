mod atomic_floating;
mod audio_buffer;
mod audio_config;
mod audio_context;
mod audio_resampling;
mod full_load_once_audio;
mod mixed_audio;
mod raw_audio;
mod stoppable_audio;

pub(crate) use audio_buffer::*;
pub use audio_config::*;
pub(crate) use audio_context::*;
pub(crate) use audio_resampling::*;
pub use full_load_once_audio::*;
pub use mixed_audio::*;
pub use raw_audio::*;
use std::fmt::Debug;
pub use stoppable_audio::*;

pub trait AudioConsume: Debug + Send {
    /// output: packed(interleaved) f32 samples. Stereo.
    fn consume(&mut self, output: &mut [f32]);
    fn is_end(&self) -> bool;
}
