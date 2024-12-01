use crate::*;
use api::{episode_editor::EPISODE_EDITOR_LOCK_TIMEOUT, team::IsTeamMember};
use database::{schema::*, WantUpdate};
use luda_rpc::episode_editor::try_edit_episode::*;

pub async fn try_edit_episode(
    Request { episode_id, action }: Request,
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

    enum AbortReason {
        YouDoNotHaveEditorLock,
        InvalidSceneIndex,
        SceneNotExists,
    }

    let editor_lock_check = |doc: &EpisodeDoc| {
        let Some(editing_user) = doc.editing_user.as_ref() else {
            return WantUpdate::Abort {
                reason: AbortReason::YouDoNotHaveEditorLock,
            };
        };

        if editing_user.user_id != user_id {
            return WantUpdate::Abort {
                reason: AbortReason::YouDoNotHaveEditorLock,
            };
        }

        let timeout = EPISODE_EDITOR_LOCK_TIMEOUT < SystemTime::now() - editing_user.last_edit_time;

        if timeout {
            return WantUpdate::Abort {
                reason: AbortReason::YouDoNotHaveEditorLock,
            };
        }

        WantUpdate::Yes
    };

    match action {
        luda_rpc::EpisodeEditAction::AddScene { index, scene } => {
            db.transact::<AbortReason>((EpisodeDocUpdate {
                id: episode_id,
                want_update: |doc| {
                    if doc.scenes.len() < index {
                        return WantUpdate::Abort {
                            reason: AbortReason::InvalidSceneIndex,
                        };
                    }

                    editor_lock_check(doc)
                },
                update: |doc| {
                    doc.scenes.insert(index, scene.id, scene);
                },
            },))
                .await
        }
        luda_rpc::EpisodeEditAction::RemoveScene { id } => {
            db.transact::<AbortReason>((EpisodeDocUpdate {
                id: episode_id,
                want_update: |doc| {
                    if !doc.scenes.contains_key(&id) {
                        return WantUpdate::No;
                    }

                    editor_lock_check(doc)
                },
                update: |doc| {
                    doc.scenes.remove_by_key(&id);
                },
            },))
                .await
        }
        luda_rpc::EpisodeEditAction::EditText {
            scene_id,
            language_code,
            text,
        } => {
            db.transact::<AbortReason>((EpisodeDocUpdate {
                id: episode_id,
                want_update: |doc| {
                    if !doc.scenes.contains_key(&scene_id) {
                        return WantUpdate::Abort {
                            reason: AbortReason::SceneNotExists,
                        };
                    }

                    editor_lock_check(doc)
                },
                update: |doc| {
                    doc.scenes
                        .get_mut_by_key(&scene_id)
                        .unwrap()
                        .text_l10n
                        .insert(language_code.to_string(), text.to_string());
                },
            },))
                .await
        }
        luda_rpc::EpisodeEditAction::UpdateScene { scene } => {
            // TODO: Use scene directly without deserialization

            let scene_id = scene.id;
            db.transact::<AbortReason>((EpisodeDocUpdate {
                id: episode_id,
                want_update: |doc| {
                    if !doc.scenes.contains_key(&scene_id) {
                        return WantUpdate::Abort {
                            reason: AbortReason::SceneNotExists,
                        };
                    }

                    editor_lock_check(doc)
                },
                update: move |doc| {
                    doc.scenes.update_by_key(scene.id, scene).unwrap();
                },
            },))
                .await
        }
    }
    .map_err(|err| match err {
        database::Error::NotExistsOnUpdate => anyhow!(Error::EpisodeNotExists),
        _ => anyhow!(err),
    })?
    .err_if_aborted(|reason| match reason {
        AbortReason::YouDoNotHaveEditorLock => Error::YouDoNotHaveEditorLock,
        AbortReason::InvalidSceneIndex => Error::InvalidSceneIndex,
        AbortReason::SceneNotExists => Error::SceneNotExists,
    })?;

    Ok(Response {})
}
