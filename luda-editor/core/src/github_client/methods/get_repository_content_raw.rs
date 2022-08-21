use super::*;
use crate::github_client::GithubClientError;
use namui::network::http::Response;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

impl GithubClient {
    #![cfg_attr(test, allow(dead_code))]
    pub async fn get_repository_content_raw(
        &self,
        branch: &str,
        path: &str,
    ) -> Result<Box<[u8]>, GithubClientError> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url(),
            self.owner(),
            self.repo(),
            path,
            branch,
        );
        let cache_key = format!("get_repository_content_raw::{}", url);

        let cached = namui::cache::get_serde::<ETagCacheValue>(&cache_key)
            .await
            .map_err(|error| GithubClientError::CacheError(error))?;

        let response =
            namui::network::http::fetch(&url, namui::network::http::Method::GET, |builder| {
                (if let Some(cached) = &cached {
                    builder.header("If-None-Match", cached.e_tag.as_str())
                } else {
                    builder
                })
                .header("Authorization", format!("token {}", self.access_token()))
                .header("Accept", "application/vnd.github.v3.raw")
                .header("User-Agent", "luda-editor")
                .header("Cache-Control", "max-age=0")
                .fetch_credentials_include()
            })
            .await?;

        if response.status().as_u16() == 304 {
            return Ok(cached.unwrap().raw);
        }

        let response = response.error_for_400599().await?;

        let (e_tag, last_modified) = get_e_tag_last_modified(&response);

        let response_raw = response.bytes().await?.as_ref().into();

        put_cache_if_needed(&cache_key, last_modified, e_tag, &response_raw).await?;

        Ok(response_raw)
    }
}

async fn put_cache_if_needed(
    cache_key: &str,
    last_modified: chrono::DateTime<chrono::FixedOffset>,
    e_tag: String,
    raw: &Box<[u8]>,
) -> Result<(), GithubClientError> {
    static CACHE_MUTEX: OnceCell<Mutex<()>> = OnceCell::new();
    let _ = CACHE_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();

    let cached = namui::cache::get_serde::<ETagCacheValue>(&cache_key)
        .await
        .map_err(|error| GithubClientError::CacheError(error))?;

    let should_put_new_cache = {
        if let Some(cached) = cached {
            cached.last_modified < last_modified
        } else {
            true
        }
    };
    if should_put_new_cache {
        namui::cache::set_serde(
            &cache_key,
            &ETagCacheValue {
                e_tag,
                last_modified,
                raw: raw.clone(),
            },
        )
        .await
        .map_err(|error| GithubClientError::CacheError(error))?;
    }

    Ok(())
}

fn get_e_tag_last_modified(response: &Response) -> (String, chrono::DateTime<chrono::FixedOffset>) {
    let e_tag = response
        .headers()
        .get("ETag")
        .expect("fail to get ETag")
        .to_str()
        .expect("fail to get ETag as str")
        .replace("W/", ""); // Note: W/ means Weak ETag.
    let last_modified = {
        let last_modified_string = response
            .headers()
            .get("Last-Modified")
            .expect("fail to get last-modified");
        chrono::DateTime::parse_from_rfc2822(
            last_modified_string
                .to_str()
                .expect("fail to get last-modified as str"),
        )
        .expect("fail to parse last-modified")
    };
    (e_tag, last_modified)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ETagCacheValue {
    last_modified: chrono::DateTime<chrono::FixedOffset>,
    e_tag: String,
    raw: Box<[u8]>,
}
