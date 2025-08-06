use aws_sdk_s3::primitives::ByteStream;
use std::process::Stdio;

#[tokio::main]
async fn main() {
    bootstrap().await.unwrap();
}

async fn bootstrap() -> anyhow::Result<()> {
    let aws_lambda_runtime_api = std::env::var("AWS_LAMBDA_RUNTIME_API")?;

    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::Config::new(&sdk_config).to_builder().build();
    let s3_client = aws_sdk_s3::Client::from_conf(config);

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

        let response = handler(&event_data, &s3_client).await?;

        reqwest::Client::new()
            .post(format!(
                "http://{aws_lambda_runtime_api}/2018-06-01/runtime/invocation/{request_id}/response",
            ))
            .body(response)
            .send()
            .await?;
    }
}

async fn handler(event_data: &str, s3_client: &aws_sdk_s3::Client) -> anyhow::Result<String> {
    println!("event_data: {event_data}");

    let bucket_name = std::env::var("BUCKET_NAME")?;

    let input: Input = serde_json::from_str(event_data)?;
    let key = &input.records[0].s3.object.key;
    let filename = std::path::Path::new(key)
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("No filename"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

    let upload_key = format!("audio/after-transcode/{filename}");

    let get_object_output = s3_client
        .get_object()
        .bucket(&bucket_name)
        .key(key)
        .send()
        .await?;

    let ffmpeg = tokio::process::Command::new("./ffmpeg")
        .args(["-i", "pipe:0", "-f", "opus", "-acodec", "libopus", "pipe:1"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut ffmpeg_stdin = ffmpeg.stdin.unwrap();
    let ffmpeg_stdout = ffmpeg.stdout.unwrap();

    let s3_to_ffmpeg_task = tokio::spawn(async move {
        tokio::io::copy(
            &mut get_object_output.body.into_async_read(),
            &mut ffmpeg_stdin,
        )
        .await?;
        Ok::<_, anyhow::Error>(())
    });

    let body = reqwest::Body::wrap_stream(tokio_util::io::ReaderStream::new(ffmpeg_stdout));

    s3_client
        .put_object()
        .bucket(&bucket_name)
        .key(&upload_key)
        .body(ByteStream::from_body_1_x(body))
        .send()
        .await?;

    s3_to_ffmpeg_task.await??;

    Ok("{\"statusCode\": 200, \"body\": \"Key $OBJECT_KEY transcoded to opus\"}".to_string())
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
