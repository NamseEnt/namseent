use super::{scene_audio_editor::SceneAudioEditor, scene_sprite_editor::SceneSpriteEditor};
use luda_rpc::{AssetDoc, EpisodeEditAction, Scene};
use namui::*;
use namui_prebuilt::{button, table::*};
use std::collections::HashMap;

static PROPERTIES_PANEL_TAB_ATOM: Atom<PropertiesPanelTab> = Atom::uninitialized();

pub struct PropertiesPanel<'a> {
    pub wh: Wh<Px>,
    pub scene: &'a Scene,
    pub edit_episode: &'a dyn Fn(EpisodeEditAction),
    pub asset_docs: Sig<'a, HashMap<String, AssetDoc>>,
}
impl Component for PropertiesPanel<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            scene,
            asset_docs,
            edit_episode,
        } = self;

        let (properties_panel_tab, set_properties_panel_tab) =
            ctx.init_atom(&PROPERTIES_PANEL_TAB_ATOM, || PropertiesPanelTab::Standing);

        let update_scene = &|scene: Scene| {
            edit_episode(EpisodeEditAction::UpdateScene { scene });
        };

        ctx.compose(|ctx| {
            vertical([
                fixed(
                    48.px(),
                    horizontal([
                        render_tab_button(
                            "스탠딩",
                            matches!(*properties_panel_tab, PropertiesPanelTab::Standing),
                            || {
                                set_properties_panel_tab.set(PropertiesPanelTab::Standing);
                            },
                        ),
                        render_tab_button(
                            "배경",
                            matches!(*properties_panel_tab, PropertiesPanelTab::Background),
                            || {
                                set_properties_panel_tab.set(PropertiesPanelTab::Background);
                            },
                        ),
                        render_tab_button(
                            "오디오",
                            matches!(*properties_panel_tab, PropertiesPanelTab::Audio),
                            || {
                                set_properties_panel_tab.set(PropertiesPanelTab::Audio);
                            },
                        ),
                    ]),
                ),
                ratio(1, |wh, ctx| match properties_panel_tab.as_ref() {
                    PropertiesPanelTab::Standing => {
                        ctx.add(SceneSpriteEditor {
                            wh,
                            scene,
                            update_scene,
                            asset_docs,
                        });
                    }
                    PropertiesPanelTab::Background => {}
                    PropertiesPanelTab::Audio => {
                        ctx.add(SceneAudioEditor {
                            wh,
                            scene,
                            update_scene,
                            asset_docs,
                        });
                    }
                }),
            ])(wh, ctx);
        });
    }
}

pub enum PropertiesPanelTab {
    Standing,
    Background,
    Audio,
}

fn render_tab_button<'a>(
    text: &'a str,
    selected: bool,
    on_click: impl 'a + FnOnce(),
) -> TableCell<'a> {
    TableCell::Some {
        unit: Unit::Ratio(1.0),
        render: Box::new(move |_direction, wh, ctx| {
            let (text_color, fill_color) = match selected {
                true => (Color::WHITE, Color::BLUE),
                false => (Color::BLUE, Color::TRANSPARENT),
            };

            ctx.add(button::TextButton {
                rect: wh.to_rect(),
                text: text.to_string(),
                text_color,
                stroke_color: Color::BLUE,
                stroke_width: 1.px(),
                fill_color,
                mouse_buttons: vec![MouseButton::Left],
                on_mouse_up_in: move |_| {
                    on_click();
                },
            });
        }),
        need_clip: true,
    }
}
