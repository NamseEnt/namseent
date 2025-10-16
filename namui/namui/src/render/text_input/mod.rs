mod draw_caret;
mod draw_texts_divided_by_selection;
mod focus;
mod selection;

use crate::*;
pub use focus::*;
use selection::*;
use std::{fmt::Debug, ops::Range, sync::Mutex};

/**
 * Specification for Text Input
 * - User can manually focus and blur the text input.
 * - Even user doesn't control the focus, the text input can be focused by the user's mouse click.
 * - The value of the text input will be passed to user when the text input becomes unfocused.
 * - Only one text input can be focused at a time.
 *   - If the user focuses on another text input, the previous text input will be unfocused.
 *   - If the user tries to focus on multiple text inputs at the same time, the last focused text input will be focused.
 *  - User can disable the default behavior of the text input by providing the codes that should be prevented.
 */
pub struct TextInput<'a> {
    pub rect: Rect<Px>,
    pub start_text: &'a str,
    pub text_align: TextAlign,
    pub text_baseline: TextBaseline,
    pub font: Font,
    pub style: Style,
    /// It only works with the value right before the text input is focused.
    /// If you change the value during the focus, it will not be reflected.
    pub prevent_default_codes: &'a [Code],
    pub focus: Option<&'a TextInputFocus>,
    pub on_edit_done: &'a dyn Fn(String),
}

unsafe extern "C" {
    fn text_input_set_selection_range(start: u16, end: u16, direction: u8);
    fn text_input_focus(
        width: u16,
        text_ptr: *const u8,
        text_len: u16,
        selection_start: u16,
        selection_end: u16,
        direction: u8, // 0: none, 1: forward, 2: backward
        prevent_default_codes_ptr: *const u8,
        prevent_default_codes_len: u8,
    );
    fn text_input_blur();
}

#[derive(Default, Debug)]
struct FocusCtx {
    id: u128,
    mouse_dragging: bool,
    selection: Selection,
    editing_text: String,
}

static FOCUS_CTX_ATOM: crate::Atom<Mutex<Option<FocusCtx>>> = crate::Atom::uninitialized();

impl FocusCtx {
    fn get_selection_of_text_input(&self, id: u128) -> Selection {
        if self.id != id {
            return Selection::None;
        }
        self.selection.clone()
    }
    fn update_selection(&mut self, event: &RawTextInputEvent) {
        let selection = get_input_element_selection(
            event.selection_direction,
            event.selection_start,
            event.selection_end,
            &event.text,
        );
        self.selection = selection;
    }
}

impl Component for TextInput<'_> {
    fn render(self, ctx: &RenderCtx) {
        let id = *ctx.memo(uuid);
        let (focus_ctx, set_focus_ctx) = ctx.init_atom(&FOCUS_CTX_ATOM, Default::default);

        let mut focus_ctx = focus_ctx.lock().unwrap();

        if let Some(focus) = self.focus
            && focus.focused()
        {
            focus.off();
            if focus_ctx.as_ref().map(|x| x.id) != Some(id) {
                *focus_ctx = Some(FocusCtx {
                    id,
                    mouse_dragging: false,
                    selection: Selection::None,
                    editing_text: self.start_text.to_string(),
                });
            }
        }

        let is_focused = focus_ctx.as_ref().is_some_and(|x| x.id == id);

        let text = {
            if is_focused {
                focus_ctx.as_ref().unwrap().editing_text.clone()
            } else {
                self.start_text.to_string()
            }
        };

        ctx.effect("Blur on umount if focused", || {
            move || {
                set_focus_ctx.mutate(move |focus_ctx| {
                    let focus_ctx = focus_ctx.get_mut().unwrap();
                    if focus_ctx.as_ref().map(|x| x.id) != Some(id) {
                        return;
                    }
                    unsafe {
                        text_input_blur();
                    }
                    *focus_ctx = None;
                });
            }
        });

        let paint = get_text_paint(self.style.text.color);

        let paragraph = Paragraph::new(
            &text,
            self.font.clone(),
            paint.clone(),
            self.text_param(&text).max_width,
        );

        let selection = focus_ctx
            .as_ref()
            .map(|focus_ctx| focus_ctx.get_selection_of_text_input(id))
            .unwrap_or(Selection::None);

        ctx.add(self.draw_caret(&self, &paragraph, &selection));

        ctx.add(self.draw_texts_divided_by_selection(&paragraph, &selection, &text));

        ctx.add(
            rect(RectParam {
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
                        if focus_ctx.as_mut().map(|x| x.id) == Some(id) {
                            unsafe {
                                text_input_blur();
                            }
                            *focus_ctx = None;
                        }
                        return;
                    }

                    match focus_ctx.as_mut() {
                        Some(atom) if atom.id == id => {
                            atom.mouse_dragging = true;
                        }
                        _ => {
                            *focus_ctx = Some(FocusCtx {
                                id,
                                mouse_dragging: true,
                                selection: Selection::None,
                                editing_text: text.to_string(),
                            });
                        }
                    }

                    self.update_focus_with_mouse_movement(
                        event.local_xy(),
                        false,
                        &paragraph,
                        focus_ctx.as_mut().unwrap(),
                        &text,
                    )
                }
                crate::Event::MouseUp { .. } => {
                    let Some(focus_ctx) = focus_ctx.as_mut() else {
                        return;
                    };
                    if focus_ctx.id == id {
                        focus_ctx.mouse_dragging = false;
                    }
                }
                crate::Event::MouseMove { event } => {
                    let Some(focus_ctx) = focus_ctx.as_mut() else {
                        return;
                    };
                    if focus_ctx.id == id && focus_ctx.mouse_dragging {
                        self.update_focus_with_mouse_movement(
                            event.local_xy(),
                            true,
                            &paragraph,
                            focus_ctx,
                            &text,
                        );
                    }
                }
                crate::Event::TextInputSelectionChange { event } => {
                    let Some(focus_ctx) = focus_ctx.as_mut() else {
                        return;
                    };
                    if focus_ctx.id != id {
                        return;
                    };

                    focus_ctx.update_selection(event);
                }
                crate::Event::TextInput { event } => {
                    let Some(focus_ctx) = focus_ctx.as_mut() else {
                        return;
                    };
                    if focus_ctx.id != id {
                        return;
                    };

                    focus_ctx.editing_text.clone_from(&event.text);
                    focus_ctx.update_selection(event);

                    (self.on_edit_done)(event.text.clone());
                }
                crate::Event::TextInputKeyDown { event } => {
                    if !is_focused {
                        return;
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

                        let next_selection_end = get_caret_index_after_apply_key_movement(
                            key,
                            &paragraph,
                            &range,
                            self.text_align,
                            self.rect.width(),
                        );

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
                            text_align: TextAlign,
                            container_width: Px,
                        ) -> usize {
                            let caret = paragraph.caret(selection.end);

                            let caret_after_move =
                                caret.get_caret_on_key(key, text_align, container_width);

                            caret_after_move.to_selection_index()
                        }
                    };

                    let update_selection = || {
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

                        // NOTE: This call is not in main thread so it can be delayed if user press other key before this call.
                        unsafe {
                            text_input_set_selection_range(
                                utf16_selection.start as u16,
                                utf16_selection.end as u16,
                                selection_direction as u8,
                            );
                        }
                    };

                    update_selection();
                }
                _ => {}
            }), // .with_mouse_cursor(MouseCursor::Text),
        );
    }
}

