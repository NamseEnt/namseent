use anyhow::Result;
use bollard::{container::ListContainersOptions, Docker};
use futures_util::{stream::stream::StreamExt, StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

const INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
const CONTAINER_NAME: &str = "oioi";
const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: i64 = 30;
lazy_static::lazy_static! {
    static ref GROUP_NAME: String = std::env::var("GROUP_NAME").expect("GROUP_NAME env var not set");
    static ref EC2_INSTANCE_ID: String = std::env::var("EC2_INSTANCE_ID").expect("EC2_INSTANCE_ID env var not set");
    static ref PORT_MAPPINGS: Vec<PortMapping> = std::env::var("PORT_MAPPINGS").map(|env_string| {
        env_string.split(',').map(|mapping| {
            let mut parts = mapping.split(&[':', '/']);
            let container_port = parts.next().expect("container port not found").parse::<u16>().expect("container port is not a number");
            let host_port = parts.next().expect("host port not found").parse::<u16>().expect("host port is not a number");
            let protocol = parts.next().expect("protocol not found").to_string();

            PortMapping {
                container_port,
                host_port,
                protocol,
            }
        }).collect()
    }).expect("PORT_MAPPINGS env var not set");
}

async fn real_main() -> Result<()> {
    println!("Environment variables: {:?}", std::env::vars());
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
    let aws_ssm_client = aws_sdk_ssm::Client::new(&config);

    let docker = Docker::connect_with_local_defaults()?;

    loop {
        let Some(image) = get_image(&aws_ssm_client).await? else {
            println!("No image found for group {}.", *GROUP_NAME);
            tokio::time::sleep(INTERVAL).await;
            continue;
        };

        let running_image = get_running_image(&docker).await?;

        if let Some(running_image) = running_image {
            if running_image == image {
                println!("Good! Image {} is already running.", image);
                tokio::time::sleep(INTERVAL).await;
                continue;
            }
            stop_running_container(&docker).await?;
        }

        run_new_container(&docker, &image).await?;

        docker_prune(&docker).await?;

        tokio::time::sleep(INTERVAL).await;
    }
}

async fn get_image(aws_ssm_client: &aws_sdk_ssm::Client) -> Result<Option<String>> {
    let parameter_path = format!("/oioi/{}/image", *GROUP_NAME);

    let image = aws_ssm_client
        .get_parameter()
        .name(&parameter_path)
        .send()
        .await?
        .parameter
        .and_then(|p| p.value);

    Ok(image)
}

async fn get_running_image(docker: &Docker) -> Result<Option<String>> {
    let containers = docker
        .list_containers(None::<ListContainersOptions<String>>)
        .await?;

    for container in containers {
        let Some(names) = container.names else {
            continue;
        };

        if !names.contains(&CONTAINER_NAME.to_string()) {
            continue;
        }

        let Some(image) = container.image else {
            anyhow::bail!("Container image should not be empty");
        };

        return Ok(Some(image));
    }

    Ok(None)
}

async fn stop_running_container(docker: &Docker) -> Result<()> {
    docker
        .stop_container(
            CONTAINER_NAME,
            Some(bollard::container::StopContainerOptions {
                t: GRACEFUL_SHUTDOWN_TIMEOUT_SECS,
            }),
        )
        .await?;

    docker
        .remove_container(
            CONTAINER_NAME,
            Some(bollard::container::RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}

async fn run_new_container(docker: &Docker, image: &str) -> Result<()> {
    while let Some(create_image_info) = docker
        .create_image(
            Some(bollard::image::CreateImageOptions {
                from_image: image,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_next()
        .await?
    {
        println!("Pull image: {:?}", create_image_info);
    }

    docker
        .create_container(
            Some(bollard::container::CreateContainerOptions {
                name: CONTAINER_NAME,
                platform: None,
            }),
            bollard::container::Config {
                image: Some(image.to_string()),
                host_config: Some(bollard::models::HostConfig {
                    log_config: Some(bollard::models::HostConfigLogConfig {
                        typ: Some("awslogs".to_string()),
                        config: Some(std::collections::HashMap::from_iter([
                            ("awslogs-group".to_string(), format!("oioi-{}", *GROUP_NAME)),
                            (
                                "awslogs-stream".to_string(),
                                format!("oioi-{}-{}", *GROUP_NAME, *EC2_INSTANCE_ID),
                            ),
                            ("awslogs-create-group".to_string(), "true".to_string()),
                        ])),
                    }),
                    port_bindings: Some(std::collections::HashMap::from_iter(
                        PORT_MAPPINGS
                            .iter()
                            .map(|mapping| {
                                (
                                    format!("{}/{}", mapping.container_port, mapping.protocol),
                                    Some(vec![bollard::models::PortBinding {
                                        host_ip: None,
                                        host_port: Some(mapping.host_port.to_string()),
                                    }]),
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

    docker
        .start_container(
            CONTAINER_NAME,
            None::<bollard::container::StartContainerOptions<String>>,
        )
        .await?;

    Ok(())
}

async fn docker_prune(docker: &Docker) -> Result<()> {
    docker
        .prune_containers(None::<bollard::container::PruneContainersOptions<String>>)
        .await?;

    docker
        .prune_images(None::<bollard::image::PruneImagesOptions<String>>)
        .await?;

    docker
        .prune_networks(None::<bollard::network::PruneNetworksOptions<String>>)
        .await?;

    docker
        .prune_volumes(None::<bollard::volume::PruneVolumesOptions<String>>)
        .await?;

    Ok(())
}

struct PortMapping {
    container_port: u16,
    host_port: u16,
    protocol: String,
}
