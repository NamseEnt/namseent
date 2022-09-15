mod draw_caret;
mod draw_texts_divided_by_selection;
mod get_selection_on_keyboard_down;
mod get_selection_on_mouse_down;
mod selection_index;

use crate::{
    namui::{self, *},
    text::{get_fallback_fonts, LineTexts},
    RectParam,
};
use std::ops::Range;

pub type Selection = Option<Range<usize>>;

#[derive(Clone, Debug)]
pub struct TextInput {
    pub(crate) selection: Selection,
    pub(crate) id: String,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub rect: Rect<Px>,
    pub rect_style: RectStyle,
    pub text: String,
    pub text_align: TextAlign,
    pub text_baseline: TextBaseline,
    pub font_type: FontType,
    pub text_style: TextStyle,
}
#[derive(Clone)]
pub struct TextInputCustomData {
    pub text_input: TextInput,
    pub props: Props,
}
pub enum Event {
    Focus {
        id: String,
        selection: Selection,
    },
    Blur {
        id: String,
    },
    TextUpdated {
        id: String,
        text: String,
        selection: Selection,
    },
    SelectionUpdated {
        id: String,
        selection: Selection,
    },
    KeyDown {
        id: String,
        code: Code,
    },
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            selection: None,
            id: crate::nanoid(),
        }
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl TextInput {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let font = namui::font::get_font(props.font_type);
        if font.is_none() {
            return RenderingTree::Empty;
        }
        let font = font.unwrap();

        let fonts = std::iter::once(font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(font.size)).flatten())
            .collect::<Vec<_>>();

        let paint = get_text_paint(props.text_style.color).build();

        let line_texts = LineTexts::new(&props.text, &fonts, &paint, Some(props.rect.width()));

        let custom_data = TextInputCustomData {
            text_input: self.clone(),
            props: props.clone(),
        };
        render([
            namui::rect(RectParam {
                rect: props.rect,
                style: props.rect_style,
            }),
            self.draw_texts_divided_by_selection(&props, &fonts, &paint, &line_texts),
            self.draw_caret(&props, &line_texts),
        ])
        .with_custom(custom_data.clone())
        .attach_event(|builder| {
            let custom_data = custom_data.clone();
            builder.on_mouse_down_in(move |event| {
                system::text_input::on_mouse_down_in_at_attach_event_calls(
                    event.local_xy,
                    &custom_data,
                )
            });
        })
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Focus { id, selection } => {
                    if self.id.eq(id) {
                        self.selection = selection.clone();
                    } else {
                        self.selection = None; // TODO: Remove this and draw unfocus caret in different way
                    }
                }
                Event::Blur { id } => {
                    if self.id.eq(id) {
                        self.selection = None; // TODO: Remove this and draw unfocus caret in different way
                    }
                }
                Event::SelectionUpdated { id, selection } => {
                    if self.id.eq(id) {
                        self.selection = selection.clone();
                    }
                }
                Event::TextUpdated { id, selection, .. } => {
                    if self.id.eq(id) {
                        self.selection = selection.clone();
                    }
                }
                Event::KeyDown { .. } => {}
            }
        }
    }
    pub fn is_focused(&self) -> bool {
        crate::system::text_input::is_focused(&self.id)
    }
    pub fn focus(&self) {
        crate::system::text_input::focus(&self.id)
    }
}

impl Props {
    pub fn text_param(&self) -> TextParam {
        TextParam {
            text: self.text.clone(),
            x: self.text_x(),
            y: self.text_y(),
            align: self.text_align,
            baseline: self.text_baseline,
            font_type: self.font_type,
            style: self.text_style,
            max_width: Some(self.rect.width()),
        }
    }
    pub fn text_x(&self) -> Px {
        match self.text_align {
            TextAlign::Left => self.rect.left(),
            TextAlign::Center => self.rect.center().x,
            TextAlign::Right => self.rect.right(),
        }
    }

    pub fn text_y(&self) -> Px {
        match self.text_baseline {
            TextBaseline::Top => self.rect.top(),
            TextBaseline::Middle => self.rect.center().y,
            TextBaseline::Bottom => self.rect.bottom(),
        }
    }
}
