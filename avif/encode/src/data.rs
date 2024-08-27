use libavif_sys::*;
use std::ops::Deref;

pub struct EncodedData {
    inner: avifRWData,
}

impl EncodedData {
    pub(crate) fn new(inner: avifRWData) -> Self {
        Self { inner }
    }
}

impl AsRef<[u8]> for EncodedData {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let data = self.inner;
            std::slice::from_raw_parts(data.data, data.size)
        }
    }
}

impl Deref for EncodedData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Drop for EncodedData {
    fn drop(&mut self) {
        unsafe {
            avifRWDataFree(&mut self.inner);
        }
    }
}
