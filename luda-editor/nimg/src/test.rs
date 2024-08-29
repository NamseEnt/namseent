// use crate::*;

// #[test]
// fn lossless_rgba_test() {
//     let rgba = vec![0, 0, 0, 0, 255, 255, 255, 255];

//     let encoded = encode(ColorFormat::R8g8b8a8, true, 2, 1, &rgba).unwrap();

//     let decoded = decode(&encoded).unwrap();

//     assert_eq!(decoded.width, 2);
//     assert_eq!(decoded.height, 1);
//     assert_eq!(decoded.color_format, ColorFormat::R8g8b8a8);
//     assert_eq!(decoded.pixels, rgba);
// }

// #[test]
// fn lossy_rgba_test() {
//     let rgba = [
//         [0, 0, 0, 0],
//         [255, 255, 255, 16],
//         [1, 1, 1, 25],
//         [16, 16, 16, 255],
//     ]
//     .concat();

//     let encoded = encode(ColorFormat::R8g8b8a8, false, 2, 2, &rgba).unwrap();

//     let decoded = decode(&encoded).unwrap();

//     assert_eq!(decoded.width, 2);
//     assert_eq!(decoded.height, 2);
//     assert_eq!(decoded.color_format, ColorFormat::R8g8b8a8);
//     assert_eq!(
//         decoded.pixels,
//         [
//             [0, 2, 2, 0,],
//             [255, 255, 255, 16],
//             [0, 0, 0, 25],
//             [12, 13, 13, 255]
//         ]
//         .concat()
//     );
// }

// #[test]
// fn lossless_r8_test() {
//     let r8 = vec![0, 255];

//     let encoded = encode(ColorFormat::R8, true, 2, 1, &r8).unwrap();

//     let decoded = decode(&encoded).unwrap();

//     assert_eq!(decoded.width, 2);
//     assert_eq!(decoded.height, 1);
//     assert_eq!(decoded.color_format, ColorFormat::R8);
//     assert_eq!(decoded.pixels, r8);
// }

// #[test]
// fn lossy_r8_test() {
//     let r8 = [0, 255, 1, 16].to_vec();

//     let encoded = encode(ColorFormat::R8, false, 2, 2, &r8).unwrap();

//     let decoded = decode(&encoded).unwrap();

//     assert_eq!(decoded.width, 2);
//     assert_eq!(decoded.height, 2);
//     assert_eq!(decoded.color_format, ColorFormat::R8);
//     assert_eq!(decoded.pixels, [0, 255, 0, 12]);
// }
