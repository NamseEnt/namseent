use namui::prelude::*;

#[derive(Debug, Clone)]
pub struct ScrollView {
    pub id: String,
    pub scroll_y: Px,
}

pub struct Props {
    pub xy: Xy<Px>,
    pub scroll_bar_width: Px,
    pub height: Px,
    pub content: RenderingTree,
}

pub enum Event {
    Scrolled(String, Px),
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            id: namui::nanoid(),
            scroll_y: px(0.0),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Scrolled(id, scroll_y) => {
                    if id != &self.id {
                        return;
                    }
                    self.scroll_y = *scroll_y;
                }
            }
        }
    }
    pub fn render(&self, props: &Props) -> RenderingTree {
        let button_id = self.id.clone();
        let content_bounding_box = props.content.get_bounding_box();
        if content_bounding_box.is_none() {
            return RenderingTree::Empty;
        }
        let content_bounding_box = content_bounding_box.unwrap();

        let scroll_y = namui::math::num::clamp(
            self.scroll_y,
            px(0.0),
            px(0.0).max(content_bounding_box.height() - props.height),
        );

        let inner = namui::clip(
            namui::PathBuilder::new().add_rect(Rect::Ltrb {
                left: px(0.0),
                top: px(0.0),
                right: content_bounding_box.width(),
                bottom: props.height,
            }),
            namui::ClipOp::Intersect,
            namui::translate(px(0.0), -scroll_y, props.content.clone()),
        );

        let scroll_bar_handle_height =
            props.height * (props.height / content_bounding_box.height());

        let scroll_bar_y = (props.height - scroll_bar_handle_height)
            * (scroll_y / (content_bounding_box.height() - props.height));

        let scroll_bar = match content_bounding_box.height() > props.height {
            true => rect(RectParam {
                rect: Rect::Xywh {
                    x: content_bounding_box.width() - props.scroll_bar_width, // iOS Style!
                    y: scroll_bar_y,
                    width: props.scroll_bar_width,
                    height: scroll_bar_handle_height,
                },
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::grayscale_f01(0.5),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            false => RenderingTree::Empty,
        };
        let whole_rect = rect(RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: content_bounding_box.width(),
                height: props.height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::TRANSPARENT,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .attach_event(move |builder| {
            let width = content_bounding_box.width() + props.scroll_bar_width;
            let height = props.height;
            let button_id = button_id.clone();
            builder.on_wheel(move |event| {
                let mouse_position = namui::mouse::position();
                let whole_rect_xy = event
                    .namui_context
                    .get_rendering_tree_xy(event.target)
                    .unwrap();

                let is_mouse_in_timeline = mouse_position.x >= whole_rect_xy.x
                    && mouse_position.x <= whole_rect_xy.x + width
                    && mouse_position.y >= whole_rect_xy.y
                    && mouse_position.y <= whole_rect_xy.y + height;
                if !is_mouse_in_timeline {
                    return;
                }

                let next_scroll_y = namui::math::num::clamp(
                    scroll_y + px(event.delta_xy.y),
                    px(0.0),
                    (px(0.0)).max(content_bounding_box.height() - height),
                );

                namui::event::send(Event::Scrolled(button_id.clone(), next_scroll_y));
            });
        });
        translate(
            props.xy.x,
            props.xy.y,
            namui::render([whole_rect, inner, scroll_bar]),
        )
    }
}
