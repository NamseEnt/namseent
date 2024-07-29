use crate::*;
use luda_rpc::Scene;
use namui::*;
use namui_prebuilt::*;
use std::collections::BTreeMap;

pub struct SpeakerSelector<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub project_id: &'a String,
    pub episode_id: &'a String,
    pub select_speaker: &'a dyn Fn(&String),
}

impl Component for SpeakerSelector<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            project_id,
            episode_id,
            select_speaker,
        } = self;
        let (speaker_ids, set_speaker_ids) = ctx.state::<Option<Vec<String>>>(|| None);
        let (speakers, set_speakers) =
            ctx.state::<Option<BTreeMap<String, Option<String>>>>(|| None);
        let (error_msg, set_error_msg) = ctx.state::<Option<String>>(|| None);

        ctx.async_effect(
            "load speaker ids",
            episode_id,
            move |episode_id| async move {
                use crate::rpc::episode_editor::load_speaker_slots::*;
                match server_connection()
                    .load_speaker_slots(RefRequest {
                        episode_id: &episode_id,
                    })
                    .await
                {
                    Ok(response) => {
                        set_speaker_ids.set(Some(response.speaker_ids));
                    }
                    Err(err) => {
                        set_error_msg.set(Some(err.to_string()));
                    }
                }
            },
        );

        ctx.async_effect(
            "load speaker names",
            (speaker_ids, project_id),
            move |(speaker_ids, project_id)| async move {
                set_speakers.set(None);
                let Some(speaker_ids) = speaker_ids else {
                    return;
                };
                use crate::rpc::episode_editor::get_speaker_names::*;
                match server_connection()
                    .get_speaker_names(RefRequest {
                        language_code: "kor",
                        project_id: &project_id,
                        speaker_ids: &speaker_ids,
                    })
                    .await
                {
                    Ok(response) => {
                        set_speakers.set(Some(
                            speaker_ids
                                .into_iter()
                                .zip(response.speaker_names)
                                .collect(),
                        ));
                    }
                    Err(err) => {
                        set_error_msg.set(Some(err.to_string()));
                    }
                }
            },
        );

        let Some(speakers) = speakers.as_ref() else {
            return;
        };

        ctx.compose(|ctx| {
            let items = {
                let mut items = vec![];
                for (speaker_id, name) in speakers {
                    items.push(table::fixed(16.px(), |_wh, _ctx| {}));
                    items.push(table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(button::TextButtonFit {
                            height: wh.height,
                            text: name.as_ref().unwrap_or(&"N/A".to_string()),
                            text_color: Color::WHITE,
                            stroke_color: Color::WHITE,
                            stroke_width: 1.px(),
                            fill_color: Color::BLACK,
                            side_padding: 8.px(),
                            mouse_buttons: vec![MouseButton::Left],
                            on_mouse_up_in: &|_event| {
                                select_speaker(speaker_id);
                            },
                        });
                    }));
                }

                items.push(table::ratio(1, |_wh, _ctx| {}));
                items.push(table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(button::TextButtonFit {
                        height: wh.height,
                        text: "설정",
                        text_color: Color::WHITE,
                        stroke_color: Color::WHITE,
                        stroke_width: 1.px(),
                        fill_color: Color::BLACK,
                        side_padding: 8.px(),
                        mouse_buttons: vec![MouseButton::Left],
                        on_mouse_up_in: &|_event| todo!(),
                    });
                }));

                items
            };
            table::horizontal(items)(wh, ctx)
        });
    }
}
