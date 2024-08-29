#[cfg(test)]
mod test;

use anyhow::Result;

pub fn encode_rgb8(width: usize, height: usize, data: &[u8]) -> Result<Vec<u8>> {
    let mut output = vec![];

    jpeg_encoder::Encoder::new(&mut output, 80).encode(
        data,
        width as u16,
        height as u16,
        jpeg_encoder::ColorType::Rgb,
    )?;

    Ok(output)
}

pub fn decode_rgb8(data: &[u8]) -> Result<Vec<u8>> {
    Ok(zune_jpeg::JpegDecoder::new(data).decode()?)
}

pub fn encode_a8(width: usize, height: usize, data: &[u8]) -> Result<Vec<u8>> {
    let mut output = vec![];

    image_webp::WebPEncoder::new(&mut output).encode(
        data,
        width as u32,
        height as u32,
        image_webp::ColorType::L8,
    )?;

    Ok(output)
}

pub fn decode_a8(data: &[u8]) -> Result<Vec<u8>> {
    let mut output = vec![];

    let mut decoder = image_webp::WebPDecoder::new(std::io::Cursor::new(data))?;
    decoder.read_image(&mut output)?;

    Ok(output)
}
