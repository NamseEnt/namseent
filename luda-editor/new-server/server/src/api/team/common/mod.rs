use database::{schema::*, Database};

pub async fn is_team_member(db: &Database, team_id: &str, user_id: &str) -> database::Result<bool> {
    let user_team_query = db.query(UserToTeamDocQuery { user_id }).await?;
    Ok(user_team_query.iter().any(|doc| doc.team_id == *team_id))
}

pub async fn is_project_member(
    db: &Database,
    project_id: &str,
    user_id: &str,
) -> database::Result<bool> {
    let Some(doc) = db.get(ProjectToTeamDocGet { project_id }).await? else {
        return Ok(false);
    };
    is_team_member(db, &doc.team_id, user_id).await
}

pub async fn has_episode_edit_permission(
    db: &Database,
    episode_id: &str,
    user_id: &str,
) -> database::Result<bool> {
    let Some(doc) = db.get(EpisodeToProjectDocGet { episode_id }).await? else {
        return Ok(false);
    };
    is_project_member(db, &doc.project_id, user_id).await
}
