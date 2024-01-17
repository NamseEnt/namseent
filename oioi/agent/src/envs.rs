pub(crate) const INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
pub(crate) const CONTAINER_NAME: &str = "oioi";
pub(crate) const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: i64 = 30;

lazy_static::lazy_static! {
    pub(crate) static ref GROUP_NAME: String = std::env::var("GROUP_NAME").expect("GROUP_NAME env var not set");
    pub(crate) static ref EC2_INSTANCE_ID: String = std::env::var("EC2_INSTANCE_ID").expect("EC2_INSTANCE_ID env var not set");
}
