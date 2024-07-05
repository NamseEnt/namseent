use database::{schema::UserToTeamDocQuery, Database};

pub async fn is_team_member(db: &Database, team_id: &str, user_id: &str) -> database::Result<bool> {
    let user_team_query = db.query(UserToTeamDocQuery { user_id }).await?;
    Ok(user_team_query.iter().any(|doc| doc.team_id == *team_id))
}
