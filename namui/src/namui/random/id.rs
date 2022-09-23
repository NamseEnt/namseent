use crate::simple_error_impl;
use base64::decode_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id {
    values: [u8; 64],
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
            64 => {
                let mut values = [0; 64];
                values.copy_from_slice(&bytes);
                Ok(Id { values })
            }
            _ => Err(IdTryFromStrError::InvalidLength),
        }
    }
}

pub fn random_id() -> Id {
    let mut array = [0u8; 64];
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
        let base64_str = "ZdNAI2qYjIKZ/030AbSxGy09M3zb9Ea0rB6PMvdzzQfKFIBeGrpwfvxV3eHHI+tymIcLslZ8FMeUDqA5OCsrEw";
        let expected = Id {
            values: [
                101, 211, 64, 35, 106, 152, 140, 130, 153, 255, 77, 244, 1, 180, 177, 27, 45, 61,
                51, 124, 219, 244, 70, 180, 172, 30, 143, 50, 247, 115, 205, 7, 202, 20, 128, 94,
                26, 186, 112, 126, 252, 85, 221, 225, 199, 35, 235, 114, 152, 135, 11, 178, 86,
                124, 20, 199, 148, 14, 160, 57, 56, 43, 43, 19,
            ],
        };
        let actual = Id::try_from(base64_str).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[wasm_bindgen_test]
    fn try_from_base64_str_should_fail() {
        let short_base64_str =
            "ZdNAI2qYjIKZ/030AbSxGy09M3zb9Ea0rB6PMvdzzQfKFIBeGrpwfvxV3eHHI+tymIcLslZ8FMeUDqA5OCsr";
        let long_base64_str = "ZdNAI2qYjIKZ/030AbSxGy09M3zb9Ea0rB6PMvdzzQfKFIBeGrpwfvxV3eHHI+tymIcLslZ8FMeUDqA5OCsrEwE";
        let invalid_character_base64_str = "ZdNAI2qYjIKZ/030AbSxGy09M3zb9Ea0rB6PMvdzzQfKFIBeGrpwfvxV3eHHI+tymIcLslZ8FMeUDqA5OCsrEw_";

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
}
