use super::EPISODE_EDITOR_LOCK_TIMEOUT;
use crate::*;
use anyhow::anyhow;
use database::{schema::*, WantUpdate};
use luda_rpc::episode_editor::exit_episode_editor::*;

pub async fn exit_episode_editor(
    Request { episode_id }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::NeedLogin)?;

    try_unlock_editor(db, episode_id, user_id).await?;

    Ok(Response {})
}

async fn try_unlock_editor(db: &Database, episode_id: u128, user_id: u128) -> Result<()> {
    db.transact::<()>(EpisodeDocUpdate {
        id: episode_id,
        want_update: |doc| {
            let Some(editing_user) = doc.editing_user.as_ref() else {
                return WantUpdate::No;
            };

            let not_timeout =
                SystemTime::now() - editing_user.last_edit_time < EPISODE_EDITOR_LOCK_TIMEOUT;

            if editing_user.user_id == user_id && not_timeout {
                return WantUpdate::Yes;
            }

            WantUpdate::No
        },
        update: |doc| {
            assert_eq!(doc.editing_user.as_ref().unwrap().user_id, user_id);
            doc.editing_user = None;
        },
    })
    .await
    .map_err(|err| match &err {
        database::Error::NotExistsOnUpdate => anyhow!(Error::EpisodeNotExists),
        _ => anyhow!(err),
    })?;

    Ok(())
}
