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
