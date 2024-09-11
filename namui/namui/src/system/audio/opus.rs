use anyhow::{bail, Result};
use opusic_sys::*;
use std::ffi::{c_int, CStr};

pub struct Decoder {
    decoder: *mut OpusDecoder,
}

impl Decoder {
    pub fn new() -> Result<Self> {
        let decoder = unsafe {
            let mut error: c_int = 0;
            let decoder = opus_decoder_create(48000, 2, &mut error);
            if error != 0 {
                bail!("{}", CStr::from_ptr(opus_strerror(error)).to_str().unwrap());
            }
            decoder
        };

        Ok(Self { decoder })
    }
    pub fn decode_float(&mut self, packet: &[u8]) -> Result<Vec<f32>> {
        let number_of_samples = self.get_nb_samples(packet)?;
        let mut samples = vec![0.0; number_of_samples * 2];
        unsafe {
            let error = opus_decode_float(
                self.decoder,
                packet.as_ptr(),
                packet.len() as c_int,
                samples.as_mut_ptr(),
                number_of_samples as c_int,
                0,
            );
            if error < 0 {
                bail!("{}", CStr::from_ptr(opus_strerror(error)).to_str().unwrap());
            }
        }
        Ok(samples)
    }
    fn get_nb_samples(&mut self, packet: &[u8]) -> Result<usize> {
        unsafe {
            let number_of_samples =
                opus_decoder_get_nb_samples(self.decoder, packet.as_ptr(), packet.len() as c_int);
            if number_of_samples < 0 {
                bail!(
                    "{}",
                    CStr::from_ptr(opus_strerror(number_of_samples))
                        .to_str()
                        .unwrap()
                );
            }
            Ok(number_of_samples as usize)
        }
    }
}
