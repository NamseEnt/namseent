use crate::game_state::{action::upgrade_trigger::UpgradeTriggerEvent, *};

pub(super) fn deduct_from_balance(game_state: &mut GameState, amount: usize) {
    game_state.gold -= amount;
    game_state.metrics.total_gold_spent += amount;
}

pub(super) fn play_spend_sound(game_state: &mut GameState, amount: usize) {
    if amount == 0 {
        return;
    }
    game_state.effect_events.push(GameEffectEvent::PlaySound(
        sound::EmitSoundParams::one_shot(
            sound::random_coin_sounds(),
            sound::SoundGroup::Ui,
            sound::VolumePreset::High,
            sound::SpatialMode::NonSpatial,
        ),
    ));
}

pub(super) fn trigger_upgrades(game_state: &mut GameState, amount: usize) {
    if amount == 0 {
        return;
    }
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::GoldSpent { amount });
}
