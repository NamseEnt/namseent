mod apis;
mod documents;
mod handle;
mod session;
mod storage;
mod utils;

use anyhow::Result;
use aws_smithy_async::rt::sleep::default_async_sleep;
use hyper::{
    service::{make_service_fn, service_fn},
    Server, Uri,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use once_cell::sync::OnceCell;
use std::io::Write;
use std::net::SocketAddr;
use storage::{dynamo_db::DynamoDb, s3::S3};

#[derive(Debug)]
struct Storage {
    dynamo_db: DynamoDb,
    s3: S3,
}

static STORAGES: OnceCell<Storage> = OnceCell::new();

pub fn dynamo_db<'a>() -> &'a DynamoDb {
    &STORAGES.get().unwrap().dynamo_db
}

pub fn s3<'a>() -> &'a S3 {
    &STORAGES.get().unwrap().s3
}

pub async fn init() {
    let server_path = {
        let item = env!("CARGO_MANIFEST_DIR").split("/").collect::<Vec<_>>();
        item[..item.len() - 1].join("/") + "/"
    };

    env_logger::builder()
        .format(move |buf, record| {
            let full_file_path = record.file().unwrap_or("unknown-file");

            let short_file_path = full_file_path
                .strip_prefix(&server_path)
                .unwrap_or(full_file_path);

            if short_file_path.contains("aws-smithy-http-tower") {
                return Ok(());
            }

            writeln!(
                buf,
                "[{}] {} {}:{} - {}",
                buf.default_level_style(record.level())
                    .value(record.level()),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                short_file_path,
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .write_style(env_logger::WriteStyle::Auto)
        .init();

    log::info!("starting up");

    if is_running_on_lambda() {
        log::info!("running on lambda");
        let config = aws_config::load_from_env().await;
        let dynamo_db = DynamoDb::new(&config);
        let s3 = S3::new(
            &config,
            format!(
                "https://s3.{region}.amazonaws.com",
                region = config.region().unwrap()
            ),
        );

        STORAGES.set(Storage { dynamo_db, s3 }).unwrap();
    } else {
        log::info!("not running on lambda");
        set_local_storage();
    }
}

pub(crate) fn set_local_storage() {
    let dynamo_db = DynamoDb::new(
        &aws_config::SdkConfig::builder()
            .endpoint_resolver(aws_sdk_dynamodb::Endpoint::immutable(Uri::from_static(
                "http://localhost:8000",
            )))
            .region(aws_sdk_dynamodb::Region::new("ap-northeast-2"))
            .credentials_provider(aws_types::credentials::SharedCredentialsProvider::new(
                aws_types::Credentials::new("local", "local", None, None, "local"),
            ))
            .sleep_impl(default_async_sleep().unwrap())
            .build(),
    );
    let s3 = S3::new(
        &aws_config::SdkConfig::builder()
            .endpoint_resolver(aws_sdk_dynamodb::Endpoint::immutable(Uri::from_static(
                "http://localhost:9000",
            )))
            .region(aws_sdk_dynamodb::Region::new("ap-northeast-2"))
            .credentials_provider(aws_types::credentials::SharedCredentialsProvider::new(
                aws_types::Credentials::new("minio", "minio123", None, None, "local"),
            ))
            .sleep_impl(default_async_sleep().unwrap())
            .build(),
        "http://localhost:9000".to_string(),
    );

    STORAGES.set(Storage { dynamo_db, s3 }).unwrap();
}

pub async fn run_server() -> Result<(), LambdaError> {
    if is_running_on_lambda() {
        let svc = service_fn(handle::handle_with_wrapped_error);
        run_hyper_on_lambda(svc).await?;
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, LambdaError>(service_fn(handle::handle_with_wrapped_error))
        });
        let server = Server::bind(&addr).serve(make_svc);

        server.await?;
    }

    Ok(())
}
