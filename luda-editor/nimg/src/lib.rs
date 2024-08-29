#[cfg(test)]
mod test;

use anyhow::Result;

#[derive(Debug, Clone)]

pub struct Nimg {
    pub color_type: ColorType,
    encoded: Encoded,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    RgbA8888,
    A8,
}

#[derive(Debug, Clone)]
enum Encoded {
    Rgb8A8 { rgb: Vec<u8>, a: Vec<u8> },
    A8 { a: Vec<u8> },
}

/// alpha channel ignored
pub fn encode_rgba8(width: usize, height: usize, pixels: &[u8]) -> Result<Nimg> {
    let rgb = {
        let mut output = Vec::with_capacity(width * height * 3);

        jpeg_encoder::Encoder::new(&mut output, 80).encode(
            pixels,
            width as u16,
            height as u16,
            jpeg_encoder::ColorType::Rgba,
        )?;

        output
    };

    let a = {
        let a_pixels = pixels
            .chunks_exact(4)
            .map(|rgba| rgba[3])
            .collect::<Vec<_>>();

        encode_a8_impl(width, height, &a_pixels)?
    };

    Ok(Nimg {
        color_type: ColorType::RgbA8888,
        encoded: Encoded::Rgb8A8 { rgb, a },
    })
}

pub fn encode_a8(width: usize, height: usize, data: &[u8]) -> Result<Nimg> {
    Ok(Nimg {
        color_type: ColorType::A8,
        encoded: Encoded::A8 {
            a: encode_a8_impl(width, height, data)?,
        },
    })
}

fn encode_a8_impl(width: usize, height: usize, data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = jpegxl_rs::encoder_builder()
        .color_encoding(jpegxl_rs::encode::ColorEncoding::SrgbLuma)
        .has_alpha(false)
        .lossless(true)
        .uses_original_profile(true)
        .speed(jpegxl_rs::encode::EncoderSpeed::Lightning)
        .decoding_speed(4)
        .build()?;

    let frame = jpegxl_rs::encode::EncoderFrame::new(data).num_channels(1);

    Ok(encoder
        .encode_frame::<u8, u8>(&frame, width as _, height as _)?
        .data)
}

impl Nimg {
    pub fn decode(&self) -> Result<Vec<u8>> {
        let Self { encoded, .. } = self;
        match encoded {
            Encoded::Rgb8A8 { rgb, a } => {
                let a_pixels = decode_a8(a)?;

                let mut rgba = {
                    use zune_jpeg::zune_core::*;
                    let mut decoder = zune_jpeg::JpegDecoder::new(rgb);
                    decoder.set_options(
                        options::DecoderOptions::default()
                            .jpeg_set_out_colorspace(colorspace::ColorSpace::RGBA),
                    );

                    decoder.decode()?
                };

                rgba.chunks_exact_mut(4)
                    .zip(a_pixels.iter())
                    .for_each(|(rgba, a)| rgba[3] = *a);

                Ok(rgba)
            }
            Encoded::A8 { a } => Ok(decode_a8(a)?),
        }
    }
}

fn decode_a8(data: &[u8]) -> Result<Vec<u8>> {
    let (_, pixels) = jpegxl_rs::decoder_builder().build()?.decode(data)?;

    if let jpegxl_rs::decode::Pixels::Uint8(pixels) = pixels {
        Ok(pixels)
    } else {
        unreachable!()
    }
}
