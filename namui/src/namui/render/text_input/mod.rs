mod draw_caret;
mod draw_texts_divided_by_selection;
mod instance;
mod selection;

use crate::{
    namui::{self, *},
    text::{get_text_widths, LineTexts},
    web::SelectionDirection,
};
pub use instance::*;
use selection::*;
use std::{
    fmt::Debug,
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, OnceLock,
    },
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

#[derive(Debug)]
struct TextInputCtx {
    pub focused_id: Option<Uuid>,
    pub mouse_dragging: bool,
    pub selection: Selection,
}

static TEXT_INPUT_ATOM: crate::Atom<TextInputCtx> = crate::Atom::uninitialized_new();

impl TextInputCtx {
    fn is_focused(&self, id: Uuid) -> bool {
        self.focused_id.is_some_and(|focused_id| focused_id == id)
    }
    fn get_selection_of_text_input(&self, id: Uuid) -> Selection {
        if self.focused_id.is_some_and(|focused_id| focused_id == id) {
            return self.selection.clone();
        }
        Selection::None
    }
}

struct TextInputMouseEvent {
    id: Uuid,
    local_xy: Xy<Px>,
}

impl Component for TextInput {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let id = self.instance.id;
        let (atom, set_atom) = ctx.atom_init(&TEXT_INPUT_ATOM, || TextInputCtx {
            focused_id: None,
            mouse_dragging: false,
            selection: Selection::None,
        });

        let is_focused = atom.is_focused(id);

        static LAST_MOUSE_DOWNED: OnceLock<Mutex<Option<TextInputMouseEvent>>> = OnceLock::new();
        static LAST_MOUSE_MOVED: OnceLock<Mutex<Option<TextInputMouseEvent>>> = OnceLock::new();
        static MOUSE_DOWN_FIRST_CALL: OnceLock<AtomicBool> = OnceLock::new();

        ctx.effect("Set WebEvent first call", || {
            MOUSE_DOWN_FIRST_CALL
                .get_or_init(Default::default)
                .store(true, Ordering::Relaxed)
        });

        // TODO: blur on unmount if focused

