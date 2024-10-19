use base64::prelude::*;
use rkyv::{
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    Archive, Portable,
};

#[repr(u8)]
pub enum SqsMessageType {
    AudioTranscodingOk = 1,
    AudioTranscodingError = 2,
}

impl TryFrom<u8> for SqsMessageType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SqsMessageType::AudioTranscodingOk),
            2 => Ok(SqsMessageType::AudioTranscodingError),
            _ => Err(anyhow::anyhow!("Invalid message type")),
        }
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct AudioTranscodingOk {
    pub asset_id: String,
    pub before_size: usize,
    pub after_size: usize,
}
impl SqsMessage for AudioTranscodingOk {
    fn types(&self) -> SqsMessageType {
        SqsMessageType::AudioTranscodingOk
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct AudioTranscodingError {
    pub asset_id: String,
    pub ffmpeg_stderr: String,
}
impl SqsMessage for AudioTranscodingError {
    fn types(&self) -> SqsMessageType {
        SqsMessageType::AudioTranscodingError
    }
}

pub struct SqsMessageHandler<
    AudioTranscodingOkHandler: FnMut(ArchivedAudioTranscodingOk) -> AudioTranscodingOkFuture,
    AudioTranscodingOkFuture: std::future::Future<Output = anyhow::Result<()>>,
> {
    pub audio_transcoding_ok: AudioTranscodingOkHandler,
}

trait SqsMessage:
    rkyv::Archive
    + for<'a> rkyv::Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, rkyv::rancor::Error>>
{
    fn types(&self) -> SqsMessageType;
}

pub struct Client {
    inner: aws_sdk_sqs::Client,
    queue_url: String,
}

impl Client {
    pub fn new(sdk_config: &aws_types::SdkConfig, queue_url: String) -> Self {
        Self {
            inner: aws_sdk_sqs::Client::new(sdk_config),
            queue_url,
        }
    }
    #[allow(private_bounds)]
    pub async fn send(&self, message: impl SqsMessage) -> anyhow::Result<()> {
        let type_id = message.types() as u8;
        let type_id_hex_padding = format!("{:02x}", type_id);

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&message)?;
        let base64 = BASE64_STANDARD.encode(bytes);

        let message_body = type_id_hex_padding + &base64;
        self.inner
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(message_body)
            .send()
            .await?;
        Ok(())
    }
    pub async fn recv<Fut>(&self, mut handler: impl FnMut(u8, Vec<u8>) -> Fut) -> anyhow::Result<()>
    where
        Fut: std::future::Future<Output = anyhow::Result<()>>,
    {
        let output = self
            .inner
            .receive_message()
            .queue_url(&self.queue_url)
            .wait_time_seconds(20)
            .send()
            .await?;

        for message in output.messages() {
            let body = message
                .body
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No message body"))?;

            if body.len() < 2 {
                return Err(anyhow::anyhow!("Invalid message body"));
            }

            let type_id = u8::from_str_radix(&body[0..2], 16)?;
            let base64 = &body[2..];
            let bytes = BASE64_STANDARD.decode(base64)?;

            handler(type_id, bytes).await?;

            self.inner
                .delete_message()
                .queue_url(&self.queue_url)
                .receipt_handle(message.receipt_handle.as_ref().unwrap())
                .send()
                .await?;
        }

        Ok(())
    }
}

pub fn access_message<T: Archive>(bytes: &[u8]) -> Result<&T::Archived, rkyv::rancor::Error>
where
    T::Archived: Portable + for<'a> CheckBytes<HighValidator<'a, rkyv::rancor::Error>>,
{
    rkyv::access::<T::Archived, rkyv::rancor::Error>(bytes)
}
