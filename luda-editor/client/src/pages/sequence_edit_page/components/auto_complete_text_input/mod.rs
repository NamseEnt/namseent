mod decomposed_string;
// mod render;
// mod update;

use crate::{color, components::sequence_player};
use decomposed_string::DecomposedString;
use namui::prelude::*;
use namui_prebuilt::*;
use std::{collections::VecDeque, fmt::Debug};

#[namui::component]
pub struct AutoCompleteTextInput<'a> {
    pub wh: Wh<Px>,
    pub text: String,
    pub candidates: Sig<'a, Vec<std::string::String>>,
    pub on_event: Box<dyn 'a + Fn(Event)>,
    pub req_queue: VecDeque<Request>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    Focus,
    Blur,
}

pub enum Event<'a> {
    TextChange { text: String },
    EditDone,
    KeyDown { event: KeyboardEvent<'a> },
    ReqQueuePopFront,
}

impl Component for AutoCompleteTextInput<'_> {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let Self {
            wh,
            ref text,
            ref candidates,
            ref on_event,
            req_queue,
        } = self;
        let on_event = on_event.clone();

        let (over_item_text, set_over_item_text) = ctx.state::<Option<String>>(|| None);
        let text_input_instance = namui::text_input::TextInputInstance::new(ctx);

        ctx.effect("handle req_queue", || {
            if let Some(req) = req_queue.front() {
                match req {
                    Request::Focus => {
                        text_input_instance.focus();
                    }
                    Request::Blur => {
                        text_input_instance.blur();
                    }
                }
                on_event(Event::ReqQueuePopFront);
            }
        });

        const LEFT_PADDING: Px = px(10.0);

        let suggestions = candidates
            .iter()
            .filter(|candidate| DecomposedString::parse(candidate).starts_with(text))
            .map(|candidate| candidate.to_string())
            .take(MAX_SUGGESTIONS)
            .collect::<Vec<_>>();

        let over_item_index = (*over_item_text).as_ref().and_then(|over_item_text| {
            suggestions
                .iter()
                .position(|suggestion| suggestion == over_item_text)
        });

        let suggestion_count = suggestions.len();

        let on_arrow_up_down_key = {
            let suggestions = suggestions.clone();
            move |code: Code| {
                let next_index = {
                    match code == Code::ArrowUp {
                        true => match over_item_index {
                            Some(over_item_index) => {
                                if over_item_index == 0 {
                                    None
                                } else {
                                    Some(over_item_index - 1)
                                }
                            }
                            None => {
                                return;
                            }
                        },
                        false => match over_item_index {
                            Some(over_item_index) => {
                                if suggestion_count == 0 {
                                    None
                                } else if over_item_index == suggestion_count - 1 {
                                    return;
                                } else {
                                    Some((over_item_index + 1).min(suggestion_count - 1))
                                }
                            }
                            None => {
                                if suggestion_count == 0 {
                                    None
                                } else {
                                    Some(0)
                                }
                            }
                        },
                    }
                };
                set_over_item_text.set(next_index.map(|index| suggestions[index].clone()));

                if next_index.is_some() {
                    text_input_instance.blur();
                } else {
                    text_input_instance.focus();
                }
            }
        };

        let on_enter_down = {
            let selected_suggestion = over_item_index.map(|index| suggestions[index].clone());
            let on_event = on_event.clone();

            move |code: Code| {
                if code != Code::Enter {
                    return;
                }

                if let Some(selected_suggestion) = &selected_suggestion {
                    on_event(Event::TextChange {
                        text: selected_suggestion.clone(),
                    });
                }

                on_event(Event::EditDone)
            }
        };

        ctx.component(
            namui::TextInput {
                instance: text_input_instance,
                rect: wh.to_rect(),
                text: text.clone(),
                text_align: TextAlign::Left,
                text_baseline: TextBaseline::Top,
                font: sequence_player::cut_text_font(),
                style: text_input::Style {
                    text: sequence_player::cut_text_style(1.one_zero()),
                    rect: RectStyle {
                        stroke: Some(RectStroke {
                            color: color::STROKE_FOCUS,
                            width: 2.px(),
                            border_position: BorderPosition::Middle,
                        }),
                        fill: Some(RectFill {
                            color: color::BACKGROUND,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                prevent_default_codes: vec![Code::Tab, Code::Enter, Code::ArrowUp, Code::ArrowDown],
                on_event: &|event| match event {
                    text_input::Event::TextUpdated { text } => {
                        on_event(Event::TextChange {
                            text: text.to_string(),
                        });
                    }
                    text_input::Event::KeyDown { code } => match code {
                        Code::Tab => {}
                        Code::Enter => {
                            on_enter_down(code);
                        }
                        Code::ArrowUp | Code::ArrowDown => {
                            on_arrow_up_down_key(code);
                        }
                        _ => {}
                    },
                    text_input::Event::SelectionUpdated { selection: _ } => {}
                },
            }
            .attach_event(|event| match event {
                namui::Event::KeyDown { event } => {
                    on_event(Event::KeyDown { event });
                }
                _ => {}
            }),
        );
        ctx.compose(|ctx| {
            let mut ctx = ctx.on_top().translate((0.px(), wh.height));
            let body_height = wh.height * suggestions.len();

            ctx.add(simple_rect(
                Wh {
                    width: wh.width,
                    height: body_height,
                },
                Color::BLACK,
                1.px(),
                Color::TRANSPARENT,
            ));

            for (index, suggestion) in suggestions.into_iter().enumerate() {
                let is_cursor_over = over_item_index == Some(index);
                let background = simple_rect(
                    wh,
                    Color::WHITE,
                    0.px(),
                    if is_cursor_over {
                        Color::from_u8(0x5C, 0x5C, 255, 255)
                    } else {
                        Color::WHITE
                    },
                );
                let text = typography::body::left(
                    wh.height,
                    suggestion,
                    if is_cursor_over {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                );
                ctx.translate((0.px(), wh.height * index))
                    .translate((LEFT_PADDING, 0.px()))
                    .add(text)
                    .add(background);
            }
        });

        ctx.done()
    }
}

// pub struct Props<
//     OnTextChange: Fn(String) + 'static,
//     OnEditDone: Fn() + 'static,
//     OnKeyDown: Fn(&KeyDownEvent) + 'static,
// > {}

// enum InternalEvent {
//     ArrowUpDown { next_index: Option<usize> },
//     UpdateItemIndex { over_item_index: Option<usize> },
// }

const MAX_SUGGESTIONS: usize = 4;

// impl AutoCompleteTextInput {
//     pub fn new() -> Self {
//         Self {
//             text_input: TextInput::new(),
//             over_item_index: None,
//         }
//     }
//     pub fn focus(&mut self) {
//         text_input.focus();
//     }

//     pub(crate) fn text_input_id(&self) -> Uuid {
//         text_input.get_id()
//     }

//     pub(crate) fn blur(&self) {
//         text_input.blur();
//     }
// }
