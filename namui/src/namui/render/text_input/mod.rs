mod draw_caret;
mod draw_texts_divided_by_selection;
mod instance;

use crate::{
    namui::{self, *},
    system::text_input::*,
    text::LineTexts,
    web::SelectionDirection,
    RectParam,
};
pub use instance::*;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
#[component]
pub struct TextInput {
    pub instance: TextInputInstance,
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
    pub(crate) on_key_down: Option<ClosurePtr<KeyDownEvent, ()>>,
    pub(crate) on_text_updated: Option<ClosurePtr<String, ()>>,
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
    pub fn on_key_down(mut self, on_key_down: impl Into<ClosurePtr<KeyDownEvent, ()>>) -> Self {
        self.on_key_down = Some(on_key_down.into());
        self
    }
    pub fn on_text_updated(mut self, on_text_updated: impl Into<ClosurePtr<String, ()>>) -> Self {
        self.on_text_updated = Some(on_text_updated.into());
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
    pub fn is_prevented_default(&self) -> bool {
        self.is_prevented_default.load(Ordering::Relaxed)
    }
}

#[derive(Clone, Debug)]
pub struct TextInputCustomData {
    pub id: Uuid,
    pub props: TextInput,
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
    pub fn get_id(&self) -> crate::Uuid {
        todo!()
        // self.id
    }
}

impl Component for TextInput {
    fn render<'a>(&'a self, ctx: RenderCtx<'a>) -> RenderDone {
        // let id = ctx.use_memo(|| self.id.unwrap_or_else(|| namui::uuid()));
        let id = self.instance.id;
        let (atom, set_atom) = ctx.use_atom(&TEXT_INPUT_ATOM);

        ctx.use_web_event(|web_event| match web_event {
            web::WebEvent::MouseDown {
                x,
                y,
                button,
                buttons,
            } => {}
            web::WebEvent::MouseMove {
                x,
                y,
                button,
                buttons,
            } => {}
            web::WebEvent::MouseUp {
                x,
                y,
                button,
                buttons,
            } => {}
            web::WebEvent::Wheel {
                x,
                y,
                delta_x,
                delta_y,
            } => {}
            web::WebEvent::HashChange { new_url, old_url } => {}
            &web::WebEvent::SelectionChange {
                selection_direction,
                selection_start,
                selection_end,
                ref text,
            } => {
                todo!("내가 포커스면 아래를 처리해라. 아니면 무시하고.");
                // let atom = TEXT_INPUT_ATOM.get();
                let Some(last_focused_text_input) = &atom.last_focused_text_input else {
                    return;
                };
                let selection = get_input_element_selection(
                    selection_direction,
                    selection_start,
                    selection_end,
                    text,
                )
                .map(|range| {
                    let chars_count = last_focused_text_input.props.text.chars().count();
                    range.start.min(chars_count)..range.end.min(chars_count)
                });

                todo!();
                // set_atom.set(state)

                // TEXT_INPUT_ATOM.mutate(move |text_input_ctx| {
                //     text_input_ctx.selection = selection;
                // });
            }
            web::WebEvent::KeyDown { code } => {}
            web::WebEvent::KeyUp { code } => {}
            web::WebEvent::Blur => {}
            web::WebEvent::VisibilityChange => {}
            web::WebEvent::Resize { width, height } => {}
            web::WebEvent::AsyncFunction { id } => {}
            web::WebEvent::TextInputTextUpdated { text } => {}
            web::WebEvent::TextInputKeyDown {
                code,
                text,
                selection_direction,
                selection_start,
                selection_end,
            } => {}
        });

        let font = namui::font::get_font(self.font_type);
        if font.is_none() {
            return ctx.done();
        }
        let font = font.unwrap();

        let fonts = crate::font::with_fallbacks(font);

        let paint = get_text_paint(self.style.text.color).build();

        let line_texts = LineTexts::new(
            &self.text,
            fonts.clone(),
            paint.clone(),
            self.text_param().max_width,
        );

        let custom_data = TextInputCustomData {
            id,
            props: self.clone(),
        };
        //
        //     let selection = atom.get_selection_of_text_input(id);

        //     ctx.add(
        //         render([
        //             namui::rect(RectParam {
        //                 rect: self.rect,
        //                 style: RectStyle {
        //                     stroke: if self.style.rect.stroke.is_some()
        //                         || self.style.rect.fill.is_some()
        //                     {
        //                         self.style.rect.stroke
        //                     } else {
        //                         Some(RectStroke {
        //                             color: Color::TRANSPARENT,
        //                             width: 0.px(),
        //                             border_position: BorderPosition::Inside,
        //                         })
        //                     },
        //                     ..self.style.rect
        //                 },
        //             }),
        //             self.draw_texts_divided_by_selection(
        //                 &self,
        //                 &fonts,
        //                 paint.clone(),
        //                 &line_texts,
        //                 &selection,
        //             ),
        //             self.draw_caret(&self, &line_texts, &selection, paint.clone()),
        //         ])
        //         .with_custom(custom_data.clone())
        //         .attach_event(|builder| {
        //             builder.on_mouse_down_in(move |event: MouseEvent| {
        //                 system::text_input::on_mouse_down_in_at_attach_event_calls(
        //                     event.local_xy,
        //                     &custom_data,
        //                 )
        //             });
        //         }),
        //     );
        //     ctx.done()
        // })
        ctx.done()
    }
}

impl TextInput {
    pub fn is_focused(&self) -> bool {
        todo!()
        // crate::system::text_input::is_focused(self.id)
    }
    pub fn focus(&self) {
        todo!()
        // crate::system::text_input::focus(self.id)
    }
    pub fn blur(&self) {
        todo!()
        // crate::system::text_input::blur()
    }
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
