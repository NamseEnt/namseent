use crate::*;
use audio_util::{get_or_load_audio, AudioLoadState};
use list_view::AutoListView;
use luda_rpc::*;
use std::collections::{HashMap, HashSet};
use time::now;

pub struct AudioSelectTool<'a> {
    pub wh: Wh<Px>,
    pub asset_docs: Sig<'a, HashMap<String, AssetDoc>>,
    pub selected_audio: &'a Option<SceneSound>,
    pub set_audio: &'a dyn Fn(Option<SceneSound>),
}

impl Component for AudioSelectTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            asset_docs,
            selected_audio,
            set_audio,
        } = self;

        let (selected_tags, set_selected_tags) =
            ctx.state::<HashSet<AssetSystemTag>>(Default::default);

        let on_select = |audio_id: Option<String>| {
            let audio = audio_id.map(|audio_id| SceneSound {
                sound_id: audio_id,
                volume: selected_audio
                    .as_ref()
                    .map(|selected_audio| selected_audio.volume)
                    .unwrap_or(100.percent()),
            });
            set_audio(audio);
        };

        let tag_filtered_asset_docs = ctx.memo(|| {
            asset_docs
                .iter()
                .filter(|(_id, asset_tag)| {
                    if !matches!(asset_tag.asset_kind, AssetKind::Audio) {
                        return false;
                    }
                    asset_tag.tags.iter().any(|tag| match tag {
                        AssetTag::System { tag } => selected_tags.contains(tag),
                        AssetTag::Custom { .. } => false,
                    })
                })
                .map(|(id, audio)| (id.clone(), audio.clone()))
                .collect::<HashMap<String, AssetDoc>>()
        });

        let tag_toggle_button = |tag: AssetSystemTag| {
            let is_on = selected_tags.contains(&tag);
            let text = match tag {
                AssetSystemTag::AudioCharacter => "인물",
                AssetSystemTag::AudioProp => "사물",
                AssetSystemTag::AudioBackground => "배경",
                _ => unreachable!(),
            };

            table::ratio(1, move |wh, ctx| {
                ctx.add(simple_toggle_button(wh, text, is_on, |_| {
                    set_selected_tags.mutate(move |selected_tags| {
                        if selected_tags.contains(&tag) {
                            selected_tags.remove(&tag);
                        } else {
                            selected_tags.insert(tag);
                        }
                    });
                }));
            })
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(
                    64.px(),
                    table::horizontal([
                        table::fixed(64.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::AudioCharacter),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::AudioProp),
                        table::fixed(16.px(), |_, _| {}),
                        tag_toggle_button(AssetSystemTag::AudioBackground),
                        table::fixed(64.px(), |_, _| {}),
                    ]),
                ),
                table::ratio(1, |wh, ctx| {
                    ctx.add(AudioList {
                        wh,
                        asset_docs: tag_filtered_asset_docs,
                        selected_audio,
                        on_select: &on_select,
                    });
                }),
            ])(wh, ctx)
        });
    }
}

struct AudioList<'a> {
    wh: Wh<Px>,
    asset_docs: Sig<'a, HashMap<String, AssetDoc>>,
    selected_audio: &'a Option<SceneSound>,
    on_select: &'a dyn Fn(Option<String>),
}
impl Component for AudioList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            asset_docs,
            selected_audio,
            on_select,
        } = self;

        let item_wh = Wh::new(wh.width, 48.px());
        let render_item = |text: String, audio_id: Option<String>| {
            let is_on = selected_audio
                .as_ref()
                .map(|selected_audio| &selected_audio.sound_id)
                .eq(&audio_id.as_ref());

            (
                audio_id.clone().unwrap_or_default(),
                AudioListItem {
                    wh: item_wh,
                    audio_id,
                    text,
                    is_on,
                    on_select,
                },
            )
        };

        let mut items = vec![render_item("없음".to_string(), None)];
        items.extend(asset_docs.values().filter_map(|asset_doc| {
            let AssetKind::Audio = asset_doc.asset_kind else {
                return None;
            };
            Some(render_item(
                asset_doc.name.to_string(),
                Some(asset_doc.id.clone()),
            ))
        }));

        ctx.add(AutoListView {
            height: wh.height,
            scroll_bar_width: 10.px(),
            item_wh,
            items: items.into_iter(),
        });
    }
}

struct AudioListItem<'a> {
    wh: Wh<Px>,
    audio_id: Option<String>,
    text: String,
    is_on: bool,
    on_select: &'a dyn Fn(Option<String>),
}
impl Component for AudioListItem<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            audio_id,
            text,
            is_on,
            on_select,
        } = self;

        let audio = audio_id.clone().map(get_or_load_audio);
        let (hovering, set_hovering) = ctx.state::<Option<Hovering>>(|| None);
        let (play_handle, set_play_handle) = ctx.state(|| None);

        ctx.interval("play audio if hovering", 1.sec(), |_| {
            let Some((Hovering { started_at }, audio)) =
                hovering.as_ref().as_ref().zip(audio.as_ref())
            else {
                return;
            };
            namui::log!("{:?}", started_at);
            if play_handle.is_some() {
                return;
            }
            if now() - started_at < 1.sec() {
                return;
            }
            let AudioLoadState::Loaded { audio } = audio.as_ref() else {
                return;
            };
            let play_handle = audio.play_repeat();
            set_play_handle.set(Some(play_handle));
        });

        ctx.add(
            simple_toggle_button(wh, text, is_on, |_| {
                on_select(audio_id);
            })
            .attach_event(|event| {
                let Event::MouseMove { event } = event else {
                    return;
                };
                match hovering.is_some() {
                    true => {
                        if event.is_local_xy_in() {
                            return;
                        }
                        set_hovering.set(None);
                        set_play_handle.set(None);
                    }
                    false => {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        set_hovering.set(Some(Hovering { started_at: now() }));
                    }
                }
            }),
        );
    }
}
struct Hovering {
    started_at: Instant,
}
