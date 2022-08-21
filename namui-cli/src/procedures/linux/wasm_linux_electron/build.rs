use crate::services::{
    electron_build_service,
    electron_package_service::{Arch, Platform},
};
use std::path::Path;

pub fn build(manifest_path: &Path, arch: Option<Arch>) -> Result<(), crate::Error> {
    electron_build_service::build(manifest_path, arch, Platform::Linux)
}
