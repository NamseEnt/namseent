mod scene_list;
mod scene_preview;
mod speaker_selector;
mod text_editor;

use super::*;
use luda_rpc::{EpisodeEditAction, Scene};
use std::collections::HashMap;

pub struct EpisodeEditor<'a> {
    pub project_id: &'a String,
    pub episode_id: &'a String,
}

impl Component for EpisodeEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            project_id,
            episode_id,
        } = self;

        let wh = namui::screen::size().map(|x| x.into_px());

        {
            use crate::rpc::episode_editor::join_episode_editor::*;
            let result = join_episode_editor(
                ctx,
                |episode_id| Some((RefRequest { episode_id }, ())),
                episode_id,
            );

            let Some(result) = result.as_ref() else {
                ctx.add(typography::center_text(
                    wh,
                    "로딩중...",
                    Color::RED,
                    16.int_px(),
                ));
                return;
            };

            match result {
                Ok((Response { scenes, texts }, _)) => {
                    ctx.add(LoadedEpisodeEditor {
                        project_id,
                        episode_id,
                        initial_scenes: scenes,
                        initial_texts: texts,
                    });
                }
                Err(err) => {
                    ctx.add(typography::center_text(
                        wh,
                        format!("에러: {:?}", err),
                        Color::RED,
                        16.int_px(),
                    ));
                }
            }
        }
    }
}

struct LoadedEpisodeEditor<'a> {
    project_id: &'a String,
    episode_id: &'a String,
    initial_scenes: &'a Vec<Scene>,
    initial_texts: &'a HashMap<String, HashMap<String, Option<String>>>,
}

