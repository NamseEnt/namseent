use super::EPISODE_EDITOR_LOCK_TIMEOUT;
use crate::*;
use api::team::IsTeamMember;
use database::{WantUpdate, schema::*};
use luda_rpc::episode_editor::join_episode_editor::*;

pub async fn join_episode_editor(
    Request { episode_id }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    let episode_doc = db
        .get(EpisodeDocGet { id: episode_id })
        .await?
        .ok_or(Error::EpisodeNotExists)?;

    let team_doc = db
        .get(TeamDocGet {
            id: episode_doc.team_id,
        })
        .await?
        .ok_or(Error::PermissionDenied)?;

    if !team_doc.is_team_member(user_id) {
        bail!(Error::PermissionDenied)
    }

    try_lock_editor(db, episode_id, user_id).await?;

    let episode = db
        .get(EpisodeDocGet { id: episode_id })
        .await?
        .ok_or(Error::EpisodeNotExists)?;

    Ok(Response {
        scenes: episode.scenes.into_values().collect(),
    })
}

async fn try_lock_editor(db: &Database, episode_id: u128, user_id: u128) -> Result<()> {
    enum AbortReason {
        OtherUserEditing,
    }
    db.transact::<AbortReason>(EpisodeDocUpdate {
        id: episode_id,
        want_update: |doc| {
            let Some(editing_user) = doc.editing_user.as_ref() else {
                return WantUpdate::Yes;
            };

            let not_timeout =
                SystemTime::now() - editing_user.last_edit_time < EPISODE_EDITOR_LOCK_TIMEOUT;

            if editing_user.user_id == user_id {
                if not_timeout {
                    return WantUpdate::No;
                }

                return WantUpdate::Yes;
            }

            if not_timeout {
                return WantUpdate::Abort {
                    reason: AbortReason::OtherUserEditing,
                };
            }

            WantUpdate::Yes
        },
        update: |doc| {
            doc.editing_user = Some(EditingUser {
                user_id,
                last_edit_time: SystemTime::now(),
            });
        },
    })
    .await
    .map_err(|err| match err {
        database::Error::NotExistsOnUpdate => anyhow!(Error::EpisodeNotExists),
        _ => anyhow!(err),
    })?
    .err_if_aborted(|reason| match reason {
        AbortReason::OtherUserEditing => Error::OtherUserEditing,
    })?;

    Ok(())
}
