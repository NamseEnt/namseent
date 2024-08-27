mod data;
mod error;
#[cfg(test)]
mod test;

pub use data::*;
pub use error::*;
use libavif_sys::*;

pub struct Encoder {
    encoder: *mut avifEncoder,
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            avifEncoderDestroy(self.encoder);
        }
    }
}

fn max_threads() -> std::io::Result<usize> {
    Ok(std::thread::available_parallelism()?.get())
}

impl Encoder {
    pub fn new(lossless: bool) -> std::io::Result<Self> {
        let encoder = unsafe { avifEncoderCreate() };

        unsafe {
            let mut encoder = *encoder;
            encoder.maxThreads = max_threads()? as i32;
            encoder.minQuantizer = if lossless {
                AVIF_QUANTIZER_LOSSLESS as i32
            } else {
                10
            };
            encoder.maxQuantizer = if lossless {
                AVIF_QUANTIZER_LOSSLESS as i32
            } else {
                40
            };
            encoder.minQuantizerAlpha = AVIF_QUANTIZER_LOSSLESS as i32;
            encoder.maxQuantizerAlpha = AVIF_QUANTIZER_LOSSLESS as i32;
            encoder.speed = 6;

            encoder.codecChoice = AVIF_CODEC_CHOICE_RAV1E;
        };

        // AVIF_PIXEL_FORMAT_YUV420

        Ok(Self { encoder })
    }

    /// Supported input pixel formats are:
    ///   - R8
    ///   - RGB888
    ///   - RGBA8888
    pub fn encode(&self, width: usize, height: usize, data: &[u8]) -> Result<EncodedData, Error> {
        enum PixelFormat {
            R8,
            RGB888,
            RGBA8888,
        }

        let pixel_format = if width * height == data.len() {
            PixelFormat::R8
        } else if width * height * 3 == data.len() {
            PixelFormat::RGB888
        } else if width * height * 4 == data.len() {
            PixelFormat::RGBA8888
        } else {
            return Err(Error::UnsupportedImageType);
        };

        unsafe {
            let avif_image = match pixel_format {
                PixelFormat::R8 => {
                    let avif_image =
                        avifImageCreate(width as u32, height as u32, 8, AVIF_PIXEL_FORMAT_YUV400);
                    avifImageAllocatePlanes(avif_image, AVIF_PLANES_YUV);
                    std::ptr::copy_nonoverlapping(
                        data.as_ptr(),
                        (*avif_image).yuvPlanes[0],
                        data.len(),
                    );
                    avif_image
                }
                PixelFormat::RGB888 | PixelFormat::RGBA8888 => {
                    let no_alpha = matches!(pixel_format, PixelFormat::RGB888);
                    let avif_rgb_image = avifRGBImage {
                        width: width as u32,
                        height: height as u32,
                        depth: 8,
                        format: if no_alpha {
                            AVIF_RGB_FORMAT_RGB
                        } else {
                            AVIF_RGB_FORMAT_RGBA
                        },
                        chromaUpsampling: AVIF_CHROMA_UPSAMPLING_AUTOMATIC,
                        chromaDownsampling: AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC,
                        avoidLibYUV: AVIF_FALSE as i32,
                        ignoreAlpha: if no_alpha { AVIF_TRUE } else { AVIF_FALSE } as i32,
                        alphaPremultiplied: AVIF_FALSE as i32,
                        isFloat: AVIF_FALSE as i32,
                        maxThreads: max_threads()? as i32,
                        pixels: data.as_ptr() as *mut u8,
                        rowBytes: width as u32 * if no_alpha { 3 } else { 4 },
                    };

                    let avif_image =
                        avifImageCreate(width as u32, height as u32, 8, AVIF_PIXEL_FORMAT_YUV420);

                    let result = avifImageRGBToYUV(avif_image, &avif_rgb_image);
                    if result != AVIF_RESULT_OK {
                        return Err(Error::Code(result));
                    }

                    avif_image
                }
            };

            let mut output = avifRWData::default();
            let result = avifEncoderWrite(self.encoder, avif_image, &mut output);
            if result != AVIF_RESULT_OK {
                return Err(Error::Code(result));
            }

            avifImageDestroy(avif_image);

            Ok(EncodedData::new(output))
        }
    }
}
