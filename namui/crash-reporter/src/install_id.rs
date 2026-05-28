use crate::{Error, queue};
use uuid::Uuid;

pub fn get_or_create(app_name: &str) -> Result<String, Error> {
    let root = queue::root_dir(app_name)?;
    std::fs::create_dir_all(&root)?;
    let path = root.join("install_id");
    if let Ok(bytes) = std::fs::read(&path) {
        let s = String::from_utf8_lossy(&bytes).trim().to_string();
        if !s.is_empty() {
            return Ok(s);
        }
    }
    let id = Uuid::new_v4().to_string();
    std::fs::write(&path, id.as_bytes())?;
    Ok(id)
}
