use super::shared::render_top_left_rank_and_suit;
use crate::{
    asset_loader::get_face_card_asset,
    card::{Card, Rank},
    game_state::hand::shared::render_background_rect,
    icon::{Icon, IconKind, IconSize},
    theme::typography::{FontSize, TextAlign, headline},
};
use namui::*;

pub(super) struct RenderCard<'a> {
    pub wh: Wh<Px>,
    pub card: &'a Card,
}

impl Component for RenderCard<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, card } = self;

        // 좌상단에 숫자와 문양 수직 배치
        render_top_left_rank_and_suit(ctx, card.rank, card.suit);

        // 중앙에 알맞은 수의 문양 배치 (숫자 카드만)
        if !card.rank.is_face() {
            self.render_center_suits(ctx, wh, card);
        } else {
            // JQK 그림카드 이미지 렌더링
            self.render_face_card(ctx, wh, card);
        }

        render_background_rect(ctx, wh);
    }
}

impl<'a> RenderCard<'a> {
    fn render_center_suits(&self, ctx: &RenderCtx, wh: Wh<Px>, card: &'a Card) {
        let center_area = Rect::Xywh {
            x: px(36.0),
            y: px(36.0),
            width: wh.width - px(72.0),
            height: wh.height - px(72.0),
        };

        let suit_positions = self.get_suit_positions_for_rank(card.rank, center_area);
        let suit_size = px(24.0);

        let ctx = ctx.translate(center_area.xy() - Xy::single(suit_size * 0.5));
        for position in suit_positions {
            ctx.translate(position).add(
                Icon::new(IconKind::Suit { suit: card.suit })
                    .wh(Wh::new(suit_size, suit_size))
                    .size(IconSize::Custom { size: suit_size }),
            );
        }
    }

    fn render_face_card(&self, ctx: &RenderCtx, wh: Wh<Px>, card: &'a Card) {
        let center_area = Rect::Xywh {
            x: px(12.0),
            y: px(12.0),
            width: wh.width - px(24.0),
            height: wh.height - px(24.0),
        };

        if let Some(face_image) = get_face_card_asset((card.rank, card.suit)) {
            ctx.add(image(ImageParam {
                image: face_image.clone(),
                rect: center_area,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
            return;
        }

        // fallback: asset loader가 없거나 이미지가 없는 경우 기존 텍스트 표시
        ctx.translate(Xy::new(
            center_area.width() / 2.0,
            center_area.height() / 2.0,
        ))
        .add(
            headline("TODO\nFace Card".to_string())
                .size(FontSize::Small)
                .color(Color::BLACK)
                .align(TextAlign::Center {
                    wh: Wh::new(center_area.width(), center_area.height()),
                })
                .build(),
        );
    }

    fn get_suit_positions_for_rank(&self, rank: Rank, center_area: Rect<Px>) -> Vec<Xy<Px>> {
        let left_x = px(0.0);
        let center_x = center_area.width() * 0.5;
        let right_x = center_area.width();

        let y0 = px(0.0);
        let y1 = center_area.height() * 0.16665;
        let y2 = center_area.height() * 0.25;
        let y3 = center_area.height() * 0.33333;
        let y4 = center_area.height() * 0.5;
        let y5 = center_area.height() * 0.66666;
        let y6 = center_area.height() * 0.75;
        let y7 = center_area.height() * 0.83331;
        let y8 = center_area.height();

        match rank {
            Rank::Seven => vec![
                Xy::new(left_x, y0),
                Xy::new(right_x, y0),
                Xy::new(center_x, y2),
                Xy::new(left_x, y4),
                Xy::new(right_x, y4),
                Xy::new(left_x, y8),
                Xy::new(right_x, y8),
            ],
            Rank::Eight => vec![
                Xy::new(left_x, y0),
                Xy::new(right_x, y0),
                Xy::new(center_x, y2),
                Xy::new(left_x, y4),
                Xy::new(right_x, y4),
                Xy::new(center_x, y6),
                Xy::new(left_x, y8),
                Xy::new(right_x, y8),
            ],
            Rank::Nine => vec![
                Xy::new(left_x, y0),
                Xy::new(right_x, y0),
                Xy::new(left_x, y3),
                Xy::new(right_x, y3),
                Xy::new(center_x, y4),
                Xy::new(left_x, y5),
                Xy::new(right_x, y5),
                Xy::new(left_x, y8),
                Xy::new(right_x, y8),
            ],
            Rank::Ten => vec![
                Xy::new(left_x, y0),
                Xy::new(right_x, y0),
                Xy::new(center_x, y1),
                Xy::new(left_x, y3),
                Xy::new(right_x, y3),
                Xy::new(left_x, y5),
                Xy::new(right_x, y5),
                Xy::new(center_x, y7),
                Xy::new(left_x, y8),
                Xy::new(right_x, y8),
            ],
            Rank::Ace => vec![
                // 중앙 1개만
                Xy::new(center_x, y4),
            ],
            Rank::Jack | Rank::Queen | Rank::King => {
                // 이 경우는 발생하지 않아야 함 (is_face()로 이미 필터링됨)
                vec![]
            }
        }
    }
}
