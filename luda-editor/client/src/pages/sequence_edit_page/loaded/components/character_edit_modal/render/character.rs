use super::*;

impl CharacterEditModal {
    pub fn render_character_list_view(
        &self,
        props: &Props,
        character_list_rect: Rect<Px>,
    ) -> RenderingTree {
        let items = props
            .characters
            .iter()
            .map(|character| WithAddButton::Item(character))
            .chain(std::iter::once(WithAddButton::AddButton))
            .collect::<Vec<_>>();

        self.character_list_view.render(list_view::Props {
            xy: character_list_rect.xy(),
            height: props.wh.height,
            scroll_bar_width: 10.px(),
            item_wh: Wh::new(character_list_rect.width(), 80.px()),
            items,
            item_render: |wh, item| {
                let border = simple_rect(wh, Color::WHITE, 1.px(), Color::BLACK);

                let content = match item {
                    WithAddButton::Item(character) => {
                        let is_selected = self.character_id == Some(character.id());

                        let (is_character_cell_in_editing_text_mode, editing_text) =
                            if let Some(EditingTextMode::CharacterName { character_id, text }) =
                                &self.editing_text_mode
                            {
                                (*character_id == character.id(), Some(text))
                            } else {
                                (false, None)
                            };

                        let stroke_color = if is_selected {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        };
                        let fill_color = if is_selected {
                            Color::WHITE
                        } else {
                            Color::BLACK
                        };

                        render([
                            simple_rect(wh, stroke_color, 1.px(), fill_color),
                            table::vertical([
                                table::ratio(1.0, |wh| {
                                    match get_character_main_image_url(character.id()) {
                                        Ok(url) => namui::image(ImageParam {
                                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                                            source: ImageSource::Url(url),
                                            style: ImageStyle {
                                                fit: ImageFit::Contain,
                                                paint_builder: None,
                                            },
                                        }),
                                        Err(error) => {
                                            namui::log!(
                                                "Failed to get character main image source: {}
    character_id: {}",
                                                error,
                                                character.id()
                                            );
                                            RenderingTree::Empty
                                        }
                                    }
                                }),
                                table::fixed(20.px(), |wh| {
                                    match is_character_cell_in_editing_text_mode {
                                        true => self.text_input.render(text_input::Props {
                                            text: editing_text.unwrap().clone(),
                                            rect: Rect::from_xy_wh(Xy::zero(), wh),
                                            rect_style: RectStyle {
                                                stroke: Some(RectStroke {
                                                    color: stroke_color,
                                                    width: 1.px(),
                                                    border_position: BorderPosition::Inside,
                                                }),
                                                fill: Some(RectFill { color: fill_color }),
                                                round: None,
                                            },
                                            text_align: TextAlign::Left,
                                            text_baseline: TextBaseline::Middle,
                                            font_type: FontType {
                                                serif: false,
                                                size: 12.int_px(),
                                                language: Language::Ko,
                                                font_weight: FontWeight::MEDIUM,
                                            },
                                            text_style: TextStyle {
                                                border: None,
                                                drop_shadow: None,
                                                color: stroke_color,
                                                background: None,
                                            },
                                            event_handler: None,
                                        }),
                                        false => typography::body::center(
                                            wh,
                                            &character.name,
                                            stroke_color,
                                        ),
                                    }
                                }),
                            ])(wh),
                        ])
                        .attach_event(move |builder| {
                            if is_character_cell_in_editing_text_mode {
                                return;
                            }
                            let cut_id = self.cut_id.clone();
                            let character_id = character.id();
                            let character_name = character.name.clone();
                            builder.on_mouse_down_in(move |event| {
                                if let Some(MouseButton::Left) = event.button {
                                    namui::event::send(Event::CharacterSelected {
                                        cut_id: cut_id.clone(),
                                        character_id,
                                    });
                                } else if let Some(MouseButton::Right) = event.button {
                                    namui::event::send(InternalEvent::CharacterRightClicked {
                                        character_id,
                                        mouse_global_xy: event.global_xy,
                                        name: character_name.clone(),
                                    });
                                }
                            });
                        })
                    }
                    WithAddButton::AddButton => button::text_button(
                        Rect::from_xy_wh(Xy::zero(), wh),
                        "+",
                        Color::WHITE,
                        Color::WHITE,
                        1.px(),
                        Color::BLACK,
                        || namui::event::send(Event::AddCharacterClicked),
                    ),
                };

                render([border, content])
            },
        })
    }
}
