#[cfg(test)]
mod test;

use anyhow::Result;
use jpegxl_rs::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    Rgba8888,
    R8,
}

/// # Parameters
/// - `lossless`: alpha channel is not lossy even if lossless is false
pub fn encode(
    color_format: ColorFormat,
    lossless: bool,
    width: usize,
    height: usize,
    bytes: &[u8],
) -> Result<Vec<u8>> {
    let parallel_runner = ThreadsRunner::default();

    let mut encoder = encoder_builder()
        .color_encoding(match color_format {
            ColorFormat::Rgba8888 => encode::ColorEncoding::Srgb,
            ColorFormat::R8 => encode::ColorEncoding::SrgbLuma,
        })
        .has_alpha(match color_format {
            ColorFormat::Rgba8888 => true,
            ColorFormat::R8 => false,
        })
        .lossless(lossless)
        .uses_original_profile(true)
        .parallel_runner(&parallel_runner)
        .build()?;

    let frame = encode::EncoderFrame::new(bytes).num_channels(match color_format {
        ColorFormat::Rgba8888 => 4,
        ColorFormat::R8 => 1,
    });

    let result = encoder.encode_frame::<_, u8>(&frame, width as u32, height as u32)?;

    Ok(result.data)
}

pub struct Decoded {
    pub width: usize,
    pub height: usize,
    pub color_format: ColorFormat,
    pub pixels: Vec<u8>,
}

pub fn decode(bytes: &[u8]) -> Result<Decoded> {
    let thread_runner = ThreadsRunner::default();
    let decoder = decoder_builder().parallel_runner(&thread_runner).build()?;

    let (metadata, pixels) = decoder.decode(bytes)?;

    Ok(Decoded {
        width: metadata.width as usize,
        height: metadata.height as usize,
        color_format: match metadata.has_alpha_channel {
            false => ColorFormat::R8,
            true => ColorFormat::Rgba8888,
        },
        pixels: if let decode::Pixels::Uint8(pixels) = pixels {
            pixels
        } else {
            return Err(anyhow::anyhow!("Expected u8 pixels"));
        },
    })
}
