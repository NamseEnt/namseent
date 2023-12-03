mod command;
mod docker_cli;
mod docker_engine;
mod envs;

use anyhow::Result;
use envs::*;

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

async fn real_main() -> Result<()> {
    println!("Environment variables: {:?}", std::env::vars());

    let docker_engine = docker_engine::DockerEngine::new()?;
    let mut running_image_digest_cache = None;

    if !DOCKER_LOGIN_SCRIPT.is_empty() {
        command::run(DOCKER_LOGIN_SCRIPT.as_str()).await?;
        println!("Docker Logged in.");
    }

    loop {
        let Some(image) = get_target_image().await? else {
            println!("No image found for group {}.", *GROUP_NAME);
            tokio::time::sleep(INTERVAL).await;
            continue;
        };

        docker_cli::pull_image(&image).await?;

        let target_image_digest = docker_engine.get_local_image_digest(&image).await?;

        let running_image_digest = {
            if running_image_digest_cache.is_none() {
                running_image_digest_cache = docker_engine
                    .get_running_container_image_digest(CONTAINER_NAME)
                    .await?;
            }
            &running_image_digest_cache
        };

        println!("Target image: {:?}", image);
        println!("Target image digest: {:?}", target_image_digest);
        println!("Running image digest: {:?}", running_image_digest);

        if let Some(running_image_digest) = running_image_digest {
            if running_image_digest == &target_image_digest {
                println!("Good! Image {} is already running.", image);
                tokio::time::sleep(INTERVAL).await;
                continue;
            }

            docker_engine.stop_running_container().await?;
        }

        docker_engine.run_new_container(&image).await?;

        running_image_digest_cache = Some(target_image_digest);

        docker_engine.docker_prune().await?;

        tokio::time::sleep(INTERVAL).await;
    }
}

async fn get_target_image() -> Result<Option<String>> {
    static AWS_SSM_CLIENT: tokio::sync::OnceCell<aws_sdk_ssm::Client> =
        tokio::sync::OnceCell::const_new();

    let aws_ssm_client = AWS_SSM_CLIENT
        .get_or_init(|| async {
            let config =
                aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
            aws_sdk_ssm::Client::new(&config)
        })
        .await;

    let parameter_path = format!("/oioi/{}/image", *GROUP_NAME);

    Ok(aws_ssm_client
        .get_parameter()
        .name(&parameter_path)
        .send()
        .await?
        .parameter
        .and_then(|p| p.value))
}
