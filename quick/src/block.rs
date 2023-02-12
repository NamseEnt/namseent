use super::*;
use namui_prebuilt::table;

enum Direction {
    Vertical,
    Horizontal,
}

pub struct Group {
    direction: Direction,
    blocks: Vec<Block>,
}

impl Group {
    pub fn render(&self, wh: Wh<Px>) -> RenderingTree {
        let mut trees = vec![];
        match self.direction {
            Direction::Vertical => {
                let mut y = 0.px();
                for block in &self.blocks {
                    let row = RowBlock::new(block);
                    let block_rendering_tree = translate(0.px(), y, row.render(wh.width));
                    trees.push(block_rendering_tree);
                    y += row.height();
                }
            }
            Direction::Horizontal => todo!(),
        }
        render(trees)
    }
}

pub fn vertical(blocks: impl IntoIterator<Item = Block>) -> Group {
    Group {
        direction: Direction::Vertical,
        blocks: blocks.into_iter().collect(),
    }
}

pub struct RowBlock<'a> {
    block: &'a Block,
}
impl<'a> RowBlock<'a> {
    fn new(block: &'a Block) -> Self {
        Self { block }
    }

    fn render(&self, width: Px) -> RenderingTree {
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
                        margin: None,
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
                y += line.height();
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

    fn height(&self) -> Px {
        self.block.vertical_margins()
            + self
                .block
                .lines
                .iter()
                .map(|line| line.height())
                .sum::<Px>()
    }
}

pub struct Line {
    items: Vec<Link>,
}
impl Line {
    fn height(&self) -> Px {
        LINE_HEIGHT
    }

    fn render(&self) -> RenderingTree {
        let mut trees = vec![];
        let mut x = 0.px();
        for item in &self.items {
            let item_rendering_tree = translate(x, 0.px(), item.render());

            x += item_rendering_tree
                .get_bounding_box()
                .map_or(0.px(), |bounding_box| bounding_box.width())
                + LINE_ITEM_GAP;

            trees.push(item_rendering_tree);
        }
        render(trees)
    }
}

pub fn line(content: impl IntoIterator<Item = Link>) -> Line {
    Line {
        items: content.into_iter().collect(),
    }
}

pub const LINE_ITEM_GAP: Px = px(4.0);

pub struct Block {
    title: String,
    lines: Vec<Line>,
}

pub const LINE_HEIGHT: Px = px(16.0);

pub const BLOCK_OUTER_TOP_MARGIN: Px = px(12.0);
pub const BLOCK_OUTER_BOTTOM_MARGIN: Px = px(4.0);
pub const BLOCK_OUTER_LEFT_RIGHT_MARGIN: Px = px(8.0);
pub const BLOCK_INNER_MARGIN: Px = px(8.0);

impl Block {
    fn vertical_margins(&self) -> Px {
        BLOCK_OUTER_TOP_MARGIN + BLOCK_OUTER_BOTTOM_MARGIN + BLOCK_INNER_MARGIN * 2
    }
}

pub fn block<'a>(title: impl ToString, lines: impl IntoIterator<Item = Line>) -> Block {
    Block {
        title: title.to_string(),
        lines: lines.into_iter().collect(),
    }

    // move |wh| {
    //     let border_round = 2.px();

    //     let border = {
    //         let border_rect =
    //             Rect::from_xy_wh(Xy::single(outer_margin), wh - Wh::single(outer_margin * 2));
    //         namui::rect(RectParam {
    //             rect: border_rect,
    //             style: RectStyle {
    //                 stroke: Some(RectStroke {
    //                     color: Color::grayscale_f01(0.8),
    //                     width: 1.px(),
    //                     border_position: BorderPosition::Inside,
    //                 }),
    //                 fill: None,
    //                 round: Some(RectRound {
    //                     radius: border_round,
    //                 }),
    //             },
    //         })
    //     };
    //     let title = {
    //         namui::text(TextParam {
    //             text: title_text.to_string(),
    //             x: outer_margin * 2,
    //             y: outer_margin,
    //             align: TextAlign::Left,
    //             baseline: TextBaseline::Middle,
    //             font_type: crate::typography::REGULAR,
    //             style: TextStyle {
    //                 color: Color::grayscale_f01(0.8),
    //                 background: Some(TextStyleBackground {
    //                     color: crate::color::BACKGROUND,
    //                     margin: None,
    //                 }),
    //                 ..Default::default()
    //             },
    //             max_width: None,
    //         })
    //     };
    //     let content = translate(
    //         outer_margin + inner_margin,
    //         outer_margin + inner_margin,
    //         content(wh - Wh::single(outer_margin * 2 + inner_margin * 2)),
    //     );
    //     namui::render([border, title, content])
    // }
}
