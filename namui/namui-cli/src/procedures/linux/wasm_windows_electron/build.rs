use crate::services::{
    electron_build_service,
    electron_package_service::{Arch, Platform},
};
use crate::*;
use std::path::Path;

pub async fn build(manifest_path: &Path, arch: Option<Arch>, release: bool) -> Result<()> {
    electron_build_service::build(manifest_path, arch, Platform::Win32, release).await
}
