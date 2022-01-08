// Copyright 2013 The Flutter Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file (https://github.com/flutter/engine/blob/main/LICENSE).

// Code from https://github.com/flutter/engine/blob/e59d312/lib/web_ui/lib/src/engine/canvaskit/image_web_codecs.dart
// Thank you yjbanov!

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn decodeImageFromBuffer(data: &[u8], content_type: &str) -> Result<JsValue, JsValue>;
}

pub(crate) async fn decode_image(data: &[u8]) -> Result<JsValue, String> {
    // this does not detect image type automatically. It requires us to
    // tell it what the image type is.
    let content_type = detect_content_type(data);

    if content_type.is_none() {
        let file_header = match data.get(0..10.min(data.len())) {
            Some(header) => format!("[{:?}]", header),
            None => "empty".to_string(),
        };
        return Err(format!(
            "Failed to detect image file format using the file header.\nFile header was {}.\n",
            file_header
        ));
    }
    let content_type = content_type.unwrap();
    match decodeImageFromBuffer(data, &content_type).await {
        Ok(image) => Ok(image),
        Err(error) => Err(js_sys::JSON::stringify(&error).unwrap().into()),
    }
}

const IMAGE_FILE_HEADER_TUPLES: &'static [(&'static [u8], &str)] = &[
    // PNG
    (
        &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
        "image/png",
    ),
    // GIF87a
    (&[0x47, 0x49, 0x46, 0x38, 0x37, 0x61], "image/gif"),
    // GIF89a
    (&[0x47, 0x49, 0x46, 0x38, 0x39, 0x61], "image/gif"),
    // JPEG
    (&[0xFF, 0xD8, 0xFF, 0xDB], "image/jpeg"),
    (
        &[
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        ],
        "image/jpeg",
    ),
    (&[0xFF, 0xD8, 0xFF, 0xEE], "image/jpeg"),
    (&[0xFF, 0xD8, 0xFF, 0xE1], "image/jpeg"),
    // WebP
    (
        &[
            0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50,
        ],
        "image/webp",
    ),
    // BMP
    (&[0x42, 0x4D], "image/bmp"),
];

/// Detects the image file format and returns the corresponding "Content-Type"
/// value (a.k.a. MIME type).
///
/// The returned value can be passed to `ImageDecoder` when decoding an image.
///
/// Returns null if [data] cannot be mapped to a known content type.
fn detect_content_type(data: &[u8]) -> Option<&'static str> {
    for (header, content_type) in IMAGE_FILE_HEADER_TUPLES.iter() {
        if header.len() > data.len() {
            continue;
        }
        if header.iter().zip(data.iter()).all(|(a, b)| a == b) {
            return Some(content_type);
        }
    }
    None
}
