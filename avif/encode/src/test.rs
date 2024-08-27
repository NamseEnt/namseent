use crate::Encoder;

#[test]
fn lossless_rgb_test() {
    let rgb = vec![0, 0, 0, 255, 255, 255];

    let encoder = Encoder::new(true).unwrap();

    let _encoded = encoder.encode(2, 1, &rgb).unwrap();
}

#[test]
fn lossy_rgb_test() {
    let rgb = vec![0, 0, 0, 255, 255, 255];

    let encoder = Encoder::new(false).unwrap();

    let _encoded = encoder.encode(2, 1, &rgb).unwrap();
}

#[test]
fn lossless_r8_test() {
    let r8 = vec![0, 255];

    let encoder = Encoder::new(true).unwrap();

    let _encoded = encoder.encode(2, 1, &r8).unwrap();
}

#[test]
fn lossy_r8_test() {
    let r8 = vec![0, 255];

    let encoder = Encoder::new(false).unwrap();

    let _encoded = encoder.encode(2, 1, &r8).unwrap();
}

#[test]
fn lossless_rgba_test() {
    let rgba = vec![0, 0, 0, 255, 255, 255, 255, 255];

    let encoder = Encoder::new(true).unwrap();

    let _encoded = encoder.encode(2, 1, &rgba).unwrap();
}

#[test]

fn lossy_rgba_test() {
    let rgba = vec![0, 0, 0, 255, 255, 255, 255, 255];

    let encoder = Encoder::new(false).unwrap();

    let _encoded = encoder.encode(2, 1, &rgba).unwrap();
}
