use crate::*;
use api::team::is_team_member;
use database::schema::*;
use luda_rpc::project::create_new_project::*;

pub async fn create_new_project(
    ArchivedRequest { team_id, name }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response, Error> {
    let Some(user_id) = session.user_id().await else {
        return Err(Error::NeedLogin);
    };

    if !is_team_member(db, team_id, &user_id).await? {
        return Err(Error::PermissionDenied);
    }

    let project_id = randum::rand();

    db.transact((
        ProjectDocPut {
            id: &project_id,
            team_id,
            name,
            ttl: None,
        },
        TeamToProjectDocPut {
            team_id,
            project_id: &project_id,
            ttl: None,
        },
        ProjectNameDocCreate {
            team_id,
            project_name: name,
            ttl: None,
        },
    ))
    .await
    .map_err(|err| match err {
        database::Error::AlreadyExistsOnCreate => Error::DuplicatedName,
        _ => err.into(),
    })?;

    Ok(Response { project_id })
}
