use crate::game_state::GameState;
use crate::game_state::upgrade::Upgrade;
use rand::RngCore;
use std::cmp::Ordering;

/// Strategy for selecting treasure options.
pub trait TreasureStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn select_treasure(
        &self,
        _game_state: &GameState,
        options: &[Upgrade],
        rng: &mut dyn RngCore,
    ) -> usize;
}

/// Strategy that picks treasures based on current upgrade state and stage economy.
pub struct SynergyTreasureStrategy;

impl TreasureStrategy for SynergyTreasureStrategy {
    fn name(&self) -> &str {
        "synergy_treasure"
    }

    fn select_treasure(
        &self,
        game_state: &GameState,
        options: &[Upgrade],
        _rng: &mut dyn RngCore,
    ) -> usize {
        options
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                self.score_option(game_state, a)
                    .partial_cmp(&self.score_option(game_state, b))
                    .unwrap_or(Ordering::Equal)
            })
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

impl SynergyTreasureStrategy {
    fn score_option(&self, game_state: &GameState, option: &Upgrade) -> f32 {
        let stage = game_state.stage as f32;
        let base_value = match option {
            Upgrade::Cat(..) => {
                7.0 + (8 - game_state.upgrade_state.gold_earn_plus()) as f32 * 0.8
            }
            Upgrade::Backpack(..) => {
                6.5 + (2 - game_state.upgrade_state.shop_slot_expand()) as f32 * 1.2
            }
            Upgrade::DiceBundle(..) => {
                7.5 + (4 - game_state.upgrade_state.dice_chance_plus()) as f32 * 1.3
            }
            Upgrade::EnergyDrink(..) => {
                6.5 + (15 - game_state.upgrade_state.shop_item_price_minus()) as f32 * 0.2
            }
            Upgrade::FourLeafClover(..) => {
                if game_state.upgrade_state.shorten_straight_flush_to_4_cards() {
                    3.0
                } else {
                    5.5
                }
            }
            Upgrade::Rabbit(..) => {
                if game_state.upgrade_state.skip_rank_for_straight() {
                    3.0
                } else {
                    5.0
                }
            }
            Upgrade::BlackWhite(..) => {
                if game_state.upgrade_state.treat_suits_as_same() {
                    3.0
                } else {
                    5.0
                }
            }
            Upgrade::Eraser(..) => {
                5.5 + (5 - game_state.upgrade_state.removed_number_rank_count()) as f32 * 0.7
            }
            _ => 4.0,
        };

        let mut score = base_value;

        if game_state.stage <= 12
            && matches!(
                option,
                Upgrade::Backpack(..) | Upgrade::DiceBundle(..) | Upgrade::Cat(..)
            )
        {
            score += 1.5;
        }

        if game_state.hp < game_state.config.player.max_hp * 0.5
            && matches!(option, Upgrade::EnergyDrink(..) | Upgrade::Cat(..))
        {
            score += 1.5;
        }

        if game_state.upgrade_state.shop_item_price_minus() == 0
            && matches!(option, Upgrade::EnergyDrink(..))
        {
            score += 1.0;
        }

        score * (1.0 + (stage / 50.0).min(0.8))
    }
}
