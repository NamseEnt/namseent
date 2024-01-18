use crate::envs::GROUP_NAME;
use anyhow::Result;
use chrono::DateTime;

pub(crate) async fn get_container_config() -> Result<Option<ContainerConfig>> {
    static AWS_SSM_CLIENT: tokio::sync::OnceCell<aws_sdk_ssm::Client> =
        tokio::sync::OnceCell::const_new();

    let aws_ssm_client = AWS_SSM_CLIENT
        .get_or_init(|| async {
            let config =
                aws_config::load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
            aws_sdk_ssm::Client::new(&config)
        })
        .await;

    let parameter_path = format!("/oioi/{}/container-config", *GROUP_NAME);

    let Some(parameter_value) = aws_ssm_client
        .get_parameter()
        .name(&parameter_path)
        .send()
        .await?
        .parameter
        .and_then(|p| p.value)
    else {
        return Ok(None);
    };

    let container_config = serde_json::from_str(&parameter_value)?;

    Ok(Some(container_config))
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ContainerConfig {
    #[serde(rename = "imageUri")]
    pub image_uri: String,
    #[serde(rename = "portMappings")]
    pub port_mappings: Vec<PortMapping>,
    #[serde(rename = "dockerLoginScript")]
    pub docker_login_script: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<chrono::offset::Utc>,
    #[serde(rename = "env")]
    pub env: std::collections::HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub(crate) struct PortMapping {
    #[serde(rename = "containerPort")]
    pub(crate) container_port: u16,
    #[serde(rename = "hostPort")]
    pub(crate) host_port: u16,
    #[serde(rename = "protocol")]
    pub(crate) protocol: String,
}
