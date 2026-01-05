use crate::card::{Rank, Suit};
use crate::game_state::{
    debug_tools::{
        add_tower_card::get_expected_tower_for_stage,
        add_upgrade::{UpgradeCategory, get_expected_upgrade_for_stage},
        spiral_place::place_selected_tower_in_spiral,
        state_snapshot,
    },
    mutate_game_state,
    tower::TowerTemplate,
    upgrade::Upgrade,
};
use crate::theme::{
    button::{Button, ButtonVariant},
    typography::{self, paragraph},
};
use namui::*;
use rand::{Rng, thread_rng};

/// Runs: snapshot -> place expected tower -> expected upgrade -> spiral place -> defense
pub struct AutoSetupButton {
    pub width: Px,
}

impl Component for AutoSetupButton {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(
            Button::new(
                Wh::new(self.width, 44.px()),
                &|| {
                    mutate_game_state(|gs| {
                        state_snapshot::save_snapshot_from_state(gs);

                        let expected_tower_kind = get_expected_tower_for_stage(gs.stage);
                        let template = TowerTemplate::new(expected_tower_kind, Suit::Spades, Rank::Ace);
                        gs.goto_placing_tower(template);

                        let (expected_rarity, expected_category) = get_expected_upgrade_for_stage(gs.stage);
                        let upgrade = if expected_category == UpgradeCategory::Random {
                            crate::game_state::upgrade::generate_upgrade(gs, expected_rarity)
                        } else {
                            let kind = expected_category.generate_upgrade_kind(expected_rarity);
                            let value = thread_rng().gen_range(0.0..=1.0);
                            Upgrade {
                                kind,
                                rarity: expected_rarity,
                                value: value.into(),
                            }
                        };
                        gs.upgrade_state.upgrade(upgrade);

                        place_selected_tower_in_spiral(gs);
                        gs.goto_defense();
                    });
                },
                &|wh, text_color, ctx| {
                    ctx.add(
                        paragraph(
                            "Auto setup: snapshot → place expected tower → expected upgrade → spiral place → defense",
                        )
                        .color(text_color)
                        .align(typography::TextAlign::Center { wh })
                        .build(),
                    );
                },
            )
            .variant(ButtonVariant::Contained),
        );
    }
}
