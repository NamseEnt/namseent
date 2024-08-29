#[cfg(test)]
mod test;

use anyhow::Result;

const ZSTD_ENCODE_LEVEL: i32 = 9;
const JPEG_QUALITY: u8 = 80;

#[derive(Debug, Clone)]

pub struct Nimg {
    pub color_type: ColorType,
    encoded: Encoded,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Rgba8888,
    A8,
}

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
enum Encoded {
    Rgba8888Zstd { rgba_zstd: Vec<u8> },
    Rgb888JpegA8Zstd { rgb_jpeg: Vec<u8>, a_zstd: Vec<u8> },
    A8Zstd { a_zstd: Vec<u8> },
}

impl std::fmt::Debug for Encoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoded::Rgba8888Zstd { rgba_zstd } => f
                .debug_struct("Rgba8888Zstd")
                .field("rgba", &rgba_zstd.len())
                .finish(),
            Encoded::Rgb888JpegA8Zstd { rgb_jpeg, a_zstd } => f
                .debug_struct("Rgb888JpegA8Zstd")
                .field("rgb", &rgb_jpeg.len())
                .field("a", &a_zstd.len())
                .finish(),
            Encoded::A8Zstd { a_zstd } => {
                f.debug_struct("A8Zstd").field("a", &a_zstd.len()).finish()
            }
        }
    }
}

pub fn encode_rgba8888(width: usize, height: usize, pixels: &[u8]) -> Result<Nimg> {
    assert_eq!(pixels.len(), width * height * 4);

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
        Ok(Nimg {
            color_type: ColorType::Rgba8888,
            encoded: Encoded::Rgb888JpegA8Zstd { rgb_jpeg, a_zstd },
        })
    } else {
        Ok(Nimg {
            color_type: ColorType::Rgba8888,
            encoded: Encoded::Rgba8888Zstd { rgba_zstd },
        })
    }
}

pub fn encode_a8(data: &[u8]) -> Result<Nimg> {
    Ok(Nimg {
        color_type: ColorType::A8,
        encoded: Encoded::A8Zstd {
            a_zstd: zstd::encode_all(data, ZSTD_ENCODE_LEVEL)?,
        },
    })
}

impl Nimg {
    pub fn decode(&self) -> Result<Vec<u8>> {
        let Self { encoded, .. } = self;
        match encoded {
            Encoded::Rgba8888Zstd { rgba_zstd } => Ok(zstd::decode_all(rgba_zstd.as_slice())?),
            Encoded::Rgb888JpegA8Zstd { rgb_jpeg, a_zstd } => {
                let mut rgba = {
                    let mut decoder = zune_jpeg::JpegDecoder::new(rgb_jpeg.as_slice());

                    decoder.set_options(
                        zune_jpeg::zune_core::options::DecoderOptions::default()
                            .jpeg_set_out_colorspace(
                                zune_jpeg::zune_core::colorspace::ColorSpace::RGBA,
                            ),
                    );

                    decoder.decode()?
                };
                let a = zstd::decode_all(a_zstd.as_slice())?;

                rgba.chunks_exact_mut(4)
                    .zip(a.iter())
                    .for_each(|(rgba, a)| rgba[3] = *a);

                Ok(rgba)
            }
            Encoded::A8Zstd { a_zstd } => Ok(zstd::decode_all(a_zstd.as_slice())?),
        }
    }

    pub fn image_encoded_byte_size(&self) -> usize {
        let Self { encoded, .. } = self;
        match encoded {
            Encoded::Rgba8888Zstd { rgba_zstd } => rgba_zstd.len(),
            Encoded::Rgb888JpegA8Zstd { rgb_jpeg, a_zstd } => rgb_jpeg.len() + a_zstd.len(),
            Encoded::A8Zstd { a_zstd } => a_zstd.len(),
        }
    }

    pub fn image_encoded_bytes(&self) -> Vec<Vec<u8>> {
        let Self { encoded, .. } = self;
        match encoded {
            Encoded::Rgba8888Zstd { rgba_zstd } => vec![rgba_zstd.clone()],
            Encoded::Rgb888JpegA8Zstd { rgb_jpeg, a_zstd } => {
                vec![rgb_jpeg.clone(), a_zstd.clone()]
            }
            Encoded::A8Zstd { a_zstd } => vec![a_zstd.clone()],
        }
    }
}
