use super::*;
use namui::animation::ImageInterpolation;
use std::str::FromStr;

impl LineEditWindow {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let border = simple_rect(props.wh, Color::BLACK, 1.px(), Color::WHITE);

        let content = match (props.selected_layer, props.editing_target) {
            (Some(layer), Some(EditingTarget::Line { point_id, layer_id })) => {
                let (_, line) = layer
                    .image
                    .image_keyframe_graph
                    .get_point_and_line(&point_id)
                    .unwrap();

                self.dropdown.render(dropdown::Props {
                    rect: Rect::Xywh {
                        x: 0.px(),
                        y: 0.px(),
                        width: props.wh.width,
                        height: 40.px(),
                    },
                    items: ImageInterpolation::iter().map(|interpolation| dropdown::Item {
                        id: interpolation.as_ref().to_string(),
                        text: interpolation.as_ref().to_string(),
                        is_selected: interpolation.as_ref().eq(line.as_ref()),
                    }),
                    visible_item_count: 0,
                    on_select_item: move |item_id| {
                        namui::event::send(Event::SelectItem {
                            line: ImageInterpolation::from_str(&item_id).unwrap(),
                            layer_id: layer_id.clone(),
                            point_id: point_id.clone(),
                        });
                    },
                })
            }
            _ => RenderingTree::Empty,
        };

        render([border, content])
    }
}
