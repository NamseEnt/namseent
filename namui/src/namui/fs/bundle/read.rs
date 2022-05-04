use crate::{
    fetch_get_vec_u8,
    fs::{types::PathLike, util::create_url},
};
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadError {
    NetworkError(String),
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn read() -> Result<Vec<u8>, ReadError> {
    todo!()
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let url = create_url(path_like);
    fetch_get_vec_u8(url.as_str())
        .await
        .map_err(|fetch_error| ReadError::NetworkError(fetch_error.to_string()))
}
