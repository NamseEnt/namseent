use super::InitResult;
use crate::url::url_to_bytes;
use js_sys::ArrayBuffer;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use url::Url;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AudioBuffer, AudioBufferSourceNode};

struct AudioSystem {
    audio_context: web_sys::AudioContext,
}

unsafe impl Send for AudioSystem {}
unsafe impl Sync for AudioSystem {}

lazy_static::lazy_static! {
    static ref AUDIO_SYSTEM: Arc<AudioSystem> = Arc::new(AudioSystem::new());
}

pub(super) async fn init() -> InitResult {
    lazy_static::initialize(&AUDIO_SYSTEM);
    Ok(())
}

impl AudioSystem {
    pub fn new() -> Self {
        AudioSystem {
            audio_context: web_sys::AudioContext::new().unwrap(),
        }
    }
}

pub struct Audio {
    buffer: Arc<Mutex<Option<AudioBuffer>>>,
    source: Option<AudioBufferSourceNode>,
    is_loaded: Arc<AtomicBool>,
    is_loop: bool,
}

impl Clone for Audio {
    fn clone(&self) -> Self {
        Audio {
            buffer: self.buffer.clone(),
            source: None,
            is_loaded: self.is_loaded.clone(),
            is_loop: self.is_loop,
        }
    }
}

impl Audio {
    pub fn new(
        url: Url,
        on_loaded: impl Fn() + 'static,
        on_error: impl Fn(Box<dyn std::error::Error>) + 'static,
    ) -> Self {
        let buffer = Arc::new(Mutex::new(None));
        let is_loaded = Arc::new(AtomicBool::new(false));
        spawn_local({
            let buffer = buffer.clone();
            let is_loaded = is_loaded.clone();
            async move {
                match create_audio_buffer_from_url(url).await {
                    Ok(audio_buffer) => {
                        {
                            let mut buffer = buffer.lock().unwrap();
                            *buffer = Some(audio_buffer);
                        }
                        is_loaded.store(true, std::sync::atomic::Ordering::Relaxed);

                        on_loaded();
                    }
                    Err(error) => {
                        on_error(error);
                    }
                }
            }
        });
        Audio {
            buffer,
            is_loaded,
            source: None,
            is_loop: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn play(&mut self) {
        let buffer = self.buffer.lock().unwrap();
        let Some(buffer) = buffer.as_ref() else {
            return;
        };

        if let Some(source) = self.source.as_ref() {
            source.stop().unwrap();
        }

        let source = AUDIO_SYSTEM.audio_context.create_buffer_source().unwrap();

        source
            .connect_with_audio_node(&AUDIO_SYSTEM.audio_context.destination())
            .unwrap();

        source.set_buffer(Some(buffer));
        source.set_loop(self.is_loop);

        source.start().unwrap();
        self.source = Some(source);
    }

    pub fn stop(&mut self) {
        if let Some(source) = self.source.take() {
            source.stop().unwrap();
        }
    }

    pub fn set_loop(&mut self, is_loop: bool) {
        self.is_loop = is_loop;

        if let Some(source) = self.source.as_ref() {
            source.set_loop(is_loop);
        }
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop
    }

    pub fn play_and_forget(&self) {
        let mut audio = self.clone();
        audio.set_loop(false);
        audio.play();
    }
}

// TODO: Save audio buffer source in system to reuse it
async fn create_audio_buffer_from_url(url: Url) -> Result<AudioBuffer> {
    let bytes = url_to_bytes(&url).await?;
    let array_buffer = bytes_to_array_buffer(bytes.as_ref());

    let promise = AUDIO_SYSTEM
        .audio_context
        .decode_audio_data(&array_buffer)
        .map_err(|error| format!("Failed to decode audio data: {:?}", error))?;

    let audio_buffer: AudioBuffer = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|error| format!("Failed to await audio buffer: {:?}", error))?
        .into();

    Ok(audio_buffer)
}

fn bytes_to_array_buffer(bytes: &[u8]) -> ArrayBuffer {
    let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);
    let array = js_sys::Uint8Array::new(&array_buffer);
    array.copy_from(bytes);
    array_buffer
}
