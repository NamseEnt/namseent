use crate::Encoder;

#[test]
fn lossless_test() {
    let rgb = vec![0, 0, 0, 255, 255, 255];

    let encoder = Encoder::new(true).unwrap();

    let _encoded = encoder.encode(2, 1, &rgb).unwrap();
}

#[test]
fn lossy_test() {
    let rgb = vec![0, 0, 0, 255, 255, 255];

    let encoder = Encoder::new(false).unwrap();

    let _encoded = encoder.encode(2, 1, &rgb).unwrap();
}
