mod draw_caret;
mod draw_texts_divided_by_selection;

use crate::{
    namui::{self, *},
    text::{get_fallback_fonts, LineTexts},
    RectParam,
};
use std::{
    ops::Range,
    sync::atomic::{AtomicBool, Ordering},
};

pub type Selection = Option<Range<usize>>;

#[derive(Clone, Debug)]
pub struct TextInput {
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
    pub event_handler: Option<EventHandler>,
}
#[derive(Clone, Default)]
pub struct EventHandler {
    pub(crate) on_key_down: Option<Arc<dyn Fn(KeyDownEvent) + 'static>>,
}
unsafe impl Send for EventHandler {}
unsafe impl Sync for EventHandler {}

impl EventHandler {
    pub fn new() -> Self {
        Self { on_key_down: None }
    }
    pub fn on_key_down(mut self, on_key_down: impl Fn(KeyDownEvent) + 'static) -> Self {
        self.on_key_down = Some(Arc::new(on_key_down));
        self
    }
}
impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventHandler")
            .field("on_key_down", &self.on_key_down.is_some())
            .finish()
    }
}

pub struct KeyDownEvent {
    pub code: Code,
    pub(crate) is_prevented_default: Arc<AtomicBool>,
    pub is_composing: bool,
}

impl KeyDownEvent {
    pub fn prevent_default(&self) {
        self.is_prevented_default.store(true, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct TextInputCustomData {
    pub id: String,
    pub props: Props,
}
pub enum Event {
    Focus { id: String, selection: Selection },
    Blur { id: String },
    TextUpdated { id: String, text: String },
    SelectionUpdated { id: String, selection: Selection },
    KeyDown { id: String, code: Code },
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
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

        let selection = crate::system::text_input::get_selection(&self.id, &props.text);

        let custom_data = TextInputCustomData {
            id: self.id.clone(),
            props: props.clone(),
        };
        render([
            namui::rect(RectParam {
                rect: props.rect,
                style: props.rect_style,
            }),
            self.draw_texts_divided_by_selection(&props, &fonts, &paint, &line_texts, &selection),
            self.draw_caret(&props, &line_texts, &selection),
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
