mod properties_panel;
mod scene_audio_editor;
mod scene_list;
mod scene_preview;
mod scene_sprite_editor;
mod speaker_selector;
mod text_editor;

use super::*;
use crate::rpc::asset::get_team_asset_docs;
use crate::rpc::episode_editor::join_episode_editor;
use luda_rpc::{AssetDoc, EpisodeEditAction, Scene, Speaker};
use properties_panel::PropertiesPanel;
use router::Route;
use rpc::{episode_editor::load_speaker_slots, project::list_speakers};
use speaker_selector::EditSpeakerModal;
use std::{collections::HashMap, sync::Arc};

pub struct EpisodeEditor {
    pub team_id: u128,
    pub project_id: u128,
    pub episode_id: u128,
}

impl Component for EpisodeEditor {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            team_id,
            project_id,
            episode_id,
        } = self;

        let wh = namui::screen::size().map(|x| x.into_px());

        let join_result = join_episode_editor::join_episode_editor(
            ctx,
            |episode_id| {
                Some((
                    join_episode_editor::RefRequest {
                        episode_id: *episode_id,
                    },
                    (),
                ))
            },
            &episode_id,
        );
        let asset_result = get_team_asset_docs::get_team_asset_docs(
            ctx,
            |team_id| Some((get_team_asset_docs::RefRequest { team_id: *team_id }, ())),
            &team_id,
        );
        let speaker_result = list_speakers::list_speakers(
            ctx,
            |project_id| {
                Some((
                    list_speakers::RefRequest {
                        project_id: *project_id,
                    },
                    (),
                ))
            },
            &project_id,
        );
        let speaker_slot_result = load_speaker_slots::load_speaker_slots(
            ctx,
            |episode_id| {
                Some((
                    load_speaker_slots::RefRequest {
                        episode_id: *episode_id,
                    },
                    (),
                ))
            },
            &episode_id,
        );
        let asset_docs = ctx.memo({
            || {
                let Some(Ok((get_team_asset_docs::Response { asset_docs }, _))) =
                    asset_result.as_ref()
                else {
                    return HashMap::new();
                };
                asset_docs
                    .iter()
                    .map(|asset_doc| (asset_doc.id, asset_doc.clone()))
                    .collect()
            }
        });

        let (
            Some(join_result),
            Some(asset_result),
            Some(speaker_result),
            Some(speaker_slot_result),
        ) = (
            join_result.as_ref(),
            asset_result.as_ref(),
            speaker_result.as_ref(),
            speaker_slot_result.as_ref(),
        )
        else {
            ctx.add(typography::center_text(
                wh,
                "로딩중...",
                Color::RED,
                16.int_px(),
            ));
            return;
        };

        match (
            join_result,
            asset_result,
            speaker_result,
            speaker_slot_result,
        ) {
            (
                Ok((join_episode_editor::Response { scenes }, _)),
                Ok(_),
                Ok((list_speakers::Response { speakers }, _)),
                Ok((load_speaker_slots::Response { speaker_ids }, _)),
            ) => {
                ctx.add(LoadedEpisodeEditor {
                    team_id,
                    project_id,
                    episode_id,
                    initial_scenes: scenes,
                    initial_speakers: speakers,
                    initial_speaker_slots: speaker_ids,
                    asset_docs,
                });
            }
            (join_result, asset_result, speaker_result, speaker_slot_result) => {
                let errors = (
                    join_result.as_ref().err(),
                    asset_result.as_ref().err(),
                    speaker_result.as_ref().err(),
                    speaker_slot_result.as_ref().err(),
                );
                ctx.add(typography::center_text(
                    wh,
                    format!("에러: {:#?}", errors),
                    Color::RED,
                    16.int_px(),
                ));
            }
        }
    }
}

struct LoadedEpisodeEditor<'a> {
    team_id: u128,
    project_id: u128,
    episode_id: u128,
    initial_scenes: &'a Vec<Scene>,
    initial_speakers: &'a Vec<Speaker>,
    initial_speaker_slots: &'a Vec<u128>,
    asset_docs: Sig<'a, HashMap<u128, AssetDoc>>,
}

