mod draw_caret;
mod draw_texts_divided_by_selection;

use crate::{
    namui::{self, *},
    system::text_input::Selection,
    text::LineTexts,
    RectParam,
};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Debug)]
pub struct TextInput {
    pub(crate) id: Uuid,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub rect: Rect<Px>,
    pub text: String,
    pub text_align: TextAlign,
    pub text_baseline: TextBaseline,
    pub font_type: FontType,
    pub style: Style,
    pub event_handler: Option<EventHandler>,
}
#[derive(Clone, Default)]
pub struct EventHandler {
    pub(crate) on_key_down: Option<Arc<dyn Fn(KeyDownEvent) + 'static>>,
    pub(crate) on_text_updated: Option<Arc<dyn Fn(&str) + 'static>>,
}
unsafe impl Send for EventHandler {}
unsafe impl Sync for EventHandler {}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            on_key_down: None,
            on_text_updated: None,
        }
    }
    pub fn on_key_down(mut self, on_key_down: impl Fn(KeyDownEvent) + 'static) -> Self {
        self.on_key_down = Some(Arc::new(on_key_down));
        self
    }
    pub fn on_text_updated(mut self, on_text_updated: impl Fn(&str) + 'static) -> Self {
        self.on_text_updated = Some(Arc::new(on_text_updated));
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

#[derive(Clone, Debug)]
pub struct Style {
    pub rect: RectStyle,
    pub text: TextStyle,
    pub padding: Ltrb<Px>,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            rect: RectStyle::default(),
            text: TextStyle::default(),
            padding: Ltrb {
                left: 4.px(),
                top: 4.px(),
                right: 4.px(),
                bottom: 4.px(),
            },
        }
    }
}

pub struct KeyDownEvent {
    pub code: Code,
    pub(crate) is_prevented_default: Arc<AtomicBool>,
    pub is_composing: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct CursorPosition {
    pub is_at_top: bool,
    pub is_at_bottom: bool,
}

impl KeyDownEvent {
    pub fn prevent_default(&self) {
        self.is_prevented_default.store(true, Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct TextInputCustomData {
    pub id: Uuid,
    pub props: Props,
}
pub enum Event {
    Focus {
        id: crate::Uuid,
    },
    Blur {
        id: crate::Uuid,
    },
    TextUpdated {
        id: crate::Uuid,
        text: String,
    },
    SelectionUpdated {
        id: crate::Uuid,
        selection: Selection,
    },
    KeyDown {
        id: crate::Uuid,
        code: Code,
    },
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput { id: crate::uuid() }
    }
    pub fn get_id(&self) -> crate::Uuid {
        self.id
    }
}

impl TextInput {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let font = namui::font::get_font(props.font_type);
        if font.is_none() {
            return RenderingTree::Empty;
        }
        let font = font.unwrap();

        let fonts = crate::font::with_fallbacks(font);

        let paint = get_text_paint(props.style.text.color).build();

        let line_texts = LineTexts::new(
            &props.text,
            fonts.clone(),
            paint.clone(),
            props.text_param().max_width,
        );

        let selection = crate::system::text_input::get_selection(self.id, &props.text);

        let custom_data = TextInputCustomData {
            id: self.id.clone(),
            props: props.clone(),
        };
        render([
            namui::rect(RectParam {
                rect: props.rect,
                style: RectStyle {
                    stroke: if props.style.rect.stroke.is_some() || props.style.rect.fill.is_some()
                    {
                        props.style.rect.stroke
                    } else {
                        Some(RectStroke {
                            color: Color::TRANSPARENT,
                            width: 0.px(),
                            border_position: BorderPosition::Inside,
                        })
                    },
                    ..props.style.rect
                },
            }),
            self.draw_texts_divided_by_selection(
                &props,
                &fonts,
                paint.clone(),
                &line_texts,
                &selection,
            ),
            self.draw_caret(&props, &line_texts, &selection, paint.clone()),
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
        crate::system::text_input::is_focused(self.id)
    }
    pub fn focus(&self) {
        crate::system::text_input::focus(self.id)
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
            style: self.style.text.clone(),
            max_width: Some(self.rect.width() - self.style.padding.left - self.style.padding.right),
        }
    }
    pub fn text_x(&self) -> Px {
        match self.text_align {
            TextAlign::Left => self.rect.left() + self.style.padding.left,
            TextAlign::Center => self.rect.center().x,
            TextAlign::Right => self.rect.right() - self.style.padding.right,
        }
    }

    pub fn text_y(&self) -> Px {
        match self.text_baseline {
            TextBaseline::Top => self.rect.top() + self.style.padding.top,
            TextBaseline::Middle => self.rect.center().y,
            TextBaseline::Bottom => self.rect.bottom() - self.style.padding.bottom,
        }
    }
    pub fn line_height_px(&self) -> Px {
        self.font_type.size.into_px() * self.style.text.line_height_percent
    }
}
