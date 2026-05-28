use crate::actions::list_tokens::TokenSummary;
use crate::common::auth;
use crate::docs::UserDoc;
use crate::route_generated::Redirect;
use forte_sdk::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Props {
    pub github_id: i64,
    pub github_login: String,
    pub tokens: Vec<TokenSummary>,
}

pub async fn handler(req: ForteRequest<'_>) -> anyhow::Result<Props> {
    let Some(user) = auth::current_user(req.jar).await else {
        return Err(Redirect::Login.into());
    };
    let UserDoc {
        github_id,
        github_login,
        cli_tokens,
        ..
    } = user;
    let tokens = cli_tokens
        .into_iter()
        .map(|t| TokenSummary {
            id: t.id,
            label: t.label,
            created_at: t.created_at,
        })
        .collect();
    Ok(Props {
        github_id,
        github_login,
        tokens,
    })
}
