use crate::envs::*;
use anyhow::Result;
use bollard::{container::ListContainersOptions, Docker};

pub(crate) struct DockerEngine {
    docker: Docker,
}

impl DockerEngine {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            docker: Docker::connect_with_local_defaults()?,
        })
    }

    pub(crate) async fn get_local_image_digest(&self, image: &str) -> Result<String> {
        let output = self.docker.inspect_image(image).await?;

        Ok(output.id.unwrap())
    }

    pub(crate) async fn get_running_container_image_digest(
        &self,
        container_name: &str,
    ) -> Result<Option<String>> {
        Ok(self
            .docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?
            .into_iter()
            .find_map(|container| {
                container
                    .names
                    .unwrap_or_default()
                    .contains(&format!("/{container_name}"))
                    .then(|| container.image_id.unwrap())
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

    pub(crate) async fn run_new_container(&self, image: &str) -> Result<()> {
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
                                    println!("mapping: {:?}", mapping);
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
