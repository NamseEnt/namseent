use super::{text_input_event, TextInput};
use crate::{
    namui::{self, translate},
    render,
};
mod draw_texts_divided_by_selection;
use draw_texts_divided_by_selection::draw_texts_divided_by_selection;
mod get_selection_on_mouse_down;
use get_selection_on_mouse_down::get_selection_on_mouse_down;

impl namui::Entity for TextInput {
    type Props = ();
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        translate(
            self.x,
            self.y,
            render![
                namui::rect(namui::RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: self.width,
                    height: self.height,
                    style: namui::RectStyle {
                        stroke: Some(namui::RectStroke {
                            color: self.border_color,
                            width: self.border_width,
                            border_position: namui::BorderPosition::Inside
                        }),
                        fill: Some(namui::RectFill {
                            color: self.background_fill_color
                        }),
                        round: None,
                    },
                    ..Default::default()
                })
                .attach_event(|builder| {
                    let text_input = self.clone();

                    builder.on_mouse_down(Box::new(move |event| {
                        namui::log(format!(
                            "text_input click {} {:?}",
                            text_input.id.clone(),
                            event.global_xy
                        ));
                        let selection = get_selection_on_mouse_down(event.local_xy.x, &text_input);
                        namui::event::send(Box::new(namui::text_input_event::SelectionChanged {
                            id: text_input.id.clone(),
                            selection: selection.ok(),
                        }));
                    }))
                }),
                draw_texts_divided_by_selection(&self)
            ],
        )
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<text_input_event::SelectionChanged>() {
            if event.id != self.id {
                return;
            }

            self.selection = event.selection;
            namui::log(format!("selection changed: {:?}", self.selection));
        }
    }
}
