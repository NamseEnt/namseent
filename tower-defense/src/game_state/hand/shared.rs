use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconKind, IconSize},
    theme::{
        palette,
        typography::{FontSize, TextAlign, headline},
    },
};
use namui::*;

/// 좌상단에 rank와 suit를 수직 배치로 렌더링
pub(super) fn render_top_left_rank_and_suit(ctx: &RenderCtx, rank: Rank, suit: Suit) {
    let padding = px(4.0);
    let rank_font_size = FontSize::Small;
    let suit_icon_size = px(16.0);

    // suit에 따른 색상 결정
    let text_color = match suit {
        Suit::Spades | Suit::Clubs => Color::BLACK,
        Suit::Hearts | Suit::Diamonds => palette::RED,
    };

    let ctx = ctx.translate(Xy::new(padding, padding));
    // 숫자 렌더링
    ctx.add(
        headline(rank.to_string())
            .size(rank_font_size)
            .color(text_color)
            .align(TextAlign::Center {
                wh: Wh::single(suit_icon_size),
            })
            .build(),
    );

    // 문양 아이콘 렌더링 (숫자 아래)
    ctx.translate(Xy::new(0.px(), suit_icon_size)).add(
        Icon::new(IconKind::Suit { suit })
            .wh(Wh::single(suit_icon_size))
            .size(IconSize::Custom {
                size: suit_icon_size,
            }),
    );
}

/// 카드/타워 배경 rect를 렌더링
pub(super) fn render_background_rect(ctx: &RenderCtx, wh: Wh<Px>) {
    ctx.add(rect(RectParam {
        rect: wh.to_rect(),
        style: RectStyle {
            stroke: Some(RectStroke {
                color: palette::OUTLINE,
                width: 4.px(),
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            round: Some(RectRound {
                radius: palette::ROUND,
            }),
        },
    }));
}
