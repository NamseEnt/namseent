use super::*;
use crate::color;
use namui_prebuilt::*;

impl CharacterPicker {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_character_list(props),
            self.render_pose_name_tooltip(),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_out(|_| {
                    namui::event::send(Event::MouseDownOutsideCharacterPicker)
                });
            })
            .with_pose_name_tooltip_destroyer(self.pose_name_tooltip.is_some())
    }

    fn render_character_list(&self, props: Props) -> namui::RenderingTree {
        const CHARACTER_THUMBNAIL_WH: Wh<Px> = Wh {
            width: px(96.0),
            height: px(144.0),
        };
        const PADDING: Px = px(8.0);

        table::padding(PADDING, |wh| {
            let max_items_per_row =
                (props.wh.width / (CHARACTER_THUMBNAIL_WH.width)).floor() as usize;
            table::vertical(self.pose_files.chunks(max_items_per_row).map(|pose_files| {
                table::fixed(CHARACTER_THUMBNAIL_WH.height, |wh| {
                    table::horizontal(pose_files.iter().map(|pose_file| {
                        table::fixed(CHARACTER_THUMBNAIL_WH.width, |wh| {
                            table::padding(PADDING, |wh| {
                                render([
                                    image(ImageParam {
                                        rect: wh.to_rect(),
                                        source: ImageSource::Url(pose_file.thumbnail_url()),
                                        style: ImageStyle {
                                            fit: ImageFit::Contain,
                                            paint_builder: None,
                                        },
                                    })
                                    .with_mouse_cursor(MouseCursor::Pointer)
                                    .with_pose_name_tooltip(pose_file.name.clone()),
                                    simple_rect(
                                        wh,
                                        color::STROKE_NORMAL,
                                        1.px(),
                                        Color::TRANSPARENT,
                                    ),
                                ])
                            })(wh)
                        })
                    }))(wh)
                })
            }))(wh)
        })(props.wh)
    }

    fn render_pose_name_tooltip(&self) -> namui::RenderingTree {
        match &self.pose_name_tooltip {
            Some(pose_name_tooltip) => pose_name_tooltip.render(),
            None => RenderingTree::Empty,
        }
    }
}

trait WithPoseNameTooltip {
    fn with_pose_name_tooltip(self, text: String) -> Self;
    fn with_pose_name_tooltip_destroyer(self, active: bool) -> Self;
}
impl WithPoseNameTooltip for namui::RenderingTree {
    fn with_pose_name_tooltip(self, name: String) -> Self {
        self.attach_event(|builder| {
            let name = name.clone();
            builder.on_mouse_move_in(move |event| {
                event.stop_propagation();
                namui::event::send(InternalEvent::OpenPoseNameTooltip {
                    global_xy: event.global_xy,
                    pose_name: name.clone(),
                })
            });
        })
    }
    fn with_pose_name_tooltip_destroyer(self, active: bool) -> Self {
        match active {
            true => self.attach_event(|builder| {
                builder
                    .on_mouse_move_in(|_event| {
                        namui::event::send(InternalEvent::ClosePoseNameTooltip);
                    })
                    .on_mouse_move_out(|_event| {
                        namui::event::send(InternalEvent::ClosePoseNameTooltip);
                    });
            }),
            false => self,
        }
    }
}

impl PoseNameTooltip {
    fn render(&self) -> namui::RenderingTree {
        const OFFSET: Px = px(16.0);
        let tooltip = text(TextParam {
            text: self.pose_name.clone(),
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font_type: FontType {
                size: 12.int_px(),
                serif: false,
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },

            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: color::STROKE_NORMAL,
                background: Some(TextStyleBackground {
                    color: color::BACKGROUND,
                    margin: Some(Ltrb {
                        left: 4.px(),
                        top: 4.px(),
                        right: 4.px(),
                        bottom: 4.px(),
                    }),
                }),
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: None,
        });

        let tooltip_bounding_box = tooltip.get_bounding_box();
        if tooltip_bounding_box.is_none() {
            return RenderingTree::Empty;
        }

        let screen_size = screen::size();
        let max_xy = (screen_size - tooltip_bounding_box.unwrap().wh()).as_xy();

        on_top(absolute(
            (self.global_xy.x + OFFSET).min(max_xy.x).max(0.px()),
            (self.global_xy.y + OFFSET).min(max_xy.y).max(0.px()),
            tooltip,
        ))
    }
}
