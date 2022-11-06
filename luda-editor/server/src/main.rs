mod handle;
mod services;
mod session;
pub mod storage;
mod utils;

use aws_smithy_async::rt::sleep::default_async_sleep;
use handle::handle_with_wrapped_error;
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use once_cell::sync::OnceCell;
use rpc::hyper::{
    service::{make_service_fn, service_fn},
    Server, Uri,
};
use std::net::SocketAddr;
use storage::{dynamo_db::DynamoDb, s3::S3};

#[derive(Debug)]
struct Services {
    auth_service: services::auth::AuthService,
    image_service: services::image::ImageService,
    sequence_service: services::sequence::SequenceService,
    project_service: services::project::ProjectService,
}

static SERVICES: OnceCell<Services> = OnceCell::new();
fn services() -> &'static Services {
    SERVICES.get().unwrap()
}

#[derive(Debug)]
struct Storage {
    dynamo_db: DynamoDb,
    s3: S3,
}

static STORAGES: OnceCell<Storage> = OnceCell::new();

fn dynamo_db<'a>() -> &'a DynamoDb {
    &STORAGES.get().unwrap().dynamo_db
}

fn s3<'a>() -> &'a S3 {
    &STORAGES.get().unwrap().s3
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    env_logger::init();
    log::info!("starting up");

    SERVICES
        .set(Services {
            auth_service: services::auth::AuthService::new(),
            sequence_service: services::sequence::SequenceService::new(),
            project_service: services::project::ProjectService::new(),
            image_service: services::image::ImageService::new(),
        })
        .unwrap();

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

        let svc = service_fn(handle_with_wrapped_error);
        run_hyper_on_lambda(svc).await?;
    } else {
        log::info!("not running on lambda");
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

        let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, LambdaError>(service_fn(handle_with_wrapped_error))
        });
        let server = Server::bind(&addr).serve(make_svc);

        server.await?;
    }
    Ok(())
}
