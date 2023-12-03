pub(crate) const INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
pub(crate) const CONTAINER_NAME: &str = "oioi";
pub(crate) const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: i64 = 30;

lazy_static::lazy_static! {
    pub(crate) static ref GROUP_NAME: String = std::env::var("GROUP_NAME").expect("GROUP_NAME env var not set");
    pub(crate) static ref EC2_INSTANCE_ID: String = std::env::var("EC2_INSTANCE_ID").expect("EC2_INSTANCE_ID env var not set");
    pub(crate) static ref PORT_MAPPINGS: Vec<PortMapping> = std::env::var("PORT_MAPPINGS").map(|env_string| {
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
    pub(crate) static ref DOCKER_LOGIN_SCRIPT: String = std::env::var("DOCKER_LOGIN_SCRIPT").expect("DOCKER_LOGIN_SCRIPT env var not set");
}

#[derive(Debug)]
pub(crate) struct PortMapping {
    pub(crate) container_port: u16,
    pub(crate) host_port: u16,
    pub(crate) protocol: String,
}
