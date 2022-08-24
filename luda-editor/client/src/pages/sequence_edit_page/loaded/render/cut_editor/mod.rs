use super::*;
use crate::storage::*;
use namui_prebuilt::*;

impl SequenceEditPage {
    pub fn render_cut_editor(&self, wh: Wh<Px>, cut: &Cut) -> namui::RenderingTree {
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            table::horizontal([
                table::ratio(
                    3.0,
                    table::vertical([
                        table::ratio(
                            4.0,
                            table::horizontal([
                                table::ratio(4.0, |wh| {
                                    let image_clip =
                                        self.selected_layer_index.and_then(|layer_index| {
                                            cut.image_clips.iter().nth(layer_index)
                                        });
                                    match image_clip {
                                        Some(image_clip) => {
                                            let image_clip_address = ImageClipAddress {
                                                sequence_id: self.selected_sequence_id.clone(),
                                                cut_id: cut.id().to_string(),
                                                image_clip_id: image_clip.id().to_string(),
                                            };
                                            self.wysiwyg_editor.render(wysiwyg_editor::Props {
                                                wh,
                                                image_clip,
                                                selected_layer_index: self.selected_layer_index,
                                                image_clip_address: &image_clip_address,
                                            })
                                        }
                                        None => RenderingTree::Empty,
                                    }
                                }),
                                table::ratio(
                                    1.0,
                                    table::vertical([
                                        table::ratio(1.0, |wh| {
                                            // TODO: script
                                            RenderingTree::Empty
                                        }),
                                        table::ratio(1.0, |wh| RenderingTree::Empty),
                                    ]),
                                ),
                            ]),
                        ),
                        table::ratio(1.0, |wh| {
                            self.timeline
                                .as_ref()
                                .unwrap()
                                .render(timeline::Props { wh, cut })
                        }),
                    ]),
                ),
                table::ratio(1.0, |wh| {
                    self.property_editor.render(property_editor::Props {
                        wh,
                        cut,
                        selected_layer_index: self.selected_layer_index,
                        selected_sequence_id: &self.selected_sequence_id,
                        selected_cut_id: cut.id(),
                    })
                }),
            ])(wh),
        ])
    }
}
