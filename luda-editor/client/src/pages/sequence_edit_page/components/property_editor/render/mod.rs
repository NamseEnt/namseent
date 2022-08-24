mod layer_list;

use super::*;
use namui_prebuilt::*;

impl PropertyEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            simple_rect(props.wh, Color::WHITE, 1.px(), Color::TRANSPARENT),
            match &self.content {
                PropertyContent::Nothing => RenderingTree::Empty,
                PropertyContent::ImageClip {
                    image_clip_address,
                    layer_list_view,
                } => table::vertical([
                    table::ratio(4.0, |_wh| RenderingTree::Empty),
                    table::ratio(4.0, |wh| {
                        self.render_layer_list(
                            &props,
                            wh,
                            &image_clip_address.image_clip_id,
                            layer_list_view,
                        )
                    }),
                ])(props.wh),
                PropertyContent::ImageLayer {
                    image_clip_address,
                    image_browser,
                    layer_list_view,
                    layer_index,
                } => table::vertical([
                    table::ratio(4.0, |wh| {
                        let selected_resource = self
                            .editor_history_system
                            .get_image_layer_image_path(image_clip_address, *layer_index);

                        image_browser.render(image_browser::Props {
                            wh,
                            selected_resource,
                        })
                    }),
                    table::ratio(4.0, |wh| {
                        self.render_layer_list(
                            &props,
                            wh,
                            &image_clip_address.image_clip_id,
                            layer_list_view,
                        )
                    }),
                ])(props.wh),
            },
        ])
    }
}
