use crate::{container_config::PortMapping, docker_cli, envs::*};
use anyhow::Result;
use bollard::{Docker, container::ListContainersOptions};

pub(crate) struct DockerEngine {
    docker: Docker,
}

impl DockerEngine {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
        })
    }

    async fn get_local_image_digest(&self, image: &str) -> Result<String> {
        let output = self.docker.inspect_image(image).await?;

        Ok(output.id.unwrap())
    }

    pub(crate) async fn get_running_container(&self) -> Result<Option<OioiContainerConfig>> {
        Ok(self
            .docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?
            .into_iter()
            .find(|container| {
                container
                    .names
                    .as_ref()
                    .map(|names| names.contains(&format!("/{CONTAINER_NAME}")))
                    .unwrap_or(false)
            })
            .map(|container| OioiContainerConfig {
                image_uri: container.image.unwrap(),
                image_digest: container.image_id.unwrap(),
                port_mappings: container
                    .ports
                    .unwrap_or_default()
                    .into_iter()
                    .map(|port| PortMapping {
                        container_port: port.private_port,
                        host_port: port.public_port.unwrap_or(port.private_port),
                        protocol: match port.typ.unwrap() {
                            bollard::service::PortTypeEnum::EMPTY => unreachable!(),
                            bollard::service::PortTypeEnum::TCP => "tcp".to_string(),
                            bollard::service::PortTypeEnum::UDP => "udp".to_string(),
                            bollard::service::PortTypeEnum::SCTP => "sctp".to_string(),
                        },
                    })
                    .collect(),
            }))
    }

    pub(crate) async fn stop_running_container(&self) -> Result<()> {
        self.docker
            .stop_container(
                CONTAINER_NAME,
                Some(bollard::container::StopContainerOptions {
                    t: GRACEFUL_SHUTDOWN_TIMEOUT_SECS,
                }),
            )
            .await?;
        Ok(())
    }

    pub(crate) async fn update_container(
        &mut self,
        OioiContainer {
            image_uri,
            port_mappings,
            env,
        }: OioiContainer,
    ) -> Result<()> {
        // pull image to get most recent remote image digest
        docker_cli::pull_image(&image_uri).await?;
        let target_container_config = OioiContainerConfig {
            image_digest: self.get_local_image_digest(&image_uri).await?,
            image_uri,
            port_mappings,
        };

        if let Some(running_container) = self.get_running_container().await? {
            if running_container == target_container_config {
                println!("No update.");
                tokio::time::sleep(INTERVAL).await;
                return Ok(());
            }

            self.stop_running_container().await?;
        }

        println!("removing container {CONTAINER_NAME}");
        let remove_container_result = self
            .docker
            .remove_container(
                CONTAINER_NAME,
                Some(bollard::container::RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await;

        if let Err(err) = remove_container_result {
            match err {
                bollard::errors::Error::DockerResponseServerError {
                    status_code,
                    message,
                } => if status_code == 404 && message.contains("No such container") {},
                _ => return Err(err.into()),
            }
        };

        println!("creating container {CONTAINER_NAME}");
        self.docker
            .create_container(
                Some(bollard::container::CreateContainerOptions {
                    name: CONTAINER_NAME,
                    platform: None,
                }),
                bollard::container::Config {
                    image: Some(target_container_config.image_uri.to_string()),
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
                            target_container_config
                                .port_mappings
                                .iter()
                                .map(|mapping| {
                                    println!("mapping: {mapping:?}");
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
                    env: Some(env.into_iter().map(|(k, v)| format!("{k}={v}")).collect()),
                    ..Default::default()
                },
            )
            .await?;

        println!("starting container {CONTAINER_NAME}");
        self.docker
            .start_container(
                CONTAINER_NAME,
                None::<bollard::container::StartContainerOptions<String>>,
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn docker_prune(&self) -> Result<()> {
        self.docker
            .prune_containers(None::<bollard::container::PruneContainersOptions<String>>)
            .await?;

        self.docker
            .prune_images(None::<bollard::image::PruneImagesOptions<String>>)
            .await?;

        self.docker
            .prune_networks(None::<bollard::network::PruneNetworksOptions<String>>)
            .await?;

        self.docker
            .prune_volumes(None::<bollard::volume::PruneVolumesOptions<String>>)
            .await?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct OioiContainer {
    pub(crate) image_uri: String,
    pub(crate) port_mappings: Vec<PortMapping>,
    pub(crate) env: std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct OioiContainerConfig {
    pub(crate) image_uri: String,
    pub(crate) image_digest: String,
    pub(crate) port_mappings: Vec<PortMapping>,
}
