use crate::{
    asset_loader,
    card::{Rank, Suit},
    game_state::{
        quest::QuestRequirement,
        tower::{AnimationKind, TowerKind},
    },
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    theme::typography::{FontSize, TextAlign, headline},
};
use namui::*;

impl QuestRequirement {
    pub fn icon(&self, wh: Wh<Px>) -> RenderingTree {
        match self {
            QuestRequirement::BuildTowerRankNew { rank, count } => namui::render([
                render_count_overlay(wh, *count),
                render_new_icon(wh),
                render_rank(wh * 0.7, *rank),
                render_barricade_tower(wh),
            ]),
            QuestRequirement::BuildTowerRank { rank, count } => namui::render([
                render_count_overlay(wh, *count),
                render_rank(wh * 0.7, *rank),
                render_barricade_tower(wh),
            ]),
            QuestRequirement::BuildTowerSuitNew { suit, count } => namui::render([
                render_count_overlay(wh, *count),
                render_new_icon(wh),
                render_suit(wh * 0.7, *suit),
                render_barricade_tower(wh),
            ]),
            QuestRequirement::BuildTowerSuit { suit, count } => namui::render([
                render_count_overlay(wh, *count),
                render_suit(wh * 0.7, *suit),
                render_barricade_tower(wh),
            ]),
            QuestRequirement::BuildTowerHandNew { hand, count } => namui::render([
                render_count_overlay(wh, *count),
                render_new_icon(wh),
                render_tower_kind(wh, *hand),
            ]),
            QuestRequirement::BuildTowerHand { hand, count } => namui::render([
                render_count_overlay(wh, *count),
                render_tower_kind(wh, *hand),
            ]),
            QuestRequirement::ClearBossRoundWithoutItems => Icon::new(IconKind::EnemyBoss)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Item).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::DealDamageWithItems { .. } => Icon::new(IconKind::AttackDamage)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Item).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::BuildTowersWithoutReroll { count } => namui::render([
                render_count_overlay(wh, *count),
                Icon::new(IconKind::Refresh)
                    .wh(wh * 0.5)
                    .size(IconSize::Custom {
                        size: wh.width * 0.5,
                    })
                    .attributes(vec![
                        IconAttribute::new(IconKind::Reject)
                            .position(IconAttributePosition::BottomRight),
                    ])
                    .to_rendering_tree(),
                render_barricade_tower(wh),
            ]),
            QuestRequirement::UseReroll { count } => {
                namui::render([render_count_overlay(wh, *count), render_refresh_icon(wh)])
            }
            QuestRequirement::SpendGold { .. } => Icon::new(IconKind::Gold)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Down).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
            QuestRequirement::EarnGold { .. } => Icon::new(IconKind::Gold)
                .wh(wh)
                .size(IconSize::Custom { size: wh.width })
                .attributes(vec![
                    IconAttribute::new(IconKind::Up).position(IconAttributePosition::BottomRight),
                ])
                .to_rendering_tree(),
        }
    }
}

fn render_new_icon(wh: Wh<Px>) -> RenderingTree {
    Icon::new(IconKind::New)
        .wh(wh)
        .size(IconSize::Custom { size: wh.width })
        .to_rendering_tree()
}

fn render_barricade_tower(wh: Wh<Px>) -> RenderingTree {
    asset_loader::get_tower_asset((TowerKind::Barricade, AnimationKind::Idle1))
        .map(|image| {
            namui::image(ImageParam {
                rect: wh.to_rect(),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
        })
        .unwrap_or_default()
}

fn render_rank(wh: Wh<Px>, rank: Rank) -> RenderingTree {
    namui::render([
        headline(rank.to_string())
            .align(TextAlign::Center { wh })
            .size(FontSize::Custom {
                size: wh.height * 0.6,
            })
            .color(Color::WHITE)
            .max_width(2560.px())
            .build()
            .into_rendering_tree(),
        namui::rect(RectParam {
            rect: wh.to_rect(),
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::from_u8(0, 0, 0, 180),
                }),
                round: Some(RectRound {
                    radius: wh.width * 0.5,
                }),
                stroke: None,
            },
        }),
    ])
}

fn render_suit(wh: Wh<Px>, suit: Suit) -> RenderingTree {
    Icon::new(IconKind::Suit { suit })
        .wh(wh)
        .size(IconSize::Custom { size: wh.width })
        .to_rendering_tree()
}

fn render_tower_kind(wh: Wh<Px>, tower_kind: TowerKind) -> RenderingTree {
    asset_loader::get_tower_asset((tower_kind, AnimationKind::Idle1))
        .map(|image| {
            namui::image(ImageParam {
                rect: wh.to_rect(),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
        })
        .unwrap_or_default()
}

fn render_refresh_icon(wh: Wh<Px>) -> RenderingTree {
    Icon::new(IconKind::Refresh)
        .wh(wh)
        .size(IconSize::Custom { size: wh.width })
        .to_rendering_tree()
}

fn render_count_overlay(wh: Wh<Px>, count: usize) -> RenderingTree {
    let overlay_size = wh * 0.3;
    let overlay_xy = Xy::new(
        wh.width - overlay_size.width,
        wh.height - overlay_size.height,
    );

    namui::translate(
        overlay_xy.x,
        overlay_xy.y,
        namui::render([
            headline(count.to_string())
                .align(TextAlign::Center { wh: overlay_size })
                .size(FontSize::Custom {
                    size: overlay_size.height * 0.6,
                })
                .color(Color::WHITE)
                .build()
                .into_rendering_tree(),
            namui::rect(RectParam {
                rect: overlay_size.to_rect(),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::from_u8(0, 0, 0, 180),
                    }),
                    round: Some(RectRound {
                        radius: overlay_size.width * 0.5,
                    }),
                    stroke: None,
                },
            }),
        ]),
    )
}
