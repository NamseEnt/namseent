use super::*;
use crate::{
    card::render::render_engraving_overlay,
    card::render::render_polish::{polish_halo_config, render_polish_overlay},
    game_state::tower::{
        AnimationKind, TowerTemplate,
        render::{TowerImage, TowerSpriteWithOverlay},
    },
    theme::card_halo_fx::CardHaloFx,
};
use namui::*;

pub struct RenderTowerCard<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}

struct RenderTowerCardInner<'a> {
    wh: Wh<Px>,
    tower_template: &'a TowerTemplate,
}

impl Component for RenderTowerCard<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let bonus_pct = tower_template.card_polish_pct();
        let tower_kind = tower_template.kind;
        let engraving = tower_template
            .used_cards()
            .iter()
            .find_map(|card| card.engraving());
        let on_enter = move || {
            let mut words = Vec::new();
            if bonus_pct > 0.0 {
                words.push(crate::l10n::word::Word::Polish(Some(bonus_pct)));
            }
            if let Some(engraving) = engraving {
                words.push(crate::l10n::word::Word::Engraving(Some(engraving)));
            }
            Some(crate::tooltip::TooltipContent::Tower {
                kind: tower_kind,
                words,
            })
        };

        ctx.add(crate::tooltip::WithHoverArea {
            component_key: "tower_card_tooltip",
            component: RenderTowerCardInner { wh, tower_template },
            placement: crate::tooltip::TooltipPlacement::Above,
            on_enter,
            on_exit: || {},
        });

        if let Some((color, strength)) = polish_halo_config(bonus_pct) {
            let seed = (tower_template.kind as u32 as f32 * 0.618034).fract();
            ctx.add(CardHaloFx {
                wh,
                radius: wh.width * 0.22,
                color,
                strength,
                seed,
            });
        }
    }
}

impl<'a> Component for RenderTowerCardInner<'a> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;
        let engraving = tower_template
            .used_cards()
            .iter()
            .find_map(|card| card.engraving());
        render_polish_overlay(ctx, wh, tower_template.card_polish_pct(), 1.0);
        render_engraving_overlay(ctx, wh, engraving, 1.0);

        let tower_image = (tower_template.kind, AnimationKind::Idle1).image();

        ctx.add(TowerSpriteWithOverlay {
            image: tower_image,
            wh,
            suit: tower_template.suit,
            rank: tower_template.rank,
            alpha: 1.0,
        });

        render_background_rect(ctx, wh);
    }
}
