use super::EPISODE_EDITOR_LOCK_TIMEOUT;
use crate::*;
use api::team::has_episode_edit_permission;
use database::{schema::*, WantUpdate};
use futures::{future::try_join_all, try_join};
use luda_rpc::episode_editor::join_episode_editor::*;
use rkyv::Archive;
use std::collections::HashMap;

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

    let episode = db
        .get(EpisodeDocGet { id: episode_id })
        .await?
        .ok_or(Error::EpisodeNotExist)?;

    let (scenes, texts) = try_join!(
        get_scenes(db, &episode.scene_ids),
        get_texts(db, &episode.scene_ids),
    )?;

    Ok(Response { scenes, texts })
}

async fn get_texts(
    db: &Database,
    scene_ids: &<Vec<String> as Archive>::Archived,
) -> Result<HashMap<String, HashMap<String, String>>> {
    let docs = try_join_all(
        scene_ids
            .iter()
            .map(|scene_id| async move { db.query(SceneTextL10nDocQuery { scene_id }).await }),
    )
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize());

    let mut texts = HashMap::new();

    for doc in docs {
        let entry = texts.entry(doc.scene_id).or_insert_with(HashMap::new);
        entry.insert(doc.language_code, doc.text);
    }

    Ok(texts)
}

async fn get_scenes(
    db: &Database,
    scene_ids: &<Vec<String> as Archive>::Archived,
) -> Result<Vec<SceneDoc>> {
    Ok(try_join_all(
        scene_ids
            .iter()
            .map(|scene_id| async move { db.get(SceneDocGet { id: scene_id }).await }),
    )
    .await?
    .into_iter()
    .flatten()
    .map(|x| x.deserialize())
    .collect())
}

async fn try_lock_editor(db: &Database, episode_id: &str, user_id: &str) -> Result<()> {
    enum AbortReason {
        OtherUserEditing,
    }
    db.transact::<AbortReason>(EpisodeEditingUserDocUpdate {
        episode_id,
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
                user_id: user_id.to_string(),
                last_edit_time: SystemTime::now(),
            });
        },
    })
    .await
    .map_err(|err| match err {
        database::Error::NotExistsOnUpdate => anyhow!(Error::EpisodeNotExist),
        _ => anyhow!(err),
    })?
    .err_if_aborted(|reason| match reason {
        AbortReason::OtherUserEditing => Error::OtherUserEditing,
    })?;

    Ok(())
}
