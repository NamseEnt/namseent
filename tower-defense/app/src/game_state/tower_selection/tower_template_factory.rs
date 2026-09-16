use crate::card::Rank;
use crate::card::Suit;
use crate::config::GameConfig;
use crate::game_state::tower::TowerKind;
use crate::game_state::tower::TowerTemplate;

pub fn create_tower_template(
    kind: TowerKind,
    suit: Suit,
    rank: Rank,
    config: &GameConfig,
) -> TowerTemplate {
    let mut template = TowerTemplate::new(kind, suit, rank);
    if let Some(stats) = config
        .towers
        .entries
        .iter()
        .find(|entry| entry.kind == kind.to_core_raw())
    {
        template.default_damage = crate::Damage::from_raw(stats.damage_raw);
        template.default_attack_range_radius = crate::WorldDistance::from_raw(stats.range_raw);
        template.shoot_interval = crate::SimTickSpan::from_millis_ceil(stats.cooldown_ms);
    }
    template
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_configured_tower_stats() {
        let config = GameConfig::default_config();
        let template = create_tower_template(TowerKind::OnePair, Suit::Hearts, Rank::Ace, &config);

        assert_eq!(template.default_damage, crate::Damage::from_integer(7));
        assert_eq!(
            template.default_attack_range_radius,
            crate::WorldDistance::from_tiles(5)
        );
        assert_eq!(
            template.shoot_interval,
            crate::SimTickSpan::from_millis_ceil(1_000)
        );
    }
}
