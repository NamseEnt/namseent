use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconKind, IconSize},
    theme::{
        palette,
        typography::{self, FontSize},
    },
};
use namui::*;

/// suit에 따른 색상을 반환하는 헬퍼 함수
pub fn get_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Spades | Suit::Clubs => Color::BLACK,
        Suit::Hearts | Suit::Diamonds => palette::RED,
    }
}

/// 좌상단에 rank와 suit를 수직 배치로 렌더링
pub(super) fn render_top_left_rank_and_suit(ctx: &RenderCtx, rank: Rank, suit: Suit) {
    let padding = px(4.0);
    let rank_font_size = FontSize::Small;
    let icon_wh = Wh::new(20.px(), 12.px());

    // suit에 따른 색상 결정
    let text_color = get_suit_color(suit);

    let ctx = ctx.translate(Xy::new(padding, padding));
    // 숫자 렌더링
    let rank_text = rank.to_string();
    ctx.add(
        typography::headline()
            .size(rank_font_size)
            .color(text_color)
            .text(&rank_text)
            .render_center(icon_wh),
    );

    // 문양 아이콘 렌더링 (숫자 아래)
    ctx.translate(Xy::new(0.px(), icon_wh.height + padding))
        .add(
            Icon::new(IconKind::Suit { suit })
                .wh(icon_wh)
                .size(IconSize::Custom {
                    size: icon_wh.height,
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
