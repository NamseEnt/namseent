use crate::*;
use api::team::IsTeamMember;
use database::{WantUpdate, schema::*};
use luda_rpc::project::create_new_project::*;

const MAX_PROJECTS_PER_TEAM: usize = 100;

pub async fn create_new_project(
    Request { team_id, name }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let team_doc = db
        .get(TeamDocGet { id: team_id })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    let project_id = new_id();

    enum AbortReason {
        TooManyProjects,
    }

    db.transact::<AbortReason>((
        TeamDocUpdate {
            id: team_id,
            want_update: |doc| {
                if doc.project_ids.len() > MAX_PROJECTS_PER_TEAM {
                    return WantUpdate::Abort {
                        reason: AbortReason::TooManyProjects,
                    };
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.project_ids.insert(project_id);
            },
        },
        ProjectDocPut {
            id: project_id,
            name: &name,
            team_id,
            speakers: &Default::default(),
            episode_ids: &Default::default(),
        },
    ))
    .await?
    .err_if_aborted(|reason| match reason {
        AbortReason::TooManyProjects => Error::TooManyProjects,
    })?;

    Ok(Response { project_id })
}
