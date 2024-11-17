use super::*;

pub struct Cache {}
impl Cache {
    pub(crate) fn get(&self, key: Key) -> Option<Option<bytes::Bytes>> {
        todo!()
    }

    pub(crate) fn put(&self, key: Key, value: Option<Bytes>) {
        todo!()
    }
}
