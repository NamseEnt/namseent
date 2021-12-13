use super::{text_input_event, TextInput};
use crate::{
    namui::{self, translate},
    render,
};
mod draw_texts_divided_by_selection;
use draw_texts_divided_by_selection::draw_texts_divided_by_selection;
mod get_selection_on_click;
use get_selection_on_click::get_selection_on_click;

impl namui::Entity for TextInput {
    type Props = ();
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let text_input = self.clone();
        translate(
            self.x,
            self.y,
            render![
                namui::rect(namui::RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: self.width,
                    height: self.height,
                    id: None,
                    on_click: Some(Box::new(move |xy| {
                        namui::log(format!(
                            "text_input click {} {:?}",
                            text_input.id.clone(),
                            xy
                        ));
                        let selection = get_selection_on_click(xy.x, &text_input);
                        namui::event::send(Box::new(namui::text_input_event::SelectionChanged {
                            id: text_input.id.clone(),
                            selection: selection.ok(),
                        }));
                    })),
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
                    }
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
