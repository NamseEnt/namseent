#[cfg(test)]
mod test;

use anyhow::*;

const ZSTD_ENCODE_LEVEL: i32 = 9;
const JPEG_QUALITY: u8 = 80;

pub fn encode(
    color_type: ColorType,
    width: usize,
    height: usize,
    pixels: &[u8],
) -> Result<Vec<u8>> {
    assert_eq!(
        pixels.len(),
        width
            * height
            * match color_type {
                ColorType::Rgba8888 => 4,
                ColorType::A8 => 1,
            }
    );

    let (format_type, body) = match color_type {
        ColorType::Rgba8888 => {
            let rgb_jpeg = {
                let mut output = Vec::with_capacity(width * height * 3);

                jpeg_encoder::Encoder::new(&mut output, JPEG_QUALITY).encode(
                    pixels,
                    width as u16,
                    height as u16,
                    jpeg_encoder::ColorType::Rgba,
                )?;

                output
            };

            let a_zstd = {
                let a_pixels = pixels
                    .chunks_exact(4)
                    .map(|rgba| rgba[3])
                    .collect::<Vec<_>>();

                zstd::encode_all(a_pixels.as_slice(), ZSTD_ENCODE_LEVEL)?
            };

            let rgba_zstd = zstd::encode_all(pixels, ZSTD_ENCODE_LEVEL)?;

            if rgb_jpeg.len() + a_zstd.len() < rgba_zstd.len() {
                let mut output = Vec::with_capacity(4 + rgb_jpeg.len() + a_zstd.len());
                output.extend_from_slice(&(rgb_jpeg.len() as u32).to_le_bytes());
                output.extend(rgb_jpeg);
                output.extend(a_zstd);

                (FormatType::Rgb888JpegA8Zstd, output)
            } else {
                (FormatType::Rgba8888Zstd, rgba_zstd)
            }
        }
        ColorType::A8 => {
            let a_zstd = zstd::encode_all(pixels, ZSTD_ENCODE_LEVEL)?;
            (FormatType::A8Zstd, a_zstd)
        }
    };

    let header = Header {
        format_type,
        body_length: body.len() as u32,
        color_type,
        width: width as u32,
        height: height as u32,
    };
    let header_bytes = bincode::serialize(&header)?;

    let mut output = Vec::with_capacity(header_bytes.len() + body.len());
    output.extend(header_bytes);
    output.extend(body);

    Ok(output)
}

pub fn decode(data: &[u8]) -> Result<(Header, Vec<u8>)> {
    let header: Header = bincode::deserialize(data)?;
    let header_size = data.len() - header.body_length as usize;
    let body = header.parse_body(&data[header_size..])?;
    Ok((header, body))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Header {
    format_type: FormatType,
    body_length: u32,
    pub color_type: ColorType,
    pub width: u32,
    pub height: u32,
}

impl Header {
    fn parse_body(&self, data: &[u8]) -> Result<Vec<u8>> {
        assert_eq!(data.len(), self.body_length as usize);

        match self.format_type {
            FormatType::Rgba8888Zstd => Ok(zstd::decode_all(data)?),
            FormatType::Rgb888JpegA8Zstd => {
                let rgb_jpeg_length =
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
                let rgb_jpeg = &data[4..4 + rgb_jpeg_length];
                let a_zstd = &data[4 + rgb_jpeg_length..];

                let mut rgba = {
                    use zune_jpeg::{
                        JpegDecoder,
                        zune_core::{colorspace::ColorSpace, options::DecoderOptions},
                    };
                    let mut decoder = JpegDecoder::new(rgb_jpeg);
                    decoder.set_options(
                        DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGBA),
                    );
                    decoder.decode()?
                };

                let a = zstd::decode_all(a_zstd)?;

                rgba.chunks_exact_mut(4)
                    .zip(a.iter())
                    .for_each(|(rgba, a)| rgba[3] = *a);

                Ok(rgba)
            }
            FormatType::A8Zstd => Ok(zstd::decode_all(data)?),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum ColorType {
    Rgba8888,
    A8,
}

impl TryFrom<u8> for ColorType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorType::Rgba8888),
            1 => Ok(ColorType::A8),
            value => Err(anyhow!("unknown color type: {}", value)),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
#[allow(clippy::enum_variant_names)]
enum FormatType {
    /// Body
    /// - rgba_zstd: [u8; N]
    Rgba8888Zstd = 0,
    /// Body
    /// - rgb_jpeg_length: u32le
    /// - rgb_jpeg: [u8; rgb_jpeg_length]
    /// - a_zstd: [u8; N - rgb_jpeg_length]
    Rgb888JpegA8Zstd = 1,
    /// Body
    /// - a: [u8; N]
    A8Zstd = 2,
}

impl TryFrom<u8> for FormatType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FormatType::Rgba8888Zstd),
            1 => Ok(FormatType::Rgb888JpegA8Zstd),
            2 => Ok(FormatType::A8Zstd),
            value => Err(anyhow!("unknown format type: {}", value)),
        }
    }
}
