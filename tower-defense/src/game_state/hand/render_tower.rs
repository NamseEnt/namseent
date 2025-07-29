use crate::{
    asset_loader::TOWER_ASSET_LOADER_ATOM,
    game_state::tower::{AnimationKind, TowerTemplate},
    icon::{Icon, IconKind, IconSize},
    theme::{
        palette,
        typography::{FontSize, TextAlign, headline},
    },
};
use namui::*;

pub(super) struct RenderTower {
    pub wh: Wh<Px>,
    pub tower_template: TowerTemplate,
}
impl Component for RenderTower {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let (tower_asset_loader_atom, _) = ctx.atom(&TOWER_ASSET_LOADER_ATOM);
        let tower_image = tower_asset_loader_atom.get(tower_template.kind, AnimationKind::Idle1);

        // 좌상단에 rank와 suit 수직 배치
        let padding = px(4.0);
        let rank_font_size = FontSize::Small;
        let suit_icon_size = px(16.0);

        // 숫자 렌더링
        ctx.translate(Xy::new(padding, padding)).add(
            headline(tower_template.rank.to_string())
                .size(rank_font_size)
                .align(TextAlign::LeftTop)
                .build(),
        );

        // 문양 아이콘 렌더링 (숫자 아래)
        ctx.translate(Xy::new(padding, padding + px(20.0))).add(
            Icon::new(IconKind::Suit {
                suit: tower_template.suit,
            })
            .wh(Wh::new(suit_icon_size, suit_icon_size))
            .size(IconSize::Custom {
                size: suit_icon_size,
            }),
        );

        // 타워 이미지 렌더링
        ctx.compose(|ctx| {
            let Some(tower_image) = tower_image else {
                return;
            };

            ctx.add(image(ImageParam {
                rect: wh.to_rect(),
                image: tower_image.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
        });

        ctx.add(rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 4.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}
