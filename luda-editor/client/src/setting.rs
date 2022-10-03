#[derive(Clone, serde::Deserialize)]
pub struct Setting {
    pub rpc_endpoint: String,
    pub resource_base_url: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            rpc_endpoint:
                "https://onl32ingagofnhyojrlu3qbkne0uyfts.lambda-url.ap-northeast-2.on.aws/"
                    .to_string(),
            resource_base_url: "https://luda-editor.s3.ap-northeast-2.amazonaws.com/master"
                .to_string(),
        }
    }
}
