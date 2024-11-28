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
use luda_rpc::{AssetDoc, EpisodeEditAction, Scene};
use properties_panel::PropertiesPanel;
use router::Route;
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

        let (Some(join_result), Some(asset_result)) = (join_result.as_ref(), asset_result.as_ref())
        else {
            ctx.add(typography::center_text(
                wh,
                "로딩중...",
                Color::RED,
                16.int_px(),
            ));
            return;
        };

        match (join_result, asset_result) {
            (Ok((join_episode_editor::Response { scenes }, _)), Ok(_)) => {
                ctx.add(LoadedEpisodeEditor {
                    team_id,
                    project_id,
                    episode_id,
                    initial_scenes: scenes,
                    asset_docs,
                });
            }
            (join_result, asset_result) => {
                let errors = (join_result.as_ref().err(), asset_result.as_ref().err());
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
    asset_docs: Sig<'a, HashMap<u128, AssetDoc>>,
}

impl Component for LoadedEpisodeEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            team_id,
            project_id,
            episode_id,
            initial_scenes,
            asset_docs,
        } = self;
        let (scenes, set_scenes) = ctx.state(|| initial_scenes.clone());
        let (selected_scene_id, set_selected_scene_id) = ctx.state(|| Option::<u128>::None);
        let (action_history, set_action_history) = ctx.state(Vec::<EditActionForUndo>::new);

        let action_to_server_queue_tx = ctx
            .memo(|| {
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

                ctx.spawn(async move {
                    while let Some(action) = rx.recv().await {
                        use rpc::episode_editor::try_edit_episode::*;
                        let result = server_connection()
                            .try_edit_episode(RefRequest {
                                episode_id,
                                action: &action,
                            })
                            .await;

                        todo!()
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
            (set_action_history, set_scenes).mutate(move |(history, scenes)| {
                let Some(action) = history.pop() else { return };

                let action_for_server = match action.clone() {
                    EditActionForUndo::AddScene { id } => EpisodeEditAction::RemoveScene { id },
                    EditActionForUndo::EditText {
                        scene_id,
                        language_code,
                        text,
                    } => EpisodeEditAction::EditText {
                        scene_id,
                        language_code,
                        text,
                    },
                    EditActionForUndo::UpdateScene { scene } => {
                        EpisodeEditAction::UpdateScene { scene }
                    }
                    EditActionForUndo::RemoveNewScene { index, scene } => {
                        EpisodeEditAction::AddScene { index, scene }
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
                        let Some(scene_index) = scenes.iter().position(|x| x.id == scene_id) else {
                            eprintln!("Undo failed: scene not found");
                            return;
                        };
                        scenes[scene_index].text_l10n.insert(language_code, text);
                    }
                    EditActionForUndo::UpdateScene { scene } => {
                        let Some(scene_index) = scenes.iter().position(|x| x.id == scene.id) else {
                            eprintln!("Undo failed: scene not found");
                            return;
                        };
                        scenes[scene_index] = scene;
                    }
                }
            });
        };

        let edit_episode = |action: EpisodeEditAction| {
            if action_to_server_queue_tx.send(action.clone()).is_err() {
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
                            scene,
                            project_id,
                            episode_id,
                            select_speaker,
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
