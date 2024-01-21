use super::{AudioConsume, AudioContext};
use anyhow::Result;
use std::sync::{atomic::AtomicBool, Arc};

#[derive(Debug)]
struct StoppableAudioInner {
    audio_consume: Box<dyn AudioConsume>,
    stopped: Arc<AtomicBool>,
}

#[derive(Debug)]
pub struct StoppableAudio {
    stopped: Arc<AtomicBool>,
}

impl StoppableAudio {
    pub fn load(audio_context: &AudioContext, audio: impl AudioConsume + 'static) -> Result<Self> {
        let stopped = Arc::new(AtomicBool::new(false));

        audio_context.load_audio(StoppableAudioInner {
            audio_consume: Box::new(audio),
            stopped: stopped.clone(),
        })?;

        Ok(Self { stopped })
    }

    pub fn stop(&self) {
        self.stopped
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

impl AudioConsume for StoppableAudioInner {
    fn consume(&mut self, output: &mut [f32]) {
        self.audio_consume.consume(output)
    }

    fn is_end(&self) -> bool {
        self.stopped.load(std::sync::atomic::Ordering::Relaxed) || self.audio_consume.is_end()
    }
}
