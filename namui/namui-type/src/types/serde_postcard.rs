pub trait SerdePostcardSerialize: serde::Serialize {
    fn to_postcard_vec(&self) -> Vec<u8> {
        postcard::to_allocvec(self).unwrap()
    }
}

pub trait SerdePostcardDeserialize: serde::de::DeserializeOwned {
    fn from_postcard_bytes(bytes: &[u8]) -> Self {
        postcard::from_bytes(bytes).unwrap()
    }
}

impl<T: serde::Serialize> SerdePostcardSerialize for T {}
impl<T: serde::de::DeserializeOwned> SerdePostcardDeserialize for T {}
