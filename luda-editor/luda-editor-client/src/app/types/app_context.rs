use super::{CameraAngleImageLoader, MetaContainer};
use crate::app::storage::GithubStorage;
use std::sync::Arc;

pub struct AppContext {
    pub storage: Arc<dyn GithubStorage>,
    pub meta_container: Arc<MetaContainer>,
    pub camera_angle_image_loader: Arc<dyn CameraAngleImageLoader>,
}
