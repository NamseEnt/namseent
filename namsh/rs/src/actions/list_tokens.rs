use crate::common::auth;
use forte_sdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {}

#[derive(Serialize)]
pub struct TokenSummary {
    pub id: String,
    pub label: String,
    pub created_at: DateTime,
}

#[derive(Serialize)]
pub enum Output {
    Ok { tokens: Vec<TokenSummary> },
    NotLoggedIn,
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(user) = auth::current_user(req.jar).await else {
        return Output::NotLoggedIn;
    };

    let tokens = user
        .cli_tokens
        .into_iter()
        .map(|t| TokenSummary {
            id: t.id,
            label: t.label,
            created_at: t.created_at,
        })
        .collect();

    Output::Ok { tokens }
}
