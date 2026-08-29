//! Headless game simulation through the versioned environment.

pub mod environment;
pub mod events;
pub mod hp_balance;
pub mod ml;
pub mod policy_runner;
pub mod recording;
pub mod stats;
pub mod trajectory;

pub(crate) fn canonicalize_kind_name(name: String) -> String {
    let trimmed = name.trim();
    let token: String = trimmed
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    let token = token.strip_suffix("Upgrade").unwrap_or(&token);
    token.to_string()
}