        ctx.web_event(|web_event| {
            let get_selection_index_of_xy = |fonts: &Vec<Arc<Font>>,
                                             line_texts: &LineTexts,
                                             click_local_xy: Xy<Px>,
                                             paint: Arc<Paint>|
             -> usize {
                let line_len = line_texts.line_len();
                if line_len == 0 {
                    return 0;
                }

                let line_index = {
                    let line_height = self.line_height_px();

                    let top_y = click_local_xy.y
                        + line_height
                            * match self.text_baseline {
                                TextBaseline::Top => 0.0,
                                TextBaseline::Middle => line_len as f32 / 2.0,
                                TextBaseline::Bottom => line_len as f32,
                            };

                    let line_index = if top_y <= 0.px() {
                        0
                    } else {
                        (top_y / line_height).floor() as usize
                    };

                    let line_max_index = line_len - 1;
                    line_index.min(line_max_index)
                };

                let str_index_before_line = line_texts.char_index_before_line(line_index);

                let line_text = line_texts.iter_str().nth(line_index).unwrap();

                let glyph_widths = get_text_widths(&line_text, &fonts, paint);

                let line_width = glyph_widths.iter().sum::<Px>();

                let aligned_x = match self.text_align {
                    TextAlign::Left => click_local_xy.x,
                    TextAlign::Center => click_local_xy.x + line_width / 2.0,
                    TextAlign::Right => click_local_xy.x + line_width,
                };

                let mut left = px(0.0);
                let index = glyph_widths
                    .iter()
                    .position(|width| {
                        let center = left + width / 2.0;
                        if aligned_x < center {
                            return true;
                        }
                        left += *width;
                        return false;
                    })
                    .unwrap_or(line_text.chars().count());

                str_index_before_line + index
            };

            let get_one_click_selection = |fonts: &Vec<Arc<Font>>,
                                           line_texts: &LineTexts,
                                           click_local_xy: Xy<Px>,
                                           is_dragging: bool,
                                           last_selection: &Selection,
                                           paint: Arc<Paint>|
             -> Range<usize> {
                let selection_index_of_xy =
                    get_selection_index_of_xy(fonts, line_texts, click_local_xy, paint);

                let start = match last_selection {
                    Selection::Range(range) => {
                        if !is_dragging {
                            selection_index_of_xy
                        } else {
                            range.start
                        }
                    }
                    Selection::None => selection_index_of_xy,
                };

                start..selection_index_of_xy
            };

            let get_selection_on_mouse_movement =
                |click_local_xy: Xy<Px>, is_dragging_by_mouse: bool| -> Selection {
                    let font = crate::font::get_font(self.font_type);

                    if font.is_none() {
                        return Selection::None;
                    };
                    let font = font.unwrap();
                    let fonts = crate::font::with_fallbacks(font);

                    let is_shift_key_pressed = crate::keyboard::any_code_press([
                        crate::Code::ShiftLeft,
                        crate::Code::ShiftRight,
                    ]);

                    let paint = get_text_paint(self.style.text.color).build();

                    let line_texts = LineTexts::new(
                        &self.text,
                        fonts.clone(),
                        paint.clone(),
                        Some(self.rect.width()),
                    );

                    // const continouslyFastClickCount: number;

                    // if (continouslyFastClickCount >= 3) {
                    //   return getMoreTripleClickSelection({ text });
                    // }
                    // if (continouslyFastClickCount === 2) {
                    //   return getDoubleClickSelection({ text, font, x: localX });
                    // }

                    let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

                    Selection::Range(get_one_click_selection(
                        &fonts,
                        &line_texts,
                        click_local_xy,
                        is_dragging,
                        &atom.selection,
                        paint,
                    ))
                };

            let update_focus_with_mouse_movement = |local_mouse_xy: Xy<Px>, is_mouse_move: bool| {
                let local_text_xy = local_mouse_xy - Xy::new(self.text_x(), self.text_y());

                let selection: Selection =
                    get_selection_on_mouse_movement(local_text_xy, is_mouse_move);

                let selection_direction = match &selection {
                    Selection::Range(range) => {
                        if range.start <= range.end {
                            "forward"
                        } else {
                            "backward"
                        }
                    }
                    Selection::None => "none",
                };

                // prev: let utf16_selection = selection.as_utf16(input_element.value());
                let utf16_selection = selection.as_utf16(&self.text);
                let selection_start = utf16_selection
                    .as_ref()
                    .map_or(0, |selection| selection.start.min(selection.end) as u32);
                let selection_end = utf16_selection
                    .as_ref()
                    .map_or(0, |selection| selection.start.max(selection.end) as u32);

                let width = self.rect.width().as_f32();

                web::execute_function_sync(
                    "
textArea.style.width = `${width}px`;
textArea.value = text;
textArea.setSelectionRange(selectionStart, selectionEnd, selectionDirection);
textArea.focus();
",
                )
                .arg("width", width)
                .arg("text", &self.text)
                .arg("selectionStart", selection_start)
                .arg("selectionEnd", selection_end)
                .arg("selectionDirection", selection_direction)
                .run::<()>();
            };

            match web_event {
                &web::WebEvent::MouseDown { .. } => {
                    let mut last_mouse_downed = LAST_MOUSE_DOWNED
                        .get_or_init(Default::default)
                        .lock()
                        .unwrap();

                    let is_me = last_mouse_downed.as_ref().is_some_and(|x| x.id == id);
                    if !is_me {
                        return;
                    }

                    let last_mouse_downed = last_mouse_downed.take().unwrap();

                    set_atom.mutate(move |x| {
                        x.focused_id = Some(last_mouse_downed.id);
                        x.mouse_dragging = true;
                    });

                    update_focus_with_mouse_movement(last_mouse_downed.local_xy, false);
                }
                &web::WebEvent::MouseUp { .. } => {
                    if is_focused && atom.mouse_dragging {
                        crate::log!("mouse up");
                        set_atom.mutate(|x| x.mouse_dragging = false);
                    }
                }
                &web::WebEvent::MouseMove { .. } => {
                    if is_focused && atom.mouse_dragging {
                        let last_mouse_downed = LAST_MOUSE_MOVED
                            .get_or_init(Default::default)
                            .lock()
                            .unwrap()
                            .take()
                            .unwrap();

                        update_focus_with_mouse_movement(last_mouse_downed.local_xy, true);
                    }
                }
                &web::WebEvent::SelectionChange {
                    selection_direction,
                    selection_start,
                    selection_end,
                    ref text,
                } => {
                    if !atom.is_focused(id) {
                        return;
                    };

                    let selection = get_input_element_selection(
                        selection_direction,
                        selection_start,
                        selection_end,
                        text,
                    )
                    .map(|range| {
                        let chars_count = self.text.chars().count();
                        range.start.min(chars_count)..range.end.min(chars_count)
                    });

                    TEXT_INPUT_ATOM
                        .mutate(move |text_input_ctx| text_input_ctx.selection = selection);
                }
                web::WebEvent::TextInputTextUpdated { text } => {}
                web::WebEvent::TextInputKeyDown {
                    code,
                    text,
                    selection_direction,
                    selection_start,
                    selection_end,
                } => {}
                _ => {}
            }
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
        let selection = atom.get_selection_of_text_input(id);

        ctx.add(
            render([
                namui::rect(RectParam {
                    rect: self.rect,
                    style: RectStyle {
                        stroke: if self.style.rect.stroke.is_some()
                            || self.style.rect.fill.is_some()
                        {
                            self.style.rect.stroke
                        } else {
                            Some(RectStroke {
                                color: Color::TRANSPARENT,
                                width: 0.px(),
                                border_position: BorderPosition::Inside,
                            })
                        },
                        ..self.style.rect
                    },
                }),
                self.draw_texts_divided_by_selection(
                    &self,
                    &fonts,
                    paint.clone(),
                    &line_texts,
                    &selection,
                ),
                self.draw_caret(&self, &line_texts, &selection, paint.clone()),
            ])
            .with_custom(custom_data.clone())
            .attach_event(|builder| {
                builder
                    .on_mouse_down_in(move |event: MouseEvent| {
                        LAST_MOUSE_DOWNED
                            .get_or_init(Default::default)
                            .lock()
                            .unwrap()
                            .replace(TextInputMouseEvent {
                                id,
                                local_xy: event.local_xy,
                            });
                    })
                    .on_mouse(move |event| match event.event_type {
                        MouseEventType::Down => {}
                        MouseEventType::Up => {}
                        MouseEventType::Move => {
                            if !is_focused {
                                return;
                            }

                            LAST_MOUSE_MOVED
                                .get_or_init(Default::default)
                                .lock()
                                .unwrap()
                                .replace(TextInputMouseEvent {
                                    id,
                                    local_xy: event.local_xy,
                                });
                        }
                    });
            }),
        );
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

fn get_input_element_selection(
    selection_direction: SelectionDirection,
    selection_start: usize,
    selection_end: usize,
    text: &str,
) -> Selection {
    let utf16_code_unit_selection = {
        if selection_direction == SelectionDirection::Backward {
            selection_end..selection_start
        } else {
            selection_start..selection_end
        }
    };

    Selection::from_utf16(Some(utf16_code_unit_selection), text)
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

pub enum ArrowUpDown {
    Up,
    Down,
}

pub enum HomeEnd {
    Home,
    End,
}

pub enum KeyInInterest {
    ArrowUpDown(ArrowUpDown),
    HomeEnd(HomeEnd),
}
