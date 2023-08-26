mod draw_caret;
mod draw_texts_divided_by_selection;
mod instance;
mod selection;

use crate::*;
pub use instance::*;
use selection::*;
use std::{
    fmt::Debug,
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
};

#[component]
pub struct TextInput<'a> {
    pub instance: TextInputInstance,
    pub rect: Rect<Px>,
    pub text: String,
    pub text_align: TextAlign,
    pub text_baseline: TextBaseline,
    pub font: Font,
    pub style: Style,
    pub prevent_default_codes: Vec<Code>,
    pub on_event: &'a dyn Fn(Event),
}

#[derive(Debug)]
pub enum Event<'a> {
    TextUpdated { text: &'a str },
    SelectionUpdated { selection: Selection },
    KeyDown { code: Code },
}

#[derive(Debug, Default)]
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

impl Component for TextInput<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let id = self.instance.id;
        let (atom, set_atom) = ctx.atom_init(&TEXT_INPUT_ATOM, Default::default);

        let is_focused = ctx.memo(|| atom.is_focused(id));

        static MOUSE_DOWN_FIRST_CALL: OnceLock<AtomicBool> = OnceLock::new();

        ctx.effect("Set WebEvent first call", || {
            MOUSE_DOWN_FIRST_CALL
                .get_or_init(Default::default)
                .store(true, Ordering::Relaxed);
        });
        let paint = get_text_paint(self.style.text.color);

        let paragraph = Paragraph::new(
            &self.text,
            system::font::group_glyph(&self.font, &paint),
            self.text_param().max_width,
        );

        ctx.effect("Blur on umount if focused", || {
            return move || {
                set_atom.mutate(move |atom| {
                    if atom.is_focused(id) {
                        *atom = Default::default();
                    }
                })
            };
        });

        let get_one_click_selection = |paragraph: &Paragraph,
                                       click_local_xy: Xy<Px>,
                                       is_dragging: bool,
                                       last_selection: &Selection|
         -> Range<usize> {
            let selection_index_of_xy = paragraph.selection_index_of_xy(
                click_local_xy,
                self.font.size,
                self.style.text.line_height_percent,
                self.text_baseline,
                self.text_align,
            );

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
            let is_shift_key_pressed =
                crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);

            let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

            Selection::Range(get_one_click_selection(
                &paragraph,
                click_local_xy,
                is_dragging,
                &atom.selection,
            ))
        };

        let update_focus_with_mouse_movement = |local_mouse_xy: Xy<Px>, is_mouse_move: bool| {
            let local_text_xy = local_mouse_xy - Xy::new(self.text_x(), self.text_y());

            let selection: Selection =
                get_selection_on_mouse_movement(local_text_xy, is_mouse_move);

            let selection_direction = match &selection {
                Selection::Range(range) => {
                    if range.start <= range.end {
                        SelectionDirection::Forward
                    } else {
                        SelectionDirection::Backward
                    }
                }
                Selection::None => SelectionDirection::None,
            };

            let utf16_selection = selection.as_utf16(&self.text);
            let selection_start = utf16_selection
                .as_ref()
                .map_or(0, |selection| selection.start.min(selection.end));
            let selection_end = utf16_selection
                .as_ref()
                .map_or(0, |selection| selection.start.max(selection.end));

            crate::system::text_input::set_width(self.rect.width());
            crate::system::text_input::set_value(&self.text);
            crate::system::text_input::set_selection_range(
                selection_start,
                selection_end,
                selection_direction,
            );
            crate::system::text_input::focus();
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

        let selection = atom.get_selection_of_text_input(id);

        ctx.component(self.draw_caret(&self, &paragraph, &selection));

        ctx.component(self.draw_texts_divided_by_selection(&paragraph, &selection));

        ctx.component(
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
            .attach_event(|event| match event {
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

                    (self.on_event)(Event::TextUpdated {
                        text: text.as_str(),
                    });
                }
                crate::Event::TextInputKeyDown { event } => {
                    if !*is_focused {
                        return;
                    }

                    (self.on_event)(Event::KeyDown { code: event.code });

                    if self.prevent_default_codes.contains(&event.code) {
                        event.prevent_default();
                    }

                    let get_selection_on_keyboard_down = |key: CaretKey| -> Selection {
                        let selection = get_input_element_selection(
                            event.selection_direction,
                            event.selection_start,
                            event.selection_end,
                            &event.text,
                        );
                        let Selection::Range(range) = selection else {
                                return Selection::None;
                            };

                        let next_selection_end =
                            get_caret_index_after_apply_key_movement(key, &paragraph, &range);

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
                            key: CaretKey,
                            paragraph: &Paragraph,
                            selection: &Range<usize>,
                        ) -> usize {
                            let caret = paragraph.caret(selection.end);

                            let caret_after_move = caret.get_caret_on_key(key);

                            let next_selection_end = caret_after_move.to_selection_index();
                            next_selection_end
                        }
                    };

                    let caret_key = match event.code {
                        Code::ArrowUp => CaretKey::ArrowUp,
                        Code::ArrowDown => CaretKey::ArrowDown,
                        Code::Home => CaretKey::Home,
                        Code::End => CaretKey::End,
                        _ => return,
                    };

                    let selection = get_selection_on_keyboard_down(caret_key);

                    let Some(utf16_selection) = selection.as_utf16(&event.text) else {
                            return;
                        };

                    let selection_direction = if utf16_selection.start <= utf16_selection.end {
                        SelectionDirection::Forward
                    } else {
                        SelectionDirection::Backward
                    };

                    crate::system::text_input::set_selection_range(
                        utf16_selection.start,
                        utf16_selection.end,
                        selection_direction,
                    );
                }
                _ => {}
            }),
        );

        ctx.done()
    }
}

impl TextInput<'_> {
    pub fn text_param(&self) -> TextParam {
        TextParam {
            text: self.text.clone(),
            x: self.text_x(),
            y: self.text_y(),
            align: self.text_align,
            baseline: self.text_baseline,
            font: self.font.clone(),
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
        self.font.size.into_px() * self.style.text.line_height_percent
    }
    // fn get_paragraph(&self) -> Option<Paragraph> {
    //     let font = crate::font::get_font(self.font)?;
    //     let fonts = crate::font::with_fallbacks(font);
    //     let paint = get_text_paint(self.style.text.color).build();
    //     Some(Paragraph::new(
    //         &self.text,
    //         fonts,
    //         paint.clone(),
    //         Some(self.rect.width()),
    //     ))
    // }
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
