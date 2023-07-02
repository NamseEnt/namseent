use super::*;

pub trait WithTooltip {
    fn with_tooltip(self, text: String) -> Self;
    fn with_tooltip_destroyer(self, active: bool) -> Self;
}
impl WithTooltip for namui::RenderingTree {
    fn with_tooltip(self, name: String) -> Self {
        self.attach_event(|builder| {
            let name = name.clone();
            builder.on_mouse_move_in(move |event: MouseEvent| {
                event.stop_propagation();
                namui::event::send(InternalEvent::OpenTooltip {
                    global_xy: event.global_xy,
                    text: name.clone(),
                })
            });
        })
    }
    fn with_tooltip_destroyer(self, active: bool) -> Self {
        match active {
            true => self.attach_event(|builder| {
                builder
                    .on_mouse_move_in(|_event| {
                        namui::event::send(InternalEvent::CloseTooltip);
                    })
                    .on_mouse_move_out(|_event| {
                        namui::event::send(InternalEvent::CloseTooltip);
                    });
            }),
            false => self,
        }
    }
}

impl Tooltip {
    pub fn render(&self) -> namui::RenderingTree {
        const OFFSET: Px = px(16.0);
        let tooltip = text(TextParam {
            text: self.text.clone(),
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
