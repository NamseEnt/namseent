#[cfg(target_family = "wasm")]
pub fn random(length: usize) -> Vec<u8> {
    let mut array = vec![0u8; length];
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut array).unwrap();
    array
}

const SAFE: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn nanoid() -> String {
    const SIZE: usize = 21;
    let mask = SAFE.len().next_power_of_two() - 1;
    let step: usize = 8 * SIZE / 5;

    let mut id = String::with_capacity(SIZE);

    loop {
        let bytes = random(step);

        for &byte in &bytes {
            let byte = byte as usize & mask;

            if SAFE.len() > byte {
                id.push(SAFE[byte]);

                if id.len() == SIZE {
                    return id;
                }
            }
        }
    }
}

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

pub fn random_id() -> Id {
    let mut array = [0u8; 64];
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut array).unwrap();
    Id { values: array }
}
