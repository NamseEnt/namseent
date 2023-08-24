use super::*;
use crate::{components::sequence_player, *};
use decomposed_string::*;
use namui_prebuilt::*;
use std::sync::Arc;

impl AutoCompleteTextInput {
    pub fn render<
        OnTextChange: Fn(String) + 'static,
        OnEditDone: Fn() + 'static,
        OnKeyDown: Fn(&KeyDownEvent) + 'static,
    >(
        &self,
        props: Props<OnTextChange, OnEditDone, OnKeyDown>,
    ) -> namui::RenderingTree {
        const LEFT_PADDING: Px = px(10.0);

        let suggestions = props
            .candidates
            .iter()
            .filter(|candidate| DecomposedString::parse(candidate).starts_with(&props.text))
            .map(|candidate| candidate.to_string())
            .take(MAX_SUGGESTIONS)
            .collect::<Vec<_>>();

        let suggestion_count = suggestions.len();
        let over_item_index = self.over_item_index.and_then(|index| {
            if suggestion_count == 0 {
                None
            } else {
                Some(index.min(suggestion_count.saturating_sub(1)))
            }
        });
        if over_item_index != self.over_item_index {
            namui::event::send(InternalEvent::UpdateItemIndex { over_item_index });
        }

        let on_arrow_up_down_key = move |code: Code| {
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
            namui::event::send(InternalEvent::ArrowUpDown { next_index });
        };
        let on_text_change = Arc::new(props.on_text_change);

        let on_enter_down = {
            let on_edit_done = Arc::new(props.on_edit_done);
            let on_text_change = on_text_change.clone();
            let selected_suggestion = over_item_index.map(|index| suggestions[index].clone());

            move |code: Code| {
                if code != Code::Enter {
                    return;
                }

                if let Some(selected_suggestion) = &selected_suggestion {
                    (on_text_change)(selected_suggestion.clone());
                }

                (on_edit_done)();
            }
        };

        let text_input = self.text_input.render(text_input::Props {
            rect: props.wh.to_rect(),
            text: props.text.clone(),
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Top,
            font_type: sequence_player::CUT_TEXT_FONT,
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
            event_handler: Some(
                text_input::EventHandler::new()
                    .on_text_updated(move |text| (on_text_change)(text))
                    .on_key_down({
                        let on_enter_down = on_enter_down.clone();
                        move |event: KeyDownEvent| {
                            (props.on_key_down)(&event);
                            if event.is_prevented_default() || event.is_composing {
                                return;
                            }

                            match event.code {
                                Code::Tab => {
                                    event.prevent_default();
                                }
                                Code::Enter => {
                                    on_enter_down(event.code);
                                    event.prevent_default();
                                }
                                Code::ArrowUp | Code::ArrowDown => {
                                    on_arrow_up_down_key(event.code);
                                    event.prevent_default();
                                }
                                _ => {}
                            }
                        }
                    }),
            ),
        });

        let body = {
            let body_height = props.wh.height * suggestions.len();
            on_top(translate(
                0.px(),
                props.wh.height,
                namui::render(
                    suggestions
                        .into_iter()
                        .enumerate()
                        .map(|(index, suggestion)| {
                            let is_cursor_over = over_item_index == Some(index);
                            let background = simple_rect(
                                props.wh,
                                Color::WHITE,
                                0.px(),
                                if is_cursor_over {
                                    Color::from_u8(0x5C, 0x5C, 255, 255)
                                } else {
                                    Color::WHITE
                                },
                            );
                            let text = translate(
                                LEFT_PADDING,
                                0.px(),
                                typography::body::left(
                                    props.wh.height,
                                    suggestion,
                                    if is_cursor_over {
                                        Color::WHITE
                                    } else {
                                        Color::BLACK
                                    },
                                ),
                            );
                            translate(
                                0.px(),
                                props.wh.height * index,
                                namui::render([background, text]),
                            )
                        })
                        .into_iter()
                        .chain([simple_rect(
                            Wh {
                                width: props.wh.width,
                                height: body_height,
                            },
                            Color::BLACK,
                            1.px(),
                            Color::TRANSPARENT,
                        )]),
                ),
            ))
        };
        namui::render([text_input, body]).attach_event(move |builder| {
            let on_enter_down = on_enter_down.clone();
            builder.on_key_down(move |event: KeyboardEvent| {
                namui::log!("builder.on_key_down.event.code: {:?}", event.code);
                match event.code {
                    Code::ArrowUp | Code::ArrowDown => {
                        on_arrow_up_down_key(event.code);
                    }
                    Code::Enter => {
                        on_enter_down(event.code);
                    }
                    _ => {}
                }
            });
        })
    }
}
