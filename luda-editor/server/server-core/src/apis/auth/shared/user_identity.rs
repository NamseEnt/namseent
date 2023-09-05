pub enum UserIdentity {
    Github {
        github_user_id: String,
        username: String,
    },
}

impl UserIdentity {
    pub fn into_document_id(self) -> String {
        match self {
            UserIdentity::Github {
                github_user_id,
                username: _,
            } => {
                format!("github.{}", github_user_id)
            }
        }
    }
    pub fn username(&self) -> &str {
        match self {
            UserIdentity::Github {
                github_user_id: _,
                username,
            } => username,
        }
    }
}
