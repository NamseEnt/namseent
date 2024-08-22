use crate::*;
use api::team::has_episode_edit_permission;
use database::{schema::*, WantUpdate};
use luda_rpc::episode_editor::join_episode_editor::*;

const LOCK_TIMEOUT: Duration = Duration::from_secs(300);

pub async fn join_episode_editor(
    ArchivedRequest { episode_id }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !has_episode_edit_permission(db, episode_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    try_lock_editor(db, episode_id, &user_id).await?;

    Ok(Response {
        scenes: todo!(),
        texts: todo!(),
    })
}

async fn try_lock_editor(db: &Database, episode_id: &str, user_id: &str) -> Result<()> {
    if let Err(err) = db
        .transact(EpisodeEditingUserDocUpdate {
            episode_id,
            want_update: |doc| {
                let Some(editing_user) = doc.editing_user.as_ref() else {
                    return WantUpdate::Yes;
                };

                if editing_user.user_id == user_id {
                    if SystemTime::now() - editing_user.last_edit_time < LOCK_TIMEOUT {
                        return WantUpdate::No;
                    }

                    return WantUpdate::Yes;
                }

                if SystemTime::now() - editing_user.last_edit_time < LOCK_TIMEOUT {
                    return WantUpdate::Abort;
                }

                WantUpdate::Yes
            },
            update: |doc| {
                doc.editing_user = Some(EditingUser {
                    user_id: user_id.to_string(),
                    last_edit_time: SystemTime::now(),
                });
            },
        })
        .await
    {
        match err {
            database::Error::UpdateAborted => {
                bail!(Error::OtherUserEditing)
            }
            _ => return Err(err.into()),
        }
    }

    Ok(())
}
