use super::*;
use crate::storage::get_project_image_url;
use namui_prebuilt::*;
use rpc::data::*;

impl LoadedSequenceEditorPage {
    pub fn screen_editor(&self, wh: Wh<Px>, cut: &Cut) -> RenderingTree {
        render([
            simple_rect(wh, Color::WHITE, 1.px(), Color::BLACK),
            table::horizontal(
                cut.screen_image_ids
                    .iter()
                    .enumerate()
                    .map(|(index, image_id)| {
                        table::ratio(1.0, move |wh| {
                            let image_source = image_id.map(|image_id| {
                                get_project_image_url(self.project_id, image_id).unwrap()
                            });

                            render([
                                simple_rect(wh, Color::WHITE, 1.px(), Color::BLACK),
                                match image_source {
                                    Some(image_source) => namui::image(ImageParam {
                                        rect: Rect::from_xy_wh(Xy::single(0.px()), wh),
                                        source: namui::ImageSource::Url(image_source),
                                        style: ImageStyle {
                                            fit: ImageFit::Contain,
                                            paint_builder: None,
                                        },
                                    }),
                                    None => RenderingTree::Empty,
                                },
                            ])
                            .attach_event(move |builder| {
                                let cut_id = cut.id();
                                builder.on_mouse_down_in(move |_event| {
                                    namui::event::send(Event::ScreenEditorCellClicked {
                                        index,
                                        cut_id,
                                    });
                                });
                            })
                        })
                    }),
            )(wh),
        ])
    }
}
