use super::*;

pub struct RowBlock<'a> {
    pub(crate) block: &'a Block,
}
impl<'a> RowBlock<'a> {
    pub(crate) fn new(block: &'a Block) -> Self {
        Self { block }
    }

    pub(crate) fn render(&self, width: Px) -> RenderingTree {
        let border_round = 2.px();
        let wh = Wh::new(width, self.height());

        let border = {
            let border_rect = Rect::from_xy_wh(
                Xy::new(BLOCK_OUTER_LEFT_RIGHT_MARGIN, BLOCK_OUTER_TOP_MARGIN),
                wh - Wh::new(
                    BLOCK_OUTER_LEFT_RIGHT_MARGIN * 2.0,
                    BLOCK_OUTER_TOP_MARGIN + BLOCK_OUTER_BOTTOM_MARGIN,
                ),
            );
            namui::rect(RectParam {
                rect: border_rect,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::grayscale_f01(0.8),
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: None,
                    round: Some(RectRound {
                        radius: border_round,
                    }),
                },
            })
        };
        let title = {
            namui::text(TextParam {
                text: self.block.title.clone(),
                x: BLOCK_OUTER_LEFT_RIGHT_MARGIN * 2,
                y: BLOCK_OUTER_TOP_MARGIN,
                align: TextAlign::Left,
                baseline: TextBaseline::Middle,
                font_type: crate::typography::REGULAR,
                style: TextStyle {
                    color: Color::grayscale_f01(0.8),
                    background: Some(TextStyleBackground {
                        color: crate::color::BACKGROUND,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                max_width: None,
            })
        };
        let content = {
            let mut content = vec![];
            let mut y = 0.px();
            for line in &self.block.lines {
                content.push(translate(0.px(), y, line.render()));
                y += line.height() + LINE_GAP;
            }
            render(content)
        };

        namui::render([
            border,
            title,
            translate(
                BLOCK_OUTER_LEFT_RIGHT_MARGIN + BLOCK_INNER_MARGIN,
                BLOCK_OUTER_TOP_MARGIN + BLOCK_INNER_MARGIN,
                content,
            ),
        ])
    }

    pub(crate) fn height(&self) -> Px {
        self.block.vertical_margins()
            + self
                .block
                .lines
                .iter()
                .map(|line| line.height())
                .sum::<Px>()
            + LINE_GAP * (self.block.lines.len() - 1)
    }
}
