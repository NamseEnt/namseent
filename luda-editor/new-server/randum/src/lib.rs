use rand::Rng;

/// random, 8 length of string in base62. it's about 47 bits.
pub fn rand() -> String {
    let mut rng = rand::thread_rng();
    let base62 = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let base62_len = base62.len();
    let mut result = String::with_capacity(8);
    for _ in 0..8 {
        let idx = rng.gen_range(0..base62_len);
        result.push(base62.chars().nth(idx).unwrap());
    }
    result
}
