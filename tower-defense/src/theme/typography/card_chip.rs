use crate::{
    card::{Rank, Suit}, theme::palette,
};
use namui::*;

#[derive(Debug, Clone, Copy)]
pub enum CardChipContent {
    Suit(Suit),
    Rank(Rank),
}

impl CardChipContent {
    fn image(self) -> Image {
        match self {
            CardChipContent::Suit(suit) => suit.hand_drawn_image(),
            CardChipContent::Rank(rank) => rank.hand_drawn_image(),
        }
    }
}

pub(super) const CARD_CHIP_RATIO: f32 = 0.78;

pub(super) fn render_card_chip(content: CardChipContent, wh: Wh<Px>) -> RenderingTree {
    let height = wh.height.as_f32();
    let border_width = px((height * 0.06).max(1.0));
    let round_radius = px(height * 0.12);
    let inset = px(height * 0.04) + border_width;

    let background = rect(RectParam {
        rect: wh.to_rect(),
        style: RectStyle {
            stroke: Some(RectStroke {
                color: palette::OUTLINE,
                width: border_width,
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            round: Some(RectRound {
                radius: round_radius,
            }),
        },
    });

    let inner_rect = Rect::Xywh {
        x: inset,
        y: inset,
        width: wh.width - inset * 2.0,
        height: wh.height - inset * 2.0,
    };
    let content_image = namui::image(ImageParam {
        rect: inner_rect,
        image: content.image(),
        style: ImageStyle {
            fit: ImageFit::Contain,
            paint: None,
        },
    });

    namui::render(vec![content_image, background])
}
