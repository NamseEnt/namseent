use super::*;
use namui_prebuilt::{button::text_button, *};

impl Timeline {
    pub fn render_context_menu(&self) -> namui::RenderingTree {
        if self.context_menu.is_none() {
            return RenderingTree::Empty;
        }
        absolute(
            0.px(),
            0.px(),
            match self.context_menu.as_ref().unwrap() {
                ContextMenu::ImageClip { global_xy: xy } => {
                    let menu_item_wh = Wh::new(100.px(), 20.px());
                    text_button(
                        Rect::from_xy_wh(*xy, menu_item_wh),
                        "New image clip",
                        Color::WHITE,
                        Color::WHITE,
                        1.px(),
                        Color::BLACK,
                        || {
                            namui::event::send(Event::NewImageClip);
                            namui::event::send(Event::CloseContextMenu);
                        },
                    )
                }
            },
        )
        .attach_event(|builder| {
            builder.on_mouse_down_out(|_event| {
                namui::event::send(Event::CloseContextMenu);
            });
        })
    }
}
