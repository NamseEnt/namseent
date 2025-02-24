use anyhow::{Result, bail};
use opusic_sys::*;
use std::{
    borrow::Cow,
    ffi::{CStr, c_int},
};

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

pub fn encode_to_ogg_opus<Channel>(pcms: impl AsRef<[Channel]>) -> Result<Vec<u8>>
where
    Channel: AsRef<[f32]>,
{
    let pcms = pcms.as_ref();
    let channel_count = pcms.len();

    let interleaved_samples = if channel_count == 1 {
        Cow::Borrowed(pcms[0].as_ref())
    } else if channel_count == 2 {
        pcms[0]
            .as_ref()
            .iter()
            .zip(pcms[1].as_ref())
            .flat_map(|(left, right)| vec![*left, *right])
            .collect()
    } else {
        bail!("Only mono or stereo is supported");
    };
    let mut interleaved_samples = interleaved_samples.as_ref();

    let mut ogg_output = Vec::new();
    let mut writer = ogg::PacketWriter::new(&mut ogg_output);
    const SERIAL: u32 = 12345;

    unsafe {
        let mut error: i32 = 0;
        let encoder = opus_encoder_create(
            48000,
            channel_count as i32,
            OPUS_APPLICATION_AUDIO,
            &mut error,
        );
        if error != 0 {
            bail!("{}", CStr::from_ptr(opus_strerror(error)).to_str().unwrap());
        }

        let mut lookahead: opus_int32 = 0;
        let error = opus_encoder_ctl(encoder, OPUS_GET_LOOKAHEAD_REQUEST, &mut lookahead) as u16;
        if error != 0 {
            bail!(
                "{}",
                CStr::from_ptr(opus_strerror(error as i32))
                    .to_str()
                    .unwrap()
            );
        }

        {
            // https://wiki.xiph.org/OggOpus#ID_Header
            //  0                   1                   2                   3
            //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            // |       'O'     |      'p'      |     'u'       |     's'       |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            // |       'H'     |       'e'     |     'a'       |     'd'       |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            // |  version = 1  | channel count |           pre-skip            |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            // |                original input sample rate in Hz               |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            // |    output gain Q7.8 in dB     |  channel map  |               |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+               :
            // |                                                               |
            // :          optional channel mapping table...                    :
            // |                                                               |
            // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
            let mut head = Vec::with_capacity(19);
            head.extend("OpusHead".bytes());
            head.push(1);
            head.push(channel_count as u8);
            head.extend((lookahead as u16).to_le_bytes());
            head.extend(48000u32.to_le_bytes());
            head.extend(0u16.to_le_bytes()); // Output gain
            head.push(0);

            assert_eq!(head.len(), 19);

            writer.write_packet(head, SERIAL, ogg::PacketWriteEndInfo::EndPage, 0)?;
        }

        {
            let mut opus_tags: Vec<u8> = Vec::with_capacity(60);
            opus_tags.extend(b"OpusTags");

            let vendor_str = "namui-ogg-opus";
            opus_tags.extend(&(vendor_str.len() as u32).to_le_bytes());
            opus_tags.extend(vendor_str.bytes());

            opus_tags.extend(&[0u8; 4]); // No user comments

            writer.write_packet(opus_tags, SERIAL, ogg::PacketWriteEndInfo::EndPage, 0)?;
        }

        const MAX_FRAME_SIZE: usize = 2880;
        const MIN_FRAME_SIZE: usize = 120;

        let mut sample_count = 0;

        while !interleaved_samples.is_empty() {
            let frame_size = if interleaved_samples.len() > MAX_FRAME_SIZE * channel_count {
                MAX_FRAME_SIZE
            } else {
                MIN_FRAME_SIZE
            };
            let pcm_len = frame_size * channel_count;

            let mut output_buffer: Vec<u8> = vec![0; 8192];

            let pcm = {
                if interleaved_samples.len() > pcm_len {
                    Cow::Borrowed(&interleaved_samples[..pcm_len])
                } else {
                    let mut frame = Vec::with_capacity(pcm_len);
                    frame.extend_from_slice(interleaved_samples);
                    frame.extend(vec![0.0; pcm_len - interleaved_samples.len()]);
                    Cow::Owned(frame)
                }
            };
            assert_eq!(pcm_len, pcm.len());

            let is_end = pcm_len >= interleaved_samples.len();

            let output_len = opus_encode_float(
                encoder,
                pcm.as_ptr(),
                frame_size as c_int,
                output_buffer.as_mut_ptr(),
                output_buffer.len() as c_int,
            );
            if output_len < 0 {
                let error = output_len;
                bail!("{}", CStr::from_ptr(opus_strerror(error)).to_str().unwrap());
            }
            let output_len = output_len as usize;

            output_buffer.truncate(output_len);

            sample_count += frame_size;

            // https://wiki.xiph.org/OggOpus#Granule_Position
            let granule_position = lookahead as usize + sample_count;

            writer.write_packet(
                output_buffer,
                SERIAL,
                if is_end {
                    ogg::PacketWriteEndInfo::EndStream
                } else {
                    ogg::PacketWriteEndInfo::NormalPacket
                },
                granule_position as u64,
            )?;

            interleaved_samples = &interleaved_samples[(pcm_len.min(interleaved_samples.len()))..];
        }
    }

    Ok(ogg_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_stereo_to_ogg_opus() {
        let pcms = vec![vec![0.0; 48000]; 2];
        let result = encode_to_ogg_opus(&pcms);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_mono_to_ogg_opus() {
        let pcms = vec![vec![0.0; 48000]];
        let result = encode_to_ogg_opus(&pcms);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_stereo_to_ogg_opus_odd_numbers_samples() {
        let pcms = vec![vec![0.0; 123456]; 2];
        let result = encode_to_ogg_opus(&pcms);
        assert!(result.is_ok());
    }
}
