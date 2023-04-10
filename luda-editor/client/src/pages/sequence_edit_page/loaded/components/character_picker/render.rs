use super::*;
use crate::color;
use namui_prebuilt::*;

impl CharacterPicker {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_character_list(props),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        let background = simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND);
        background.attach_event(|builder| {
            builder
                .on_mouse_down_out(|_| namui::event::send(Event::MouseDownOutsideCharacterPicker));
        })
    }

    fn render_character_list(&self, props: Props) -> namui::RenderingTree {
        const CHARACTER_THUMBNAIL_WH: Wh<Px> = Wh {
            width: px(96.0),
            height: px(144.0),
        };
        const PADDING: Px = px(8.0);

        table::padding(PADDING, |wh| {
            let max_items_per_row =
                (props.wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
            table::vertical(self.pose_files.chunks(max_items_per_row).map(|pose_files| {
                table::fixed(CHARACTER_THUMBNAIL_WH.height, |wh| {
                    table::horizontal(pose_files.iter().map(|pose_file| {
                        table::fixed(CHARACTER_THUMBNAIL_WH.width, |wh| {
                            table::padding(PADDING, |wh| {
                                render([
                                    image(ImageParam {
                                        rect: wh.to_rect(),
                                        source: ImageSource::Url(pose_file.thumbnail_url()),
                                        style: ImageStyle {
                                            fit: ImageFit::Contain,
                                            paint_builder: None,
                                        },
                                    })
                                    .with_mouse_cursor(MouseCursor::Pointer),
                                    simple_rect(
                                        wh,
                                        color::STROKE_NORMAL,
                                        1.px(),
                                        Color::TRANSPARENT,
                                    ),
                                ])
                            })(wh)
                        })
                    }))(wh)
                })
            }))(wh)
        })(props.wh)
    }
}
