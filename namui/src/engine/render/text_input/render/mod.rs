use super::TextInput;
use crate::{
    engine::{self, translate},
    render,
};
mod draw_texts_divided_by_selection;
use draw_texts_divided_by_selection::draw_texts_divided_by_selection;
mod get_selection_on_click;
use get_selection_on_click::get_selection_on_click;

impl engine::Render for TextInput {
    fn render(&self) -> engine::RenderingTree {
        let text_input = self.clone();
        translate(
            self.x,
            self.y,
            render![
                engine::rect(engine::RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: self.width,
                    height: self.height,
                    id: None,
                    on_click: Some(Box::new(move |xy| {
                        engine::log(format!(
                            "text_input click {} {:?}",
                            text_input.id.clone(),
                            xy
                        ));
                        let selection = get_selection_on_click(xy.x, &text_input);
                        engine::event::send(Box::new(engine::text_input_event::SelectionChanged {
                            id: text_input.id.clone(),
                            selection: selection.ok(),
                        }));
                    })),
                    style: engine::RectStyle {
                        stroke: Some(engine::RectStroke {
                            color: self.border_color,
                            width: self.border_width,
                            border_position: engine::BorderPosition::Inside
                        }),
                        fill: Some(engine::RectFill {
                            color: self.background_fill_color
                        }),
                        round: None,
                    }
                }),
                draw_texts_divided_by_selection(&self)
            ],
        )
    }
}
