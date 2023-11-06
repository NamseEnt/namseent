use super::types::{Dirent, DirentKind};
use namui_cfg::namui_cfg;
use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub enum ReadDirError {
    DirNotFound(String),
    ParseError(serde_json::Error),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn read_dir(path: &str) -> Result<Vec<Dirent>, ReadDirError> {
    let dirent_list_from_js_string =
        read_dir_from_electron(path)
            .await
            .and_then(|dirent_list_string| {
                Ok(dirent_list_string.as_string().unwrap_or(String::from("")))
            })?;
    let dirent_list_from_js =
        serde_json::from_str::<Vec<DirentFromJs>>(&dirent_list_from_js_string)?;
    let dirent_list = dirent_list_from_js
        .into_iter()
        .map(|dirent_from_js| dirent_from_js.into())
        .collect();
    Ok(dirent_list)
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = readDir)]
    async fn read_dir_from_electron(
        path: &str,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}

impl From<JsValue> for ReadDirError {
    fn from(error: JsValue) -> Self {
        let error: js_sys::Error = error.dyn_into().unwrap();
        let message = error.message();
        if message.starts_with("ENOENT", 0) {
            Self::DirNotFound(format!("{}", message))
        } else {
            Self::Other(format!("{}", message))
        }
    }
}
impl From<serde_json::Error> for ReadDirError {
    fn from(error: serde_json::Error) -> Self {
        ReadDirError::ParseError(error)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DirentFromJs {
    path: String,
    is_dir: bool,
}
impl From<DirentFromJs> for Dirent {
    fn from(val: DirentFromJs) -> Self {
        let url_string = format!("bundle:{}", val.path);
        Dirent::new(
            Url::parse(&url_string).unwrap_or_else(|_| panic!("fail to parse url: {}", url_string)),
            match val.is_dir {
                true => DirentKind::Directory,
                false => DirentKind::File,
            },
        )
    }
}
