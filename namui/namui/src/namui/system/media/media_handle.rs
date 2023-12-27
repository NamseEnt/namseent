use super::media_struct::Media;
use anyhow::Result;
use namui_type::ImageHandle;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MediaHandle {
    media: Arc<Mutex<Media>>,
}

impl MediaHandle {
    pub(crate) fn new(media: Media) -> Result<Self> {
        Ok(Self {
            media: Arc::new(Mutex::new(media)),
        })
    }
    pub fn play(&self) -> Result<()> {
        let mut media = self.media.lock().unwrap();
        media.play();

        Ok(())
    }
    pub fn pause(&self) -> Result<()> {
        todo!()
    }
    pub fn is_playing(&self) -> Result<bool> {
        todo!()
    }
    pub fn get_image(&self) -> Result<Option<ImageHandle>> {
        todo!()
    }
}
