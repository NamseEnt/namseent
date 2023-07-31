use crate::{simple_rect, typography::center_text_full_height};
use namui::prelude::*;

fn attach_text_button_event(
    button: RenderingTree,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl IntoClosure<dyn Fn(MouseEvent)>,
) -> RenderingTree {
    let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
    button.attach_event(|builder| {
        let on_mouse_up_in = on_mouse_up_in.into_arc();
        builder.on_mouse_up_in(move |event: MouseEvent| {
            let Some(button) = event.button else {
                return;
            };
            if mouse_buttons.contains(&button) {
                on_mouse_up_in(event);
            }
        });
    })
}

pub fn text_button(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl IntoClosure<dyn Fn(MouseEvent)>,
) -> namui::RenderingTree {
    attach_text_button_event(
        translate(
            rect.x(),
            rect.y(),
            render([
                simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
                center_text_full_height(rect.wh(), text, text_color),
            ]),
        ),
        mouse_buttons,
        on_mouse_up_in,
    )
}

pub fn text_button_fit(
    height: Px,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    side_padding: Px,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl IntoClosure<dyn Fn(MouseEvent)>,
) -> namui::RenderingTree {
    let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
    let center_text = center_text_full_height(Wh::new(0.px(), height), text, text_color);
    let width = match center_text.get_bounding_box() {
        Some(bounding_box) => bounding_box.width(),
        None => return RenderingTree::Empty,
    };

    attach_text_button_event(
        render([
            simple_rect(
                Wh::new(width + side_padding * 2, height),
                stroke_color,
                stroke_width,
                fill_color,
            ),
            translate(width / 2 + side_padding, 0.px(), center_text),
        ]),
        mouse_buttons,
        on_mouse_up_in,
    )
}

pub fn body_text_button(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    text_align: TextAlign,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl IntoClosure<dyn Fn(MouseEvent)>,
) -> namui::RenderingTree {
    attach_text_button_event(
        translate(
            rect.x(),
            rect.y(),
            render([
                simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
                match text_align {
                    TextAlign::Left => {
                        crate::typography::body::left(rect.wh().height, text, text_color)
                    }
                    TextAlign::Center => {
                        crate::typography::body::center(rect.wh(), text, text_color)
                    }
                    TextAlign::Right => crate::typography::body::right(rect.wh(), text, text_color),
                },
            ]),
        ),
        mouse_buttons,
        on_mouse_up_in,
    )
}
