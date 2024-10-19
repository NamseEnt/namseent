use aws_sdk_s3::primitives::ByteStream;
use futures::TryStreamExt;
use std::{
    process::Stdio,
    sync::{atomic::AtomicUsize, Arc},
};
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    bootstrap().await.unwrap();
}

async fn bootstrap() -> anyhow::Result<()> {
    let aws_lambda_runtime_api = std::env::var("AWS_LAMBDA_RUNTIME_API")?;

    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&sdk_config);

    let queue_url = std::env::var("QUEUE_URL")?;
    let sqs_client = sqs_message::Client::new(&sdk_config, queue_url);

    loop {
        let invoke_next_response = reqwest::get(format!(
            "http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/next",
        ))
        .await?;

        let request_id = invoke_next_response
            .headers()
            .get("Lambda-Runtime-Aws-Request-Id")
            .ok_or_else(|| anyhow::anyhow!("No request id"))?
            .to_str()?
            .to_string();

        let event_data = invoke_next_response.text().await?;

        let response = handler(&event_data, &s3_client, &sqs_client).await?;

        reqwest::Client::new()
            .post(format!(
                "http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/{request_id}/response",
            ))
            .body(response)
            .send()
            .await?;
    }
}

async fn handler(
    event_data: &str,
    s3_client: &aws_sdk_s3::Client,
    sqs_client: &sqs_message::Client,
) -> anyhow::Result<String> {
    println!("event_data: {}", event_data);

    let bucket_name = std::env::var("BUCKET_NAME")?;

    let input: Input = serde_json::from_str(event_data)?;
    let key = &input.records[0].s3.object.key;
    let asset_id = std::path::Path::new(key)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("No filename"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

    let upload_key = format!("audio/after-transcode/{asset_id}");

    let get_object_output = s3_client
        .get_object()
        .bucket(&bucket_name)
        .key(key)
        .send()
        .await?;

    let before_size = get_object_output
        .content_length
        .ok_or_else(|| anyhow::anyhow!("No content length"))? as usize;

    let mut ffmpeg = tokio::process::Command::new("./ffmpeg")
        .args("-hide_banner -loglevel error -i pipe:0 -f opus -acodec libopus pipe:1".split(' '))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut ffmpeg_stdin = ffmpeg.stdin.take().unwrap();
    let ffmpeg_stdout = ffmpeg.stdout.take().unwrap();
    let mut ffmpeg_stderr = ffmpeg.stderr.take().unwrap();

    let ffmpeg_stderr_task = tokio::spawn(async move {
        let mut ffmpeg_stderr_string = String::new();
        ffmpeg_stderr
            .read_to_string(&mut ffmpeg_stderr_string)
            .await?;
        anyhow::Ok(ffmpeg_stderr_string)
    });

    let s3_to_ffmpeg_task = tokio::spawn(async move {
        tokio::io::copy(
            &mut get_object_output.body.into_async_read(),
            &mut ffmpeg_stdin,
        )
        .await?;
        Ok::<_, anyhow::Error>(())
    });

    let after_size = Arc::new(AtomicUsize::new(0));
    let body =
        reqwest::Body::wrap_stream(tokio_util::io::ReaderStream::new(ffmpeg_stdout).map_ok({
            let after_size = after_size.clone();
            move |chunk| {
                after_size.fetch_add(chunk.len(), std::sync::atomic::Ordering::Relaxed);
                chunk
            }
        }));

    let putting_result = s3_client
        .put_object()
        .bucket(&bucket_name)
        .key(&upload_key)
        .body(ByteStream::from_body_1_x(body))
        .send()
        .await;

    let getting_result = s3_to_ffmpeg_task.await?;

    let ffmpeg_exit_status = ffmpeg.wait().await?;

    if !ffmpeg_exit_status.success() {
        let mut ffmpeg_stderr = ffmpeg_stderr_task.await??;

        // 120K length to fit in SQS message
        ffmpeg_stderr.truncate(120 * 1024);

        sqs_client
            .send(sqs_message::AudioTranscodingError {
                asset_id: asset_id.to_string(),
                ffmpeg_stderr,
            })
            .await?;

        return Ok("Failed on ffmpeg".to_string());
    }

    getting_result?;
    putting_result?;

    let after_size = after_size.load(std::sync::atomic::Ordering::Relaxed);

    // NOTE: Removing the original object will be implemented in server side when receive AudioTranscodingOk message.

    sqs_client
        .send(sqs_message::AudioTranscodingOk {
            asset_id: asset_id.to_string(),
            after_size,
            before_size,
        })
        .await?;

    Ok("Success".to_string())
}

#[derive(serde::Deserialize)]
struct Input {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}
#[derive(serde::Deserialize)]
struct Record {
    #[serde(rename = "s3")]
    s3: S3,
}
#[derive(serde::Deserialize)]
struct S3 {
    #[serde(rename = "object")]
    object: Object,
}
#[derive(serde::Deserialize)]
struct Object {
    #[serde(rename = "key")]
    key: String,
}
