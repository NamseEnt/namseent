use super::InitResult;
use anyhow::*;
use namui_media::*;
use std::{path::Path, sync::OnceLock};

static MEDIA_SYSTEM: OnceLock<MediaContext> = OnceLock::new();

// TODO: Restore Media system
pub(super) async fn init() -> InitResult {
    MEDIA_SYSTEM
        .set(MediaContext::new()?)
        .map_err(|_| anyhow!("Media system already initialized"))?;

    Ok(())
}

pub fn new_media(path: &impl AsRef<Path>) -> Result<MediaHandle> {
    MEDIA_SYSTEM.get().unwrap().new_media(path)
}

// pub fn play_media_source(media_source: &MediaSource) {
//     MEDIA_SYSTEM.get().unwrap().play(media_source);
// }

// pub struct Media {
//     buffer: Arc<Mutex<Option<MediaBuffer>>>,
//     source: Option<MediaBufferSourceNode>,
//     is_loaded: Arc<AtomicBool>,
//     is_loop: bool,
// }

// impl Clone for Media {
//     fn clone(&self) -> Self {
//         Media {
//             buffer: self.buffer.clone(),
//             source: None,
//             is_loaded: self.is_loaded.clone(),
//             is_loop: self.is_loop,
//         }
//     }
// }

// impl Media {
//     pub fn new(
//         url: Url,
//         on_loaded: impl Fn() + 'static,
//         on_error: impl Fn(Box<dyn std::error::Error>) + 'static,
//     ) -> Self {
//         let buffer = Arc::new(Mutex::new(None));
//         let is_loaded = Arc::new(AtomicBool::new(false));
//         spawn_local({
//             let buffer = buffer.clone();
//             let is_loaded = is_loaded.clone();
//             async move {
//                 match create_media_buffer_from_url(url).await {
//                     Ok(media_buffer) => {
//                         {
//                             let mut buffer = buffer.lock().unwrap();
//                             *buffer = Some(media_buffer);
//                         }
//                         is_loaded.store(true, std::sync::atomic::Ordering::Relaxed);

//                         on_loaded();
//                     }
//                     Err(error) => {
//                         on_error(error);
//                     }
//                 }
//             }
//         });
//         Media {
//             buffer,
//             is_loaded,
//             source: None,
//             is_loop: false,
//         }
//     }

//     pub fn is_loaded(&self) -> bool {
//         self.is_loaded.load(std::sync::atomic::Ordering::Relaxed)
//     }

//     pub fn play(&mut self) {
//         let buffer = self.buffer.lock().unwrap();
//         let Some(buffer) = buffer.as_ref() else {
//             return;
//         };

//         if let Some(source) = self.source.as_ref() {
//             source.stop().unwrap();
//         }

//         let source = MEDIA_SYSTEM.media_context.create_buffer_source().unwrap();

//         source
//             .connect_with_media_node(&MEDIA_SYSTEM.media_context.destination())
//             .unwrap();

//         source.set_buffer(Some(buffer));
//         source.set_loop(self.is_loop);

//         source.start().unwrap();
//         self.source = Some(source);
//     }

//     pub fn stop(&mut self) {
//         if let Some(source) = self.source.take() {
//             source.stop().unwrap();
//         }
//     }

//     pub fn set_loop(&mut self, is_loop: bool) {
//         self.is_loop = is_loop;

//         if let Some(source) = self.source.as_ref() {
//             source.set_loop(is_loop);
//         }
//     }

//     pub fn is_loop(&self) -> bool {
//         self.is_loop
//     }

//     pub fn play_and_forget(&self) {
//         let mut media = self.clone();
//         media.set_loop(false);
//         media.play();
//     }
// }

// // TODO: Save media buffer source in system to reuse it
// async fn create_media_buffer_from_url(url: Url) -> Result<MediaBuffer>{
//     let bytes = url_to_bytes(&url).await?;
//     let array_buffer = bytes_to_array_buffer(bytes.as_ref());

//     let promise = MEDIA_SYSTEM
//         .media_context
//         .decode_media_data(&array_buffer)
//         .map_err(|error| format!("Failed to decode media data: {:?}", error))?;

//     let media_buffer: MediaBuffer = wasm_bindgen_futures::JsFuture::from(promise)
//         .await
//         .map_err(|error| format!("Failed to await media buffer: {:?}", error))?
//         .into();

//     Ok(media_buffer)
// }

// fn bytes_to_array_buffer(bytes: &[u8]) -> ArrayBuffer {
//     let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);
//     let array = js_sys::Uint8Array::new(&array_buffer);
//     array.copy_from(bytes);
//     array_buffer
// }
