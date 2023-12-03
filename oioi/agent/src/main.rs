mod command;
mod container_config;
mod docker_cli;
mod docker_engine;
mod envs;

use crate::{container_config::ContainerConfig, docker_engine::OioiContainer};
use anyhow::Result;
use envs::*;

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

async fn real_main() -> Result<()> {
    println!("Environment variables: {:?}", std::env::vars());

    let mut docker_engine = docker_engine::DockerEngine::new()?;
    let mut last_updated_at = None;

    loop {
        let Some(ContainerConfig {
            image_uri,
            port_mappings,
            docker_login_script,
            updated_at,
        }) = container_config::get_container_config().await?
        else {
            println!("No container config found for group {}.", *GROUP_NAME);
            tokio::time::sleep(INTERVAL).await;
            continue;
        };

        if last_updated_at == Some(updated_at) {
            println!("No update.");
            tokio::time::sleep(INTERVAL).await;
            continue;
        }

        last_updated_at = Some(updated_at);

        if !docker_login_script.is_empty() {
            command::run(docker_login_script.as_str()).await?;
            println!("Docker Logged in.");
        }

        docker_engine
            .update_container(OioiContainer {
                image_uri,
                port_mappings,
            })
            .await?;

        docker_engine.docker_prune().await?;

        tokio::time::sleep(INTERVAL).await;
    }
}