impl Component for LoadedEpisodeEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            team_id,
            project_id,
            episode_id,
            initial_scenes,
            initial_speakers,
            initial_speaker_slots,
            asset_docs,
        } = self;
        let (scenes, set_scenes) = ctx.state(|| initial_scenes.clone());
        let (speakers, set_speakers) = ctx.state(|| initial_speakers.clone());
        let (speaker_slots, set_speaker_slots) = ctx.state(|| initial_speaker_slots.clone());
        let (selected_scene_id, set_selected_scene_id) = ctx.state(|| Option::<u128>::None);
        let (action_history, set_action_history) = ctx.state(Vec::<EditActionForUndo>::new);
        let (show_edit_speaker_modal, set_show_edit_speaker_modal) = ctx.state(|| false);

        let speakers_in_slot = ctx.memo(|| {
            speakers
                .iter()
                .filter(|x| speaker_slots.contains(&x.id))
                .cloned()
                .collect::<Vec<_>>()
        });

        let action_to_server_queue_tx = ctx
            .memo(|| {
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<EditActionForServer>();

                ctx.spawn(async move {
                    while let Some(action) = rx.recv().await {
                        match action {
                            EditActionForServer::Project { action } => match action {
                                ProjectEditAction::PutSpeaker { speaker } => {
                                    use rpc::project::put_speaker::*;
                                    let result = server_connection()
                                        .put_speaker(RefRequest {
                                            project_id,
                                            speaker: &speaker,
                                        })
                                        .await;
                                    if let Err(err) = result {
                                        toast::negative(format!("인물 추가 실패: {:#?}", err));
                                    }
                                }
                                ProjectEditAction::DeleteSpeaker { speaker_id } => {
                                    use rpc::project::delete_speaker::*;
                                    let result = server_connection()
                                        .delete_speaker(RefRequest {
                                            project_id,
                                            speaker_id,
                                        })
                                        .await;
                                    if let Err(err) = result {
                                        toast::negative(format!("인물 삭제 실패: {:#?}", err));
                                    }
                                }
                                ProjectEditAction::SaveSpeakerSlots { speaker_slots } => {
                                    use rpc::episode_editor::save_speaker_slots::*;
                                    let result = server_connection()
                                        .save_speaker_slots(RefRequest {
                                            episode_id,
                                            speaker_ids: &speaker_slots,
                                        })
                                        .await;
                                    if let Err(err) = result {
                                        toast::negative(format!("인물 슬롯 저장 실패: {:#?}", err));
                                    }
                                }
                            },
                            EditActionForServer::Episode { action } => {
                                use rpc::episode_editor::try_edit_episode::*;
                                let result = server_connection()
                                    .try_edit_episode(RefRequest {
                                        episode_id,
                                        action: &action,
                                    })
                                    .await;

                                todo!();
                            }
                        }
                    }
                });

                Arc::new(tx)
            })
            .clone_inner();

        let undo = || {
            if action_history.is_empty() {
                return;
            };
            let action_to_server_queue_tx = action_to_server_queue_tx.clone();
            (
                set_action_history,
                set_scenes,
                set_speakers,
                set_speaker_slots,
            )
                .mutate(move |(history, scenes, speakers, speaker_slots)| {
                    let Some(action) = history.pop() else { return };

                    let action_for_server = match action.clone() {
                        EditActionForUndo::AddScene { id } => EditActionForServer::Episode {
                            action: EpisodeEditAction::RemoveScene { id },
                        },
                        EditActionForUndo::EditText {
                            scene_id,
                            language_code,
                            text,
                        } => EditActionForServer::Episode {
                            action: EpisodeEditAction::EditText {
                                scene_id,
                                language_code,
                                text,
                            },
                        },
                        EditActionForUndo::UpdateScene { scene } => EditActionForServer::Episode {
                            action: EpisodeEditAction::UpdateScene { scene },
                        },
                        EditActionForUndo::RemoveNewScene { index, scene } => {
                            EditActionForServer::Episode {
                                action: EpisodeEditAction::AddScene { index, scene },
                            }
                        }
                    };
                    action_to_server_queue_tx.send(action_for_server).unwrap();

                    match action {
                        EditActionForUndo::AddScene { id } => {
                            let index = scenes.iter().position(|x| x.id == id).unwrap();
                            scenes.remove(index);
                        }
                        EditActionForUndo::RemoveNewScene { scene, index } => {
                            scenes.insert(index, scene);
                        }
                        EditActionForUndo::EditText {
                            scene_id,
                            language_code,
                            text,
                        } => {
                            let Some(scene_index) = scenes.iter().position(|x| x.id == scene_id)
                            else {
                                eprintln!("Undo failed: scene not found");
                                return;
                            };
                            scenes[scene_index].text_l10n.insert(language_code, text);
                        }
                        EditActionForUndo::UpdateScene { scene } => {
                            let Some(scene_index) = scenes.iter().position(|x| x.id == scene.id)
                            else {
                                eprintln!("Undo failed: scene not found");
                                return;
                            };
                            scenes[scene_index] = scene;
                        }
                    }
                });
        };

        let edit_episode = |action: EpisodeEditAction| {
            if action_to_server_queue_tx
                .send(EditActionForServer::Episode {
                    action: action.clone(),
                })
                .is_err()
            {
                return;
            }

            match action {
                EpisodeEditAction::AddScene { index, scene } => (set_scenes, set_action_history)
                    .mutate({
                        move |(scenes, history)| {
                            let id = scene.id;
                            scenes.insert(index, scene);
                            history.push(EditActionForUndo::AddScene { id });
                        }
                    }),
                EpisodeEditAction::RemoveScene { id } => {
                    (set_scenes, set_action_history).mutate({
                        move |(scenes, history)| {
                            let index = scenes.iter().position(|x| x.id == id).unwrap();
                            let scene = scenes.remove(index);
                            history.push(EditActionForUndo::RemoveNewScene { index, scene });
                        }
                    });
                }
                EpisodeEditAction::EditText {
                    scene_id,
                    language_code,
                    text,
                } => {
                    (set_scenes, set_action_history).mutate(move |(scenes, history)| {
                        let scene = scenes.iter_mut().find(|x| x.id == scene_id).unwrap();
                        let text = scene
                            .text_l10n
                            .insert(language_code.clone(), text.clone())
                            .unwrap_or_default();
                        history.push(EditActionForUndo::EditText {
                            scene_id,
                            language_code,
                            text,
                        });
                    });
                }
                EpisodeEditAction::UpdateScene { scene } => {
                    (set_scenes, set_action_history).mutate(move |(scenes, history)| {
                        let scene_index = scenes.iter().position(|x| x.id == scene.id).unwrap();
                        let scene = std::mem::replace(&mut scenes[scene_index], scene);
                        history.push(EditActionForUndo::UpdateScene { scene });
                    });
                }
            }
        };

        let edit_project = |action: ProjectEditAction| {
            if action_to_server_queue_tx
                .send(EditActionForServer::Project {
                    action: action.clone(),
                })
                .is_err()
            {
                return;
            }

            match action {
                ProjectEditAction::PutSpeaker { speaker } => {
                    set_speakers.mutate(move |speakers| {
                        if speakers.iter().any(|x| x.id == speaker.id) {
                            return;
                        }
                        speakers.push(speaker.clone());
                    });
                }
                ProjectEditAction::DeleteSpeaker { speaker_id } => {
                    set_speakers.mutate(move |speakers| {
                        let speaker_index =
                            speakers.iter().position(|x| x.id == speaker_id).unwrap();
                        speakers.remove(speaker_index);
                    });
                }
                ProjectEditAction::SaveSpeakerSlots { speaker_slots } => {
                    set_speaker_slots.mutate(move |slots| {
                        let _ = std::mem::replace(slots, speaker_slots.clone());
                    });
                }
            };
        };

        let add_new_scene = || {
            edit_episode(EpisodeEditAction::AddScene {
                index: scenes.len(),
                scene: Scene {
                    id: namui::uuid(),
                    speaker_id: None,
                    scene_sprites: vec![],
                    background_sprite: None,
                    bgm: None,
                    text_l10n: HashMap::new(),
                },
            });
        };

        let scene = selected_scene_id
            .as_ref()
            .as_ref()
            .and_then(|id| scenes.iter().find(|x| &x.id == id));

        let select_speaker = &|speaker_id: u128| {
            let Some(scene) = scene else { return };
            edit_episode(EpisodeEditAction::UpdateScene {
                scene: {
                    let mut scene = scene.clone();
                    scene.speaker_id = Some(speaker_id);
                    scene
                },
            });
        };

        let add_speaker = &|speaker_name: String| {
            let speaker = Speaker {
                id: uuid(),
                name_l10n: HashMap::from_iter([("kor".to_string(), speaker_name)]),
            };
            edit_project(ProjectEditAction::PutSpeaker { speaker });
        };

        let save_speaker_slots = &|speaker_slots: Vec<u128>| {
            edit_project(ProjectEditAction::SaveSpeakerSlots { speaker_slots });
        };

        let on_text_edit_done = &|scene_id: u128, text: String| {
            edit_episode(EpisodeEditAction::EditText {
                scene_id,
                language_code: "kor".to_string(),
                text,
            });
        };

        let select_scene = &|scene_id: u128| {
            set_selected_scene_id.set(Some(scene_id));
        };

        let wh = namui::screen::size().map(|x| x.into_px());

        let scene_list = table::fixed(160.px(), |wh, ctx| {
            ctx.add(scene_list::SceneList {
                wh,
                scenes: &scenes,
                select_scene,
                add_new_scene: &add_new_scene,
            });
        });
        let scene_editor = table::ratio(1, |wh, ctx| {
            let Some(scene) = scene else { return };

            ctx.compose(|ctx| {
                table::vertical([
                    table::ratio(1, |wh, ctx| {
                        ctx.add(scene_preview::ScenePreview { wh, scene });
                    }),
                    table::fixed(48.px(), |wh, ctx| {
                        ctx.add(speaker_selector::SpeakerSelector {
                            wh,
                            select_speaker,
                            speakers_in_slot,
                            open_edit_speaker_modal: &|| set_show_edit_speaker_modal.set(true),
                        });
                    }),
                    table::fixed(320.px(), |wh, ctx| {
                        let empty_text = "".to_string();
                        let text = scene.text_l10n.get("kor").unwrap_or(&empty_text);
                        ctx.add(text_editor::TextEditor {
                            wh,
                            text,
                            scene_id: scene.id,
                            on_edit_done: on_text_edit_done,
                        });
                    }),
                ])(wh, ctx);
            });
        });
        let properties_panel = table::ratio(1, |wh, ctx| {
            let Some(scene) = scene else { return };
            ctx.add(PropertiesPanel {
                wh,
                scene,
                edit_episode: &edit_episode,
                asset_docs,
            });
        });

        let top_bar = table::fixed(24.px(), |wh, ctx| {
            let button_wh = Wh::new(128.px(), wh.height);
            ctx.add(simple_button(button_wh, "back", |_| {
                router::route(Route::Home {
                    initial_selection: home::Selection::Project {
                        team_id,
                        project_id,
                    },
                });
            }));
        });

        ctx.compose(|ctx| {
            if !*show_edit_speaker_modal {
                return;
            }
            ctx.add(EditSpeakerModal {
                speakers,
                speaker_slots,
                add_speaker,
                save_speaker_slots,
                close: &|| set_show_edit_speaker_modal.set(false),
            });
        });

        ctx.compose(|ctx| {
            vertical([
                top_bar,
                ratio(1, horizontal([scene_list, scene_editor, properties_panel])),
            ])(wh, ctx)
        });
    }
}

#[derive(Debug, Clone)]
enum EditActionForUndo {
    AddScene {
        id: u128,
    },
    RemoveNewScene {
        index: usize,
        scene: Scene,
    },
    EditText {
        scene_id: u128,
        language_code: String,
        text: String,
    },
    UpdateScene {
        scene: Scene,
    },
}

#[derive(Debug, Clone)]
enum EditActionForServer {
    Project { action: ProjectEditAction },
    Episode { action: EpisodeEditAction },
}

#[derive(Debug, Clone)]
enum ProjectEditAction {
    PutSpeaker { speaker: Speaker },
    DeleteSpeaker { speaker_id: u128 },
    SaveSpeakerSlots { speaker_slots: Vec<u128> },
}
