mod decomposed_string;

use crate::components::sequence_player;
use decomposed_string::DecomposedString;
use namui::prelude::*;
use namui_prebuilt::*;

#[namui::component]
pub struct AutoCompleteTextInput<'a> {
    pub text_input_instance: TextInputInstance,
    pub wh: Wh<Px>,
    pub text: String,
    pub candidates: Sig<'a, Vec<std::string::String>>,
    pub on_event: &'a dyn Fn(text_input::Event),
    pub style: text_input::Style,
}

impl Component for AutoCompleteTextInput<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            text_input_instance,
            wh,
            text,
            candidates,
            on_event,
            style,
        } = self;

        let (over_item_text, set_over_item_text) = ctx.state::<Option<String>>(|| None);

        const LEFT_PADDING: Px = px(10.0);

        let suggestions = candidates
            .iter()
            .filter(|candidate| DecomposedString::parse(candidate).starts_with(&text))
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

        ctx.component(namui::TextInput {
            instance: text_input_instance,
            rect: wh.to_rect(),
            text: text.clone(),
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Top,
            font: sequence_player::cut_text_font(),
            style,
            prevent_default_codes: vec![Code::Tab, Code::Enter, Code::ArrowUp, Code::ArrowDown],
            on_event: &|event| match event {
                text_input::Event::TextUpdated { text } => {
                    on_event(text_input::Event::TextUpdated { text });
                }
                text_input::Event::KeyDown { event } => {
                    if !event.is_composing {
                        match event.code {
                            Code::Enter => {
                                let selected_suggestion =
                                    over_item_index.map(|index| &suggestions[index]);

                                if let Some(selected_suggestion) = selected_suggestion {
                                    on_event(text_input::Event::TextUpdated {
                                        text: selected_suggestion.as_str(),
                                    });
                                }
                            }
                            Code::ArrowUp | Code::ArrowDown => {
                                on_arrow_up_down_key(event.code);
                            }
                            _ => {}
                        }
                    }

                    on_event(text_input::Event::KeyDown { event });
                }
                text_input::Event::SelectionUpdated { selection } => {
                    on_event(text_input::Event::SelectionUpdated { selection });
                }
            },
        });

        ctx.compose(|ctx| {
            if !text_input_instance.focused() {
                return;
            }

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

const MAX_SUGGESTIONS: usize = 4;
