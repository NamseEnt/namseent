use crate::simple_error_impl;
use base64::{decode_config, encode_config};
use serde::{Deserialize, Serialize};

const ID_BYTE_LENGTH: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct Id {
    values: [u8; ID_BYTE_LENGTH],
}
impl<'a> PartialEq<Id> for &'a Id {
    fn eq(&self, other: &Id) -> bool {
        self.values == other.values
    }
}
impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print in hexadecimal
        for i in 0..self.values.len() {
            write!(f, "{:02x}", self.values[i])?;
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum IdTryFromStrError {
    InvalidLength,
    Base64DecodeError(base64::DecodeError),
}
simple_error_impl!(IdTryFromStrError);
impl TryFrom<&str> for Id {
    type Error = IdTryFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = decode_config(value, base64::STANDARD_NO_PAD)
            .map_err(|error| IdTryFromStrError::Base64DecodeError(error))?;
        match bytes.len() {
            ID_BYTE_LENGTH => {
                let mut values = [0; ID_BYTE_LENGTH];
                values.copy_from_slice(&bytes);
                Ok(Id { values })
            }
            _ => Err(IdTryFromStrError::InvalidLength),
        }
    }
}
impl Into<String> for Id {
    fn into(self) -> String {
        encode_config(self.values, base64::STANDARD_NO_PAD)
    }
}

pub fn random_id() -> Id {
    let mut array = [0u8; ID_BYTE_LENGTH];
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut array).unwrap();
    Id { values: array }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn try_from_base64_str() {
        let base64_str = "ZdNAI2qYjIKZ/030AbSxGw";
        let expected = Id {
            values: [
                101, 211, 64, 35, 106, 152, 140, 130, 153, 255, 77, 244, 1, 180, 177, 27,
            ],
        };
        let actual = Id::try_from(base64_str).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[wasm_bindgen_test]
    fn try_from_base64_str_should_fail() {
        let short_base64_str = "ZdNAI2qYjIKZ/030AbSx";
        let long_base64_str = "ZdNAI2qYjIKZ/030AbSxGy0";
        let invalid_character_base64_str = "ZdNAI2qYjIKZ/030AbSxGw_";

        assert_eq!(
            Id::try_from(short_base64_str),
            Err(IdTryFromStrError::InvalidLength)
        );
        assert_eq!(
            Id::try_from(long_base64_str),
            Err(IdTryFromStrError::InvalidLength)
        );
        assert!(match Id::try_from(invalid_character_base64_str) {
            Err(IdTryFromStrError::Base64DecodeError(_)) => true,
            _ => false,
        });
    }

    #[test]
    #[wasm_bindgen_test]
    fn serde_should_work() {
        let original = Id {
            values: [
                101, 211, 64, 35, 106, 152, 140, 130, 153, 255, 77, 244, 1, 180, 177, 27,
            ],
        };
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