impl Component for LoadedEpisodeEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            project_id,
            episode_id,
            initial_scenes,
            initial_texts,
        } = self;
        let (scenes, set_scenes) = ctx.state(|| initial_scenes.clone());
        let (texts, set_texts) = ctx.state(|| initial_texts.clone());
        let (selected_scene_id, set_selected_scene_id) = ctx.state(|| Option::<String>::None);
        let (action_history, set_action_history) = ctx.state(Vec::<EditActionForUndo>::new);

        let action_queue_tx = ctx.memo(|| {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

            let episode_id = episode_id.clone();
            ctx.spawn(async move {
                while let Some(action) = rx.recv().await {
                    use rpc::episode_editor::try_edit_episode::*;
                    let result = server_connection()
                        .try_edit_episode(RefRequest {
                            episode_id: &episode_id,
                            action: &action,
                        })
                        .await;

                    todo!()
                }
            });

            tx
        });

        let undo = || {
            if action_history.is_empty() {
                return;
            };
            (set_action_history, set_scenes, set_texts).mutate(|(history, scenes, texts)| {
                let Some(action) = history.pop() else { return };

                let action_for_server = match action.clone() {
                    EditActionForUndo::AddNewScene { id } => EpisodeEditAction::RemoveScene { id },
                    EditActionForUndo::EditText {
                        scene_id,
                        language_code,
                        prev_text,
                    } => EpisodeEditAction::EditText {
                        scene_id,
                        language_code,
                        text: prev_text,
                    },
                    EditActionForUndo::UpdateScene { prev_scene } => {
                        EpisodeEditAction::UpdateScene { scene: prev_scene }
                    }
                };
                // action_queue_tx.send(action_for_server).unwrap();

                match action {
                    EditActionForUndo::AddNewScene { id } => {
                        scenes.retain(|scene| scene.id != id);
                    }
                    EditActionForUndo::EditText {
                        scene_id,
                        language_code,
                        prev_text,
                    } => {
                        texts
                            .get_mut(&scene_id)
                            .unwrap()
                            .insert(language_code, Some(prev_text));
                    }
                    EditActionForUndo::UpdateScene { prev_scene } => {
                        let Some(scene_index) = scenes.iter().position(|x| x.id == prev_scene.id)
                        else {
                            eprintln!("Undo failed: scene not found");
                            return;
                        };
                        scenes[scene_index] = prev_scene;
                    }
                }
            });
        };

        let edit_episode = |action: EpisodeEditAction| {
            if action_queue_tx.send(action.clone()).is_err() {
                return;
            }

            match action {
                EpisodeEditAction::AddNewScene { id } => (set_scenes, set_action_history).mutate({
                    |(scenes, history)| {
                        scenes.push(Scene {
                            id: id.clone(),
                            speaker_id: None,
                            sprites: vec![],
                            background_sprite: None,
                            bgm: None,
                        });
                        history.push(EditActionForUndo::AddNewScene { id });
                    }
                }),
                EpisodeEditAction::EditText {
                    scene_id,
                    language_code,
                    text,
                } => {
                    (set_texts, set_action_history).mutate(move |(texts, history)| {
                        let prev_text = texts
                            .entry(scene_id.clone())
                            .or_insert_with(HashMap::new)
                            .insert(language_code.clone(), Some(text.clone()))
                            .unwrap_or(None)
                            .unwrap_or("".to_string());
                        history.push(EditActionForUndo::EditText {
                            scene_id,
                            language_code,
                            prev_text,
                        });
                    });
                }
                EpisodeEditAction::UpdateScene { scene } => {
                    (set_scenes, set_action_history).mutate(move |(scenes, history)| {
                        let scene_index = scenes.iter().position(|x| x.id == scene.id).unwrap();
                        let prev_scene = std::mem::replace(&mut scenes[scene_index], scene);
                        history.push(EditActionForUndo::UpdateScene { prev_scene });
                    });
                }
                EpisodeEditAction::RemoveScene { id } => todo!(),
            }
        };

        let add_new_scene = || {
            edit_episode(EpisodeEditAction::AddNewScene { id: randum::rand() });
        };

        let scene = selected_scene_id
            .as_ref()
            .as_ref()
            .and_then(|id| scenes.iter().find(|x| &x.id == id));

        let select_speaker = &|speaker_id: &String| {
            let Some(scene) = scene else { return };
            edit_episode(EpisodeEditAction::UpdateScene {
                scene: {
                    let mut scene = scene.clone();
                    scene.speaker_id = Some(speaker_id.clone());
                    scene
                },
            });
        };

        let on_text_edit_done = &|text: String| {
            let Some(scene) = scene else { return };
            edit_episode(EpisodeEditAction::EditText {
                scene_id: scene.id.clone(),
                language_code: "kor".to_string(),
                text,
            });
        };

        let wh = namui::screen::size().map(|x| x.into_px());

        let scene_list = table::fixed(160.px(), |wh, ctx| {
            ctx.add(scene_list::SceneList {
                wh,
                scenes: &scenes,
            });
        });
        let scene_editor = table::ratio(1, |wh, ctx| {
            let Some(scene) = scene else { return };

            ctx.compose(|ctx| {
                table::vertical([
                    table::ratio(1, |wh, ctx| {
                        ctx.add(scene_preview::ScenePreview { wh, scene });
                    }),
                    table::fixed(160.px(), |wh, ctx| {
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
                        let text = texts
                            .get(&scene.id)
                            .and_then(|x| x.get("kor"))
                            .unwrap_or(&None)
                            .as_ref()
                            .unwrap_or(&empty_text);
                        ctx.add(text_editor::TextEditor {
                            wh,
                            text,
                            on_edit_done: on_text_edit_done,
                        });
                    }),
                ])(wh, ctx);
            });
        });
        let properties_panel = table::ratio(1, |wh, ctx| {});

        ctx.compose(|ctx| horizontal([scene_list, scene_editor, properties_panel])(wh, ctx));
    }
}

#[derive(Debug, Clone)]
enum EditActionForUndo {
    AddNewScene {
        id: String,
    },
    EditText {
        scene_id: String,
        language_code: String,
        prev_text: String,
    },
    UpdateScene {
        prev_scene: Scene,
    },
}
