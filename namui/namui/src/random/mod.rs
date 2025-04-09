pub use namui_type::uuid;
use rand::RngCore;

pub fn random(length: usize) -> Vec<u8> {
    let mut array = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut array);
    array
}

pub fn rand_random<T>() -> T
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    rand::random()
}
