use crate::*;
use api::{episode_editor::EPISODE_EDITOR_LOCK_TIMEOUT, team::has_episode_edit_permission};
use database::{schema::*, DeserializeInfallible, WantUpdate};
use luda_rpc::episode_editor::try_edit_episode::*;

pub async fn try_edit_episode(
    ArchivedRequest { episode_id, action }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().await.ok_or(Error::NeedLogin)?;

    if !has_episode_edit_permission(db, episode_id, &user_id).await? {
        bail!(Error::PermissionDenied)
    }

    enum AbortReason {
        YouDoNotHaveEditorLock,
        InvalidSceneIndex,
    }

    let editor_lock_check = EpisodeEditingUserDocUpdate {
        episode_id,
        want_update: |doc| {
            let Some(editing_user) = doc.editing_user.as_ref() else {
                return WantUpdate::Abort {
                    reason: AbortReason::YouDoNotHaveEditorLock,
                };
            };

            if editing_user.user_id != *user_id {
                return WantUpdate::Abort {
                    reason: AbortReason::YouDoNotHaveEditorLock,
                };
            }

            let timeout =
                EPISODE_EDITOR_LOCK_TIMEOUT < SystemTime::now() - editing_user.last_edit_time;

            if timeout {
                return WantUpdate::Abort {
                    reason: AbortReason::YouDoNotHaveEditorLock,
                };
            }

            WantUpdate::No
        },
        update: |_doc| unreachable!(),
    };

    match action {
        luda_rpc::ArchivedEpisodeEditAction::AddScene { index, scene } => {
            // TODO: Use scene directly without deserialization
            let scene: SceneDoc = scene.deserialize();

            db.transact::<AbortReason>((
                editor_lock_check,
                EpisodeDocUpdate {
                    id: episode_id,
                    want_update: |doc| {
                        if doc.scene_ids.len() < *index as usize {
                            return WantUpdate::Abort {
                                reason: AbortReason::InvalidSceneIndex,
                            };
                        }

                        WantUpdate::Yes
                    },
                    update: |doc| {
                        doc.scene_ids.insert(*index as usize, scene.id.clone());
                    },
                },
                SceneDocPut {
                    background_sprite: &scene.background_sprite,
                    id: &scene.id,
                    speaker_id: &scene.speaker_id,
                    scene_sprites: &scene.scene_sprites,
                    bgm: &scene.bgm,
                    ttl: None,
                },
            ))
            .await
        }
        luda_rpc::ArchivedEpisodeEditAction::RemoveScene { id } => {
            db.transact::<AbortReason>((
                editor_lock_check,
                EpisodeDocUpdate {
                    id: episode_id,
                    want_update: |doc| {
                        if doc.scene_ids.contains(id) {
                            return WantUpdate::No;
                        }

                        WantUpdate::Yes
                    },
                    update: |doc| {
                        doc.scene_ids.retain(|scene_id| scene_id != id);
                    },
                },
                SceneDocDelete { id },
            ))
            .await
        }
        luda_rpc::ArchivedEpisodeEditAction::EditText {
            scene_id,
            language_code,
            text,
        } => {
            db.transact::<AbortReason>((
                editor_lock_check,
                SceneTextL10nDocPut {
                    scene_id,
                    language_code,
                    text,
                    ttl: None,
                },
            ))
            .await
        }
        luda_rpc::ArchivedEpisodeEditAction::UpdateScene { scene } => {
            // TODO: Use scene directly without deserialization
            let scene: SceneDoc = scene.deserialize();

            db.transact::<AbortReason>((
                editor_lock_check,
                SceneDocPut {
                    background_sprite: &scene.background_sprite,
                    id: &scene.id,
                    speaker_id: &scene.speaker_id,
                    scene_sprites: &scene.scene_sprites,
                    bgm: &scene.bgm,
                    ttl: None,
                },
            ))
            .await
        }
    }
    .map_err(|err| match err {
        database::Error::NotExistsOnUpdate => anyhow!(Error::EpisodeNotExist),
        _ => anyhow!(err),
    })?
    .err_if_aborted(|reason| match reason {
        AbortReason::YouDoNotHaveEditorLock => Error::YouDoNotHaveEditorLock,
        AbortReason::InvalidSceneIndex => Error::InvalidSceneIndex,
    })?;

    todo!()
}
