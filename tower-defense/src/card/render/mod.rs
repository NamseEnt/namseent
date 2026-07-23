mod render_card;
mod render_tower_card;

use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconKind, IconSize},
    theme::{
        palette,
        typography::{FontSize, memoized_text},
    },
};
use namui::*;
pub use render_card::RenderCard;
pub use render_tower_card::RenderTowerCard;

/// suit에 따른 색상을 반환하는 헬퍼 함수
pub fn get_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::BLACK,
        Suit::Hearts | Suit::Diamonds => palette::RED,
    }
}

pub(super) fn render_top_left_rank_and_suit_with_opacity(
    ctx: &RenderCtx,
    rank: Rank,
    suit: Suit,
    opacity: f32,
) {
    let padding = px(4.0);
    let icon_wh = Wh::new(20.px(), 12.px());

    // suit에 따른 색상 결정
    let text_color = with_opacity(get_suit_color(suit), opacity);

    let ctx = ctx.translate(Xy::new(padding, padding + 4.px()));

    ctx.add(memoized_text((&rank, &text_color), |mut builder| {
        builder
            .headline()
            .size(FontSize::Small)
            .color(text_color)
            .text(rank.to_string())
            .render_center(icon_wh)
    }));

    // 문양 아이콘 렌더링 (숫자 아래)
    ctx.translate(Xy::new(0.px(), icon_wh.height + padding))
        .add(
            Icon::new(IconKind::Suit { suit })
                .wh(icon_wh)
                .opacity(opacity)
                .size(IconSize::Custom {
                    size: icon_wh.height,
                }),
        );
}

fn with_opacity(color: Color, opacity: f32) -> Color {
    color.with_alpha((color.a as f32 * opacity).round() as u8)
}

/// 카드/타워 배경 rect를 렌더링
pub(super) fn render_background_rect(ctx: &RenderCtx, wh: Wh<Px>) {
    render_background_rect_with_opacity(ctx, wh, 1.0);
}

pub(super) fn render_background_rect_with_opacity(ctx: &RenderCtx, wh: Wh<Px>, opacity: f32) {
    let outline_color = with_opacity(palette::OUTLINE, opacity);
    let fill_color = with_opacity(Color::WHITE, opacity);

    ctx.add(rect(RectParam {
        rect: wh.to_rect(),
        style: RectStyle {
            stroke: Some(RectStroke {
                color: outline_color,
                width: 4.px(),
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill { color: fill_color }),
            round: Some(RectRound {
                radius: palette::ROUND,
            }),
        },
    }));
}
