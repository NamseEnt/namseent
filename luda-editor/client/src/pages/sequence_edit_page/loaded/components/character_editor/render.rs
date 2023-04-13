use super::*;
use crate::color;
use namui_prebuilt::{table::TableCell, typography::center_text_full_height, *};
use std::iter::once;

impl CharacterEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_pose_part_list(props.wh, &mock()),
            self.render_pose_name_tooltip(),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_out(|_| {
                    namui::event::send(Event::MouseDownOutsideCharacterEditor)
                });
            })
            .with_variant_name_tooltip_destroyer(self.variant_name_tooltip.is_some())
    }

    fn render_pose_part_list(&self, wh: Wh<Px>, pose_file: &PoseFile) -> namui::RenderingTree {
        const PADDING: Px = px(8.0);
        table::padding(PADDING, |wh| {
            table::vertical(
                pose_file
                    .parts
                    .iter()
                    .flat_map(|pose_part| render_pose_part_group(wh.width, pose_part)),
            )(wh)
        })(wh)
    }

    fn render_pose_name_tooltip(&self) -> namui::RenderingTree {
        match &self.variant_name_tooltip {
            Some(variant_name_tooltip) => variant_name_tooltip.render(),
            None => RenderingTree::Empty,
        }
    }
}

fn render_pose_part_group(width: Px, pose_part: &PosePart) -> Vec<TableCell> {
    const TITLE_BAR_HEIGHT: Px = px(32.0);
    const THUMBNAIL_WH: Wh<Px> = Wh {
        width: px(96.0),
        height: px(96.0),
    };
    const PADDING: Px = px(4.0);

    fn render_thumbnail(pose_variant: &PoseVariant) -> TableCell {
        table::fixed(THUMBNAIL_WH.width, |wh| {
            table::padding(PADDING, |wh| {
                render([
                    simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                        .with_mouse_cursor(MouseCursor::Pointer),
                    simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
                ])
                .with_variant_name_tooltip(pose_variant.name.clone())
            })(wh)
        })
    }

    let title_bar = table::fixed(TITLE_BAR_HEIGHT, |wh| {
        table::horizontal_padding(PADDING, |wh| {
            render([
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND),
                center_text_full_height(wh, pose_part.name.clone(), color::STROKE_NORMAL),
            ])
        })(wh)
    });
    let divider = table::fixed(TITLE_BAR_HEIGHT, |_wh| RenderingTree::Empty);
    let no_selection_thumbnail = table::fixed(THUMBNAIL_WH.width, |wh| {
        table::padding(PADDING, |wh| {
            render([
                simple_rect(wh, Color::TRANSPARENT, 0.px(), color::BACKGROUND)
                    .with_mouse_cursor(MouseCursor::Pointer),
                simple_rect(wh, color::STROKE_NORMAL, 1.px(), Color::TRANSPARENT),
            ])
        })(wh)
    });

    let max_thumbnails_per_row = (width / (THUMBNAIL_WH.width)).floor() as usize;
    let chunks = pose_part.variants.chunks_exact(max_thumbnails_per_row);
    let chunk_remainder = chunks.remainder();
    let last_variant_row = table::fixed(THUMBNAIL_WH.height, |wh| {
        table::horizontal(
            chunk_remainder
                .iter()
                .map(|variant| render_thumbnail(variant))
                .chain(once(no_selection_thumbnail)),
        )(wh)
    });
    let variant_rows = chunks.map(|row| {
        table::fixed(THUMBNAIL_WH.height, |wh| {
            table::horizontal(row.iter().map(|variant| render_thumbnail(variant)))(wh)
        })
    });

    once(title_bar)
        .chain(variant_rows)
        .chain(once(last_variant_row))
        .chain(once(divider))
        .collect()
}

fn mock() -> PoseFile {
    PoseFile {
        parts: vec![
            PosePart {
                name: "PosePart name 0".to_string(),
                variants: vec![
                    PoseVariant {
                        name: "PoseVariant name 0".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 1".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 2".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 3".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 4".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 5".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 6".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 7".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 8".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 9".to_string(),
                    },
                ],
            },
            PosePart {
                name: "PosePart name 1".to_string(),
                variants: vec![
                    PoseVariant {
                        name: "PoseVariant name 0".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 1".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 2".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 3".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 4".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 5".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 6".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 7".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 8".to_string(),
                    },
                ],
            },
        ],
    }
}

struct PoseFile {
    parts: Vec<PosePart>,
}
struct PosePart {
    name: String,
    variants: Vec<PoseVariant>,
}
struct PoseVariant {
    name: String,
}

trait WithVariantNameTooltip {
    fn with_variant_name_tooltip(self, text: String) -> Self;
    fn with_variant_name_tooltip_destroyer(self, active: bool) -> Self;
}
impl WithVariantNameTooltip for namui::RenderingTree {
    fn with_variant_name_tooltip(self, name: String) -> Self {
        self.attach_event(|builder| {
            let name = name.clone();
            builder.on_mouse_move_in(move |event| {
                event.stop_propagation();
                namui::event::send(InternalEvent::OpenVariantNameTooltip {
                    global_xy: event.global_xy,
                    pose_name: name.clone(),
                })
            });
        })
    }
    fn with_variant_name_tooltip_destroyer(self, active: bool) -> Self {
        match active {
            true => self.attach_event(|builder| {
                builder
                    .on_mouse_move_in(|_event| {
                        namui::event::send(InternalEvent::CloseVariantNameTooltip);
                    })
                    .on_mouse_move_out(|_event| {
                        namui::event::send(InternalEvent::CloseVariantNameTooltip);
                    });
            }),
            false => self,
        }
    }
}

impl VariantNameTooltip {
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
