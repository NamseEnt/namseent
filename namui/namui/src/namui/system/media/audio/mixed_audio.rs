use super::AudioConsume;

#[derive(Debug)]
pub struct MixedAudio {
    audio_consume: Vec<Box<dyn AudioConsume>>,
}

impl MixedAudio {
    pub fn new<Audio: AudioConsume + 'static>(audios: impl IntoIterator<Item = Audio>) -> Self {
        Self {
            audio_consume: audios
                .into_iter()
                .map(|audio| Box::new(audio) as _)
                .collect(),
        }
    }
}

impl AudioConsume for MixedAudio {
    fn consume(&mut self, output: &mut [f32]) {
        for audio in &mut self.audio_consume {
            audio.consume(output);
        }
    }

    fn is_end(&self) -> bool {
        self.audio_consume.iter().all(|audio| audio.is_end())
    }
}
