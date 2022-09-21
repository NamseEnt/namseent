use crate::simple_error_impl;

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
    InvalidCharacter,
}
simple_error_impl!(IdTryFromStrError);
impl TryFrom<&str> for Id {
    type Error = IdTryFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 128 {
            return Err(IdTryFromStrError::InvalidLength);
        }
        let mut decimal_value = [0u8; 64];
        for index in 0..64 {
            decimal_value[index] = u8::from_str_radix(&value[index * 2..(index * 2 + 2)], 16)
                .map_err(|_| IdTryFromStrError::InvalidCharacter)?;
        }
        Ok(Id {
            values: decimal_value,
        })
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
    fn try_from_hexadecimal_str() {
        let hexadecimal = "65d340236a988c8299ff4df401b4b11b2d3d337cdbf446b4ac1e8f32f773cd07ca14805e1aba707efc55dde1c723eb7298870bb2567c14c7940ea039382b2b13";
        let expected = Id {
            values: [
                101, 211, 64, 35, 106, 152, 140, 130, 153, 255, 77, 244, 1, 180, 177, 27, 45, 61,
                51, 124, 219, 244, 70, 180, 172, 30, 143, 50, 247, 115, 205, 7, 202, 20, 128, 94,
                26, 186, 112, 126, 252, 85, 221, 225, 199, 35, 235, 114, 152, 135, 11, 178, 86,
                124, 20, 199, 148, 14, 160, 57, 56, 43, 43, 19,
            ],
        };
        let actual = Id::try_from(hexadecimal).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[wasm_bindgen_test]
    fn try_from_hexadecimal_str_should_fail() {
        // length is 127
        let short_hexadecimal = "65d340236a988c8299ff4df401b4b11b2d3d337cdbf446b4ac1e8f32f773cd07ca14805e1aba707efc55dde1c723eb7298870bb2567c14c7940ea039382b2b1";
        // length is 129
        let long_hexadecimal = "65d340236a988c8299ff4df401b4b11b2d3d337cdbf446b4ac1e8f32f773cd07ca14805e1aba707efc55dde1c723eb7298870bb2567c14c7940ea039382b2b133";
        // last 'g' is invalid
        let invalid_character_hexadecimal = "65d340236a988c8299ff4df401b4b11b2d3d337cdbf446b4ac1e8f32f773cd07ca14805e1aba707efc55dde1c723eb7298870bb2567c14c7940ea039382b2b1g";

        assert_eq!(
            Id::try_from(short_hexadecimal),
            Err(IdTryFromStrError::InvalidLength)
        );
        assert_eq!(
            Id::try_from(long_hexadecimal),
            Err(IdTryFromStrError::InvalidLength)
        );
        assert_eq!(
            Id::try_from(invalid_character_hexadecimal),
            Err(IdTryFromStrError::InvalidCharacter)
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn convert_to_str_and_revert() {
        let original = Id {
            values: [
                101, 211, 64, 35, 106, 152, 140, 130, 153, 255, 77, 244, 1, 180, 177, 27, 45, 61,
                51, 124, 219, 244, 70, 180, 172, 30, 143, 50, 247, 115, 205, 7, 202, 20, 128, 94,
                26, 186, 112, 126, 252, 85, 221, 225, 199, 35, 235, 114, 152, 135, 11, 178, 86,
                124, 20, 199, 148, 14, 160, 57, 56, 43, 43, 19,
            ],
        };
        let hexadecimal = format!("{}", original);
        let reverted = Id::try_from(hexadecimal.as_ref()).unwrap();
        assert_eq!(original, reverted);
    }
}
