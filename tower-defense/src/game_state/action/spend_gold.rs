use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(crate) fn spend_gold(game_state: &mut GameState, gold: usize) {
    game_state.gold -= gold;
    game_state.metrics.total_gold_spent += gold;
    if gold > 0 {
        game_state.effect_events.push(GameEffectEvent::PlaySound(
            sound::EmitSoundParams::one_shot(
                sound::random_coin_sounds(),
                sound::SoundGroup::Ui,
                sound::VolumePreset::High,
                sound::SpatialMode::NonSpatial,
            ),
        ));
        game_state.handle_upgrade_trigger(UpgradeTriggerEvent::GoldSpent { amount: gold });
    }
}
