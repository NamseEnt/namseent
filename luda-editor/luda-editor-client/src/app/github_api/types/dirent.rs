use super::{Content, RequestBuilder};
use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub enum Dirent {
    File {
        path: String,
        name: String,
        sha: String,
        download_url: String,
    },
    Dir {
        path: String,
        name: String,
        sha: String,
    },
    Symlink {
        path: String,
        name: String,
        sha: String,
    },
    Submodule {
        path: String,
        name: String,
        sha: String,
    },
}

impl Dirent {
    pub async fn download(self) -> Result<Vec<u8>, DownloadError> {
        match self {
            Dirent::File { download_url, .. } => {
                let response = RequestBuilder::new(download_url).send().await;

                if !response.ok() {
                    return Err(DownloadError::ResponseNotOk(response.status()));
                }

                Ok(Uint8Array::new(
                    JsFuture::from(response.array_buffer().unwrap())
                        .await
                        .unwrap()
                        .as_ref(),
                )
                .to_vec())
            }
            Dirent::Dir { .. } => Err(DownloadError::NotFile),
            Dirent::Symlink { .. } => Err(DownloadError::NotFile),
            Dirent::Submodule { .. } => Err(DownloadError::NotFile),
        }
    }

    pub fn path(&self) -> &String {
        match self {
            Dirent::File { path, .. } => path,
            Dirent::Dir { path, .. } => path,
            Dirent::Symlink { path, .. } => path,
            Dirent::Submodule { path, .. } => path,
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Dirent::File { name, .. } => name,
            Dirent::Dir { name, .. } => name,
            Dirent::Symlink { name, .. } => name,
            Dirent::Submodule { name, .. } => name,
        }
    }

    pub fn sha(&self) -> &String {
        match self {
            Dirent::File { sha, .. } => sha,
            Dirent::Dir { sha, .. } => sha,
            Dirent::Symlink { sha, .. } => sha,
            Dirent::Submodule { sha, .. } => sha,
        }
    }
}

#[derive(Debug)]
pub enum DownloadError {
    ResponseNotOk(u16),
    NotFile,
}

impl From<Content> for Dirent {
    fn from(content: Content) -> Self {
        match content.r#type {
            super::Type::File => Dirent::File {
                path: content.path,
                name: content.name,
                sha: content.sha,
                download_url: content.download_url.unwrap(),
            },
            super::Type::Dir => Dirent::Dir {
                path: content.path,
                name: content.name,
                sha: content.sha,
            },
            super::Type::Symlink => Dirent::Symlink {
                path: content.path,
                name: content.name,
                sha: content.sha,
            },
            super::Type::Submodule => Dirent::Submodule {
                path: content.path,
                name: content.name,
                sha: content.sha,
            },
        }
    }
}
