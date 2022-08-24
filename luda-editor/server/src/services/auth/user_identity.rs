pub enum UserIdentity {
    Github { github_user_id: String },
}

impl UserIdentity {
    pub fn into_document_id(self) -> String {
        match self {
            UserIdentity::Github { github_user_id } => {
                format!("github.{}", github_user_id)
            }
        }
    }
}
