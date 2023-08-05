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
        OnceLock,
    },
};

#[component]
pub struct TextInput<'a> {
    pub instance: TextInputInstance,
    pub rect: Rect<Px>,
    pub text: String,
    pub text_align: TextAlign,
    pub text_baseline: TextBaseline,
    pub font_type: FontType,
    pub style: Style,
    pub prevent_default_codes: Vec<Code>,
    pub on_event: callback!('a, Event),
}

pub enum Event<'a> {
    Focus,
    Blur,
    TextUpdated { text: &'a str },
    SelectionUpdated { selection: Selection },
    KeyDown { code: Code },
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

impl Component for TextInput<'_> {
    fn render<'a>(&'a self, ctx: &'a RenderCtx) -> RenderDone {
        let on_event = &self.on_event;
        let id = self.instance.id;
        let (atom, set_atom) = ctx.atom_init(&TEXT_INPUT_ATOM, || TextInputCtx {
            focused_id: None,
            mouse_dragging: false,
            selection: Selection::None,
        });

        let is_focused = ctx.memo(|| atom.is_focused(id));
        let prevent_default_codes = ctx.track_eq(&self.prevent_default_codes);

        static MOUSE_DOWN_FIRST_CALL: OnceLock<AtomicBool> = OnceLock::new();

        ctx.effect("Set WebEvent first call", || {
            MOUSE_DOWN_FIRST_CALL
                .get_or_init(Default::default)
                .store(true, Ordering::Relaxed)
        });

        ctx.effect("Update prevent default codes", || {
            if !*is_focused {
                return;
            }
            if prevent_default_codes.on_effect() {
                web::execute_function_sync(
                    "
                    globalThis.textAreaKeydownPreventDefaultCodes = preventDefaultcodes;
                    ",
                )
                .arg(
                    "preventDefaultcodes",
                    prevent_default_codes
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>(),
                )
                .run::<()>();
            }
        });

        // TODO: blur on unmount if focused

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

        let get_selection_on_mouse_movement = |click_local_xy: Xy<Px>,
                                               is_dragging_by_mouse: bool|
         -> Selection {
            let font = crate::font::get_font(self.font_type);

            if font.is_none() {
                return Selection::None;
            };
            let font = font.unwrap();
            let fonts = crate::font::with_fallbacks(font);

            let is_shift_key_pressed =
                crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);

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

        let update_selection = |selection_direction: SelectionDirection,
                                selection_start: usize,
                                selection_end: usize,
                                text: &str| {
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

            TEXT_INPUT_ATOM.mutate(move |text_input_ctx| text_input_ctx.selection = selection);
        };

        let Some(font) = namui::font::get_font(self.font_type) else {
            return ctx.return_no();
        };

        let fonts = crate::font::with_fallbacks(font);

        let paint = get_text_paint(self.style.text.color).build();

        let line_texts = LineTexts::new(
            &self.text,
            fonts.clone(),
            paint.clone(),
            self.text_param().max_width,
        );

        let selection = atom.get_selection_of_text_input(id);

        ctx.return_((
            namui::rect(RectParam {
                rect: self.rect,
                style: RectStyle {
                    stroke: if self.style.rect.stroke.is_some() || self.style.rect.fill.is_some() {
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
            })
            .on_event(|event| {
                match event {
                    crate::Event::MouseDown { event } => {
                        if !event.is_local_xy_in() {
                            return;
                        }

                        let id = id.clone();
                        set_atom.mutate(move |atom| {
                            atom.focused_id = Some(id);
                            atom.mouse_dragging = true;
                        });

                        update_focus_with_mouse_movement(event.local_xy(), false);
                    }
                    crate::Event::MouseUp { .. } => {
                        if *is_focused && atom.mouse_dragging {
                            set_atom.mutate(|x| x.mouse_dragging = false);
                        }
                    }
                    crate::Event::MouseMove { event } => {
                        if *is_focused && atom.mouse_dragging {
                            update_focus_with_mouse_movement(event.local_xy(), true);
                        }
                    }
                    crate::Event::SelectionChange {
                        selection_direction,
                        selection_start,
                        selection_end,
                        ref text,
                    } => {
                        if !*is_focused {
                            return;
                        };

                        update_selection(selection_direction, selection_start, selection_end, text);
                    }
                    crate::Event::TextInputTextUpdated {
                        ref text,
                        selection_direction,
                        selection_start,
                        selection_end,
                    } => {
                        if !*is_focused {
                            return;
                        }

                        update_selection(selection_direction, selection_start, selection_end, text);

                        on_event(Event::TextUpdated {
                            text: text.as_str(),
                        });
                    }
                    crate::Event::TextInputKeyDown {
                        code,
                        ref text,
                        selection_direction,
                        selection_start,
                        selection_end,
                        is_composing,
                    } => {
                        if !*is_focused {
                            return;
                        }

                        on_event(Event::KeyDown { code });

                        // self.event_handler.as_ref().map(|event_handler| {
                        //     event_handler.on_key_down.as_ref().map(|on_key_down| {
                        //         let is_prevented_default = Arc::new(AtomicBool::new(false));

                        //         let key_down_event = KeyDownEvent {
                        //             code,
                        //             is_prevented_default: is_prevented_default.clone(),
                        //             is_composing,
                        //         };
                        //         on_key_down.invoke(key_down_event);

                        //         if is_prevented_default.load(Ordering::Relaxed) {
                        //             todo!()
                        //             // event.prevent_default();
                        //         }
                        //     })
                        // });

                        let get_selection_on_keyboard_down = |key: KeyInInterest| -> Selection {
                            let selection = get_input_element_selection(
                                selection_direction,
                                selection_start,
                                selection_end,
                                &text,
                            );
                            let Selection::Range(range) = selection else {
                                    return Selection::None;
                                };

                            let Some(line_texts) = self.get_line_texts() else {
                                    return Selection::None;
                                };

                            let next_selection_end =
                                get_caret_index_after_apply_key_movement(key, line_texts, &range);

                            let is_shift_key_pressed = crate::keyboard::any_code_press([
                                crate::Code::ShiftLeft,
                                crate::Code::ShiftRight,
                            ]);
                            let is_dragging = is_shift_key_pressed;

                            return match is_dragging {
                                true => Selection::Range(range.start..next_selection_end),
                                false => Selection::Range(next_selection_end..next_selection_end),
                            };

                            fn get_caret_index_after_apply_key_movement(
                                key: KeyInInterest,
                                line_texts: LineTexts,
                                selection: &Range<usize>,
                            ) -> usize {
                                let multiline_caret =
                                    line_texts.into_multiline_caret(selection.end);

                                let caret_after_move = multiline_caret.get_caret_on_key(key);

                                let next_selection_end = caret_after_move.to_selection_index();
                                next_selection_end
                            }
                        };

                        let key_in_interest = match code {
                            Code::ArrowUp => KeyInInterest::ArrowUpDown(ArrowUpDown::Up),
                            Code::ArrowDown => KeyInInterest::ArrowUpDown(ArrowUpDown::Down),
                            Code::Home => KeyInInterest::HomeEnd(HomeEnd::Home),
                            Code::End => KeyInInterest::HomeEnd(HomeEnd::End),
                            _ => return,
                        };

                        let selection = get_selection_on_keyboard_down(key_in_interest);

                        let Some(utf16_selection) = selection.as_utf16(&text) else {
                                return;
                            };

                        let selection_direction = if utf16_selection.start <= utf16_selection.end {
                            "forward"
                        } else {
                            "backward"
                        };

                        web::execute_function_sync(
                            "
                                        textArea.setSelectionRange(
                                            selectionStart,
                                            selectionEnd,
                                            selectionDirection,
                                        )
                                    ",
                        )
                        .arg(
                            "selectionStart",
                            utf16_selection.start.min(utf16_selection.end) as u32,
                        )
                        .arg(
                            "selectionEnd",
                            utf16_selection.start.max(utf16_selection.end) as u32,
                        )
                        .arg("selectionDirection", selection_direction)
                        .run::<()>();
                    }
                    _ => {}
                }
            }),
            self.draw_texts_divided_by_selection(
                &self,
                &fonts,
                paint.clone(),
                &line_texts,
                &selection,
            ),
            self.draw_caret(&self, &line_texts, &selection, paint.clone()),
        ))
    }
}

impl TextInput<'_> {
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
    fn get_line_texts(&self) -> Option<LineTexts> {
        let font = crate::font::get_font(self.font_type)?;
        let fonts = crate::font::with_fallbacks(font);
        let paint = get_text_paint(self.style.text.color).build();
        Some(LineTexts::new(
            &self.text,
            fonts,
            paint.clone(),
            Some(self.rect.width()),
        ))
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
