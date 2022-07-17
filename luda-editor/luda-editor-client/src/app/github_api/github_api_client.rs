use dashmap::DashMap;

#[derive(Debug)]
pub struct GithubAPiClient {
    access_token: String,
    base_url: String,
    owner: String,
    repo: String,
    path_sha_map: DashMap<String, String>,
}

impl GithubAPiClient {
    pub fn new(access_token: String, base_url: String, owner: String, repo: String) -> Self {
        Self {
            access_token,
            base_url,
            repo,
            owner,
            path_sha_map: DashMap::new(),
        }
    }

    pub(super) fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub(super) fn get_base_url(&self) -> &String {
        &self.base_url
    }

    pub(super) fn get_owner(&self) -> &String {
        &self.owner
    }

    pub(super) fn get_repo(&self) -> &String {
        &self.repo
    }

    pub(super) fn get_sha(&self, path: &str) -> Option<dashmap::mapref::one::Ref<String, String>> {
        self.path_sha_map.get(path)
    }
    pub(super) fn set_sha(&self, path: String, sha: String) {
        self.path_sha_map.insert(path, sha);
    }
    pub(super) fn remove_sha(&self, path: &str) {
        self.path_sha_map.remove(path);
    }
}
