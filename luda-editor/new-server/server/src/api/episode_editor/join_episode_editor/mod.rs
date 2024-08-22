use crate::*;
use api::team::has_episode_edit_permission;
use database::{schema::*, WantUpdate};
use futures::{future::try_join_all, try_join};
use luda_rpc::episode_editor::join_episode_editor::*;
use rkyv::Archive;
use std::collections::HashMap;

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
