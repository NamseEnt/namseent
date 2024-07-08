use std::{
    io::Cursor,
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub struct InputContext {
    context: ffmpeg_next::format::context::Input,
    _destroyer: Arc<InputContextDestroyer>,
}
impl InputContext {
    pub fn wrap(context: ffmpeg_next::format::context::Input, mode: InputContextMode) -> Self {
        Self {
            context,
            _destroyer: Arc::new(InputContextDestroyer(mode)),
        }
    }
}

impl AsRef<ffmpeg_next::format::context::Input> for InputContext {
    fn as_ref(&self) -> &ffmpeg_next::format::context::Input {
        &self.context
    }
}
impl AsMut<ffmpeg_next::format::context::Input> for InputContext {
    fn as_mut(&mut self) -> &mut ffmpeg_next::format::context::Input {
        &mut self.context
    }
}
impl Deref for InputContext {
    type Target = ffmpeg_next::format::context::Input;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
impl DerefMut for InputContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

pub struct InputBytes(pub(crate) Arc<Vec<u8>>);
impl AsRef<[u8]> for InputBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub enum InputContextMode {
    Path,
    Bytes(*mut Cursor<InputBytes>),
}
unsafe impl Send for InputContextMode {}
unsafe impl Sync for InputContextMode {}

struct InputContextDestroyer(InputContextMode);
impl Drop for InputContextDestroyer {
    fn drop(&mut self) {
        match self.0 {
            InputContextMode::Path => {}
            InputContextMode::Bytes(byte_reader) => unsafe {
                byte_reader.drop_in_place();
            },
        }
    }
}
