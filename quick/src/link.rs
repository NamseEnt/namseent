use super::*;
use namui_prebuilt::table;

pub struct Link {
    text: String,
}
impl Link {
    pub(crate) fn render(&self) -> RenderingTree {
        namui::text(TextParam {
            text: self.text.clone(),
            x: 0.px(),
            y: LINE_HEIGHT / 2.0,
            align: TextAlign::Left,
            baseline: TextBaseline::Middle,
            font_type: crate::typography::REGULAR,
            style: TextStyle {
                color: Color::grayscale_f01(1.8),
                background: Some(TextStyleBackground {
                    color: crate::color::BACKGROUND,
                    margin: None,
                }),
                underline: Some(
                    PaintBuilder::new()
                        .set_stroke_width(1.px())
                        .set_color(Color::grayscale_alpha_f01(0.8, 0.5)),
                ),
                ..Default::default()
            },
            max_width: None,
        })
    }
}

pub fn link(link_text: impl ToString) -> Link {
    Link {
        text: link_text.to_string(),
    }
    // table::fit(table::FitAlign::LeftTop, {
    //     let outer_margin = 4.px();

    //     namui::text(TextParam {
    //         text: link_text.to_string(),
    //         x: outer_margin * 2,
    //         y: outer_margin,
    //         align: TextAlign::Left,
    //         baseline: TextBaseline::Top,
    //         font_type: crate::typography::REGULAR,
    //         style: TextStyle {
    //             color: Color::grayscale_f01(0.8),
    //             background: Some(TextStyleBackground {
    //                 color: crate::color::BACKGROUND,
    //                 margin: None,
    //             }),
    //             underline: Some(
    //                 PaintBuilder::new()
    //                     .set_stroke_width(1.px())
    //                     .set_color(Color::grayscale_alpha_f01(0.8, 0.5)),
    //             ),
    //             ..Default::default()
    //         },
    //         max_width: None,
    //     })
    // })
}
