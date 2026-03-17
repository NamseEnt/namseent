use crate::card::{Rank, Suit};
use namui::*;

use super::image::{tower_overlay_rank_image, tower_overlay_suit_image};
use super::{
    TOWER_OVERLAY_ICON_SCALE, TOWER_OVERLAY_RANK_X_RATIO, TOWER_OVERLAY_ROTATION_DEG,
    TOWER_OVERLAY_SIDE_Y_RATIO, TOWER_OVERLAY_SUIT_X_RATIO,
};

pub struct TowerSuitRankOverlay {
    pub suit: Suit,
    pub rank: Rank,
    pub wh: Wh<Px>,
    pub alpha: f32,
}

impl Component for TowerSuitRankOverlay {
    fn render(self, ctx: &RenderCtx) {
        let TowerSuitRankOverlay {
            suit,
            rank,
            wh,
            alpha,
        } = self;

        let center_y = wh.height * TOWER_OVERLAY_SIDE_Y_RATIO;
        let left_x = wh.width * TOWER_OVERLAY_SUIT_X_RATIO;
        let right_x = wh.width * TOWER_OVERLAY_RANK_X_RATIO;
        let icon_wh = Wh::new(wh.width * 0.2, wh.height * 0.2);
        let rotation = TOWER_OVERLAY_ROTATION_DEG.deg();
        let icon_scale = TOWER_OVERLAY_ICON_SCALE;

        let overlay_alpha = (alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
        let overlay_color = match suit {
            Suit::Hearts | Suit::Diamonds => Color::from_u8(220, 50, 45, overlay_alpha),
            _ => Color::from_u8(37, 26, 31, overlay_alpha),
        };

        let paint = Some(
            Paint::new(Color::WHITE).set_color_filter(ColorFilter::Blend {
                color: overlay_color,
                blend_mode: BlendMode::SrcIn,
            }),
        );

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(left_x, center_y))
                .rotate(-rotation)
                .scale(Xy::new(icon_scale, icon_scale))
                .add(namui::image(ImageParam {
                    rect: Rect::from_xy_wh(
                        Xy::new(-icon_wh.width * 0.5, -icon_wh.height * 0.5),
                        icon_wh,
                    ),
                    image: tower_overlay_suit_image(suit),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: paint.clone(),
                    },
                }));
        });

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(right_x, center_y))
                .rotate(rotation)
                .scale(Xy::new(icon_scale, icon_scale))
                .add(namui::image(ImageParam {
                    rect: Rect::from_xy_wh(
                        Xy::new(-icon_wh.width * 0.5, -icon_wh.height * 0.5),
                        icon_wh,
                    ),
                    image: tower_overlay_rank_image(rank),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint,
                    },
                }));
        });
    }
}

pub struct TowerSpriteWithOverlay {
    pub image: Image,
    pub wh: Wh<Px>,
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
    pub alpha: f32,
}

impl Component for TowerSpriteWithOverlay {
    fn render(self, ctx: &RenderCtx) {
        let square_edge = self.wh.width.min(self.wh.height);
        let square_wh = Wh::new(square_edge, square_edge);

        let square_offset = Xy::new(
            (self.wh.width - square_wh.width) * 0.5,
            (self.wh.height - square_wh.height) * 0.5,
        );

        let ctx = ctx.translate(square_offset);

        if let (Some(suit), Some(rank)) = (self.suit, self.rank) {
            ctx.add(TowerSuitRankOverlay {
                suit,
                rank,
                wh: square_wh,
                alpha: self.alpha,
            });
        }

        let paint = if self.alpha >= 0.999 {
            None
        } else {
            Some(Paint::new(Color::grayscale_alpha_f01(
                1.0,
                self.alpha.clamp(0.0, 1.0),
            )))
        };

        ctx.add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), square_wh),
            image: self.image,
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint,
            },
        }));
    }
}
