use serde::Deserialize;

#[derive(Deserialize)]
pub struct Content {
    pub r#type: Type,
    pub encoding: Option<String>,
    pub size: u32,
    pub name: String,
    pub path: String,
    pub content: Option<String>,
    pub sha: String,
    pub url: String,
    pub git_url: Option<String>,
    pub html_url: Option<String>,
    pub download_url: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    File,
    Dir,
    Symlink,
    Submodule,
}

#[derive(Deserialize)]
pub struct Links {
    pub git: Option<String>,
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub html: String,
}
