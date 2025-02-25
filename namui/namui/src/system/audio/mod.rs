#[cfg(not(target_os = "wasi"))]
mod cpal;
mod ogg_async;
mod opus;
#[cfg(target_os = "wasi")]
mod wasi;

use super::InitResult;
use anyhow::{Result, anyhow, bail};
#[cfg(not(target_os = "wasi"))]
use cpal as inner;
use futures::TryStreamExt;
pub use inner::Audio;
pub use opus::encode_to_ogg_opus;
#[cfg(target_os = "wasi")]
use wasi as inner;

pub(super) async fn init() -> InitResult {
    inner::init().await?;
    Ok(())
}

type InterleavedAllSamples = Vec<Vec<f32>>;

impl Audio {
    pub fn from_ogg_opus_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let cursor = std::io::Cursor::new(bytes);
        let mut packet_reader = ogg::PacketReader::new(cursor);

        let identification_header_packet = packet_reader
            .read_packet()?
            .ok_or_else(|| anyhow!("Identification header packet expected"))?;
        if !identification_header_packet.data.starts_with(b"OpusHead") {
            bail!("OpusHead expected");
        }

        let comment_header_first_packet = packet_reader
            .read_packet()?
            .ok_or_else(|| anyhow!("Comment header first packet expected"))?;
        if !comment_header_first_packet.data.starts_with(b"OpusTags") {
            bail!("OpusTags expected");
        }

        let mut all_samples = Vec::with_capacity(1024);
        let mut decoder = opus::Decoder::new()?;

        while let Some(packet) = packet_reader.read_packet()? {
            let samples = decoder.decode_float(&packet.data)?;
            all_samples.push(samples);
        }

        Ok(Self::new(all_samples))
    }
    pub async fn from_ogg_opus_stream<Error>(
        stream: impl futures::TryStream<Ok = bytes::Bytes, Error = Error> + std::marker::Unpin,
    ) -> Result<Self>
    where
        Error: std::error::Error + Send + Sync + 'static,
    {
        let mut packet_reader = ogg_async::PacketReader::new(stream);

        let identification_header_packet = packet_reader
            .try_next()
            .await?
            .ok_or_else(|| anyhow!("Identification header packet expected"))?;
        if !identification_header_packet.data.starts_with(b"OpusHead") {
            bail!("OpusHead expected");
        }

        let comment_header_first_packet = packet_reader
            .try_next()
            .await?
            .ok_or_else(|| anyhow!("Comment header first packet expected"))?;
        if !comment_header_first_packet.data.starts_with(b"OpusTags") {
            bail!("OpusTags expected");
        }

        let mut all_samples = Vec::with_capacity(1024);
        let mut decoder = opus::Decoder::new()?;

        while let Some(packet) = packet_reader.try_next().await? {
            let samples = decoder.decode_float(&packet.data)?;
            all_samples.push(samples);
        }

        Ok(Self::new(all_samples))
    }
}

/// Volume will be clamped to 0.0 ~ 1.0 if it is out of range.
pub fn set_volume(zero_to_one: f32) {
    inner::set_volume(zero_to_one);
}

/// Volume value range is 0.0 ~ 1.0.
pub fn volume() -> f32 {
    inner::volume()
}
