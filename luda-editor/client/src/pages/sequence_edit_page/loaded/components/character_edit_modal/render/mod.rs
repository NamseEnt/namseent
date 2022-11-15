mod character;

use super::*;

#[derive(Debug)]
enum WithAddButton<'a, T> {
    Item(&'a T),
    AddButton,
}

impl CharacterEditModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let character_list_rect = Rect::from_xy_wh(Xy::zero(), Wh::new(80.px(), props.wh.height));

        let character_list_view = self.render_character_list_view(&props, character_list_rect);
        let context_menu = match self.context_menu.as_ref() {
            Some(context_menu) => context_menu.render(),
            None => RenderingTree::Empty,
        };

        on_top(absolute(
            self.global_xy.x,
            self.global_xy.y,
            event_trap(
                render([character_list_view, context_menu]).attach_event(|builder| {
                    builder.on_mouse_down_out(|event| {
                        event.stop_propagation();
                        namui::event::send(Event::Close)
                    });
                }),
            ),
        ))
    }
}