impl TextInput<'_> {
    pub fn text_param(&self, text: &str) -> TextParam {
        TextParam {
            text: text.to_string(),
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

    fn get_selection_on_mouse_movement(
        &self,
        click_local_xy: Xy<Px>,
        is_dragging_by_mouse: bool,
        paragraph: &Paragraph,
        focus_ctx: &FocusCtx,
    ) -> Selection {
        let is_shift_key_pressed =
            crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);

        let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

        Selection::Range(self.get_one_click_selection(
            paragraph,
            click_local_xy,
            is_dragging,
            &focus_ctx.selection,
        ))
    }

    fn get_one_click_selection(
        &self,
        paragraph: &Paragraph,
        click_local_xy: Xy<Px>,
        is_dragging: bool,
        last_selection: &Selection,
    ) -> Range<usize> {
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
    }

    fn update_focus_with_mouse_movement(
        &self,
        local_mouse_xy: Xy<Px>,
        is_mouse_move: bool,
        paragraph: &Paragraph,
        focus_ctx: &mut FocusCtx,
        text: &str,
    ) {
        let local_text_xy = local_mouse_xy - Xy::new(self.text_x(), self.text_y());

        let selection: Selection = self.get_selection_on_mouse_movement(
            local_text_xy,
            is_mouse_move,
            paragraph,
            focus_ctx,
        );

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

        let utf16_selection = selection.as_utf16(text);
        let selection_start = utf16_selection
            .as_ref()
            .map_or(0, |selection| selection.start.min(selection.end));
        let selection_end = utf16_selection
            .as_ref()
            .map_or(0, |selection| selection.start.max(selection.end));

        let prevent_default_codes_in_u8s = self
            .prevent_default_codes
            .iter()
            .map(|code| unsafe { std::mem::transmute::<Code, u8>(*code) })
            .collect::<Vec<u8>>();
        let text_bytes = text.as_bytes();
        unsafe {
            text_input_focus(
                self.rect.width().as_f32() as u16,
                text_bytes.as_ptr(),
                text_bytes.len() as u16,
                selection_start as u16,
                selection_end as u16,
                match selection_direction {
                    SelectionDirection::None => 0,
                    SelectionDirection::Forward => 1,
                    SelectionDirection::Backward => 2,
                },
                prevent_default_codes_in_u8s.as_slice().as_ptr(),
                prevent_default_codes_in_u8s.len() as u8,
            );
        }
        focus_ctx.selection = selection;
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

#[derive(Clone, Copy, Debug)]
pub struct CursorPosition {
    pub is_at_top: bool,
    pub is_at_bottom: bool,
}

pub enum ArrowUpDown {
    Up,
    Down,
}

pub enum HomeEnd {
    Home,
    End,
}
