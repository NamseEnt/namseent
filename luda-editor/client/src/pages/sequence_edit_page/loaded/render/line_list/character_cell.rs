use super::*;
use namui_prebuilt::*;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn character_cell(
        &self,
        wh: Wh<Px>,
        cut: &Cut,
        characters: &Vec<Character>,
    ) -> RenderingTree {
        let character = cut
            .character_id
            .as_ref()
            .and_then(|id| characters.iter().find(|character| character.id().eq(id)));
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::BLACK),
            table::vertical([
                table::ratio(1.0, move |wh| {
                    let image_source = character.and_then(|_character| {
                        // TODO: Get main character image
                        None
                    });
                    match image_source {
                        Some(image_source) => namui::image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                            source: image_source,
                            style: ImageStyle {
                                fit: ImageFit::Contain,
                                paint_builder: None,
                            },
                        }),
                        None => RenderingTree::Empty,
                    }
                }),
                table::fixed(24.px(), move |wh| {
                    let character_name = character
                        .map(|character| character.name.as_ref())
                        .unwrap_or("".into());
                    typography::body::center(wh, character_name, Color::WHITE)
                }),
            ])(wh),
        ])
        .attach_event(move |builder| {
            let cut_id = cut.id();
            builder.on_mouse_down_in(move |event| {
                namui::event::send(Event::CharacterCellClicked {
                    cut_id,
                    global_xy: event.global_xy,
                });
            });
        })
    }
}
