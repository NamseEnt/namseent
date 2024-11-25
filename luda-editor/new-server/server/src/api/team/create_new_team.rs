use crate::*;
use database::{schema::*, WantUpdate};
use luda_rpc::team::create_new_team::*;

const MAX_TEAM_COUNT: usize = 20;

pub async fn create_new_team(
    ArchivedRequest { name }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let team_id = new_id();

    enum AbortReason {
        TooManyTeams,
    }

    db.transact::<AbortReason>((
        UserDocUpdate {
            id: user_id,
            want_update: |doc| {
                if doc.team_ids.len() > MAX_TEAM_COUNT {
                    return WantUpdate::Abort {
                        reason: AbortReason::TooManyTeams,
                    };
                }
                WantUpdate::Yes
            },
            update: |doc| {
                doc.team_ids.insert(team_id);
            },
        },
        TeamDocPut {
            id: team_id,
            name,
            member_ids: &[user_id].into_iter().collect(),
            project_ids: &Default::default(),
            invite_codes: &Default::default(),
            asset_ids: &Default::default(),
            asset_bytes_limit: 100 * 1024 * 1024, // 100MB
            asset_bytes_used: 0,
        },
        TeamNameAssignDocCreate {
            team_name: name,
            team_id,
        },
    ))
    .await
    .map_err(|err| match err {
        database::Error::AlreadyExistsOnCreate => anyhow!(Error::DuplicatedName),
        _ => anyhow!(err),
    })?
    .err_if_aborted(|reason| match reason {
        AbortReason::TooManyTeams => Error::TooManyTeams,
    })?;

    Ok(Response { team_id })
}
