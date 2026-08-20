use super::{GameState, RecordedPlayerCommand};
use crate::PresentationInstant;
use crate::config::GameConfig;
use crate::deterministic_rng;
use namui::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub(crate) const REPLAY_SCHEMA_VERSION: u32 = 1;
pub(crate) const RNG_ALGORITHM_VERSION: u32 = deterministic_rng::RNG_ALGORITHM_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, SerdeSerialize, SerdeDeserialize)]
pub(crate) struct Replay {
    pub(crate) schema_version: u32,
    pub(crate) rng_algorithm_version: u32,
    pub(crate) config_version: u32,
    pub(crate) config_digest: String,
    pub(crate) seed: u64,
    pub(crate) commands: Vec<RecordedPlayerCommand>,
    #[serde(default)]
    pub(crate) checkpoints: Vec<ReplayCheckpoint>,
}

#[derive(Clone, Debug, PartialEq, Eq, SerdeSerialize, SerdeDeserialize, State)]
pub(crate) struct ReplayCheckpoint {
    pub(crate) sequence: u64,
    pub(crate) completed_sim_tick: u64,
    pub(crate) state_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReplayDivergence {
    pub(crate) sequence: u64,
    pub(crate) completed_sim_tick: u64,
    pub(crate) expected_hash: String,
    pub(crate) actual_hash: String,
}

#[derive(Debug)]
pub(crate) enum ReplayError {
    UnsupportedSchemaVersion(u32),
    UnsupportedRngAlgorithmVersion(u32),
    UnsupportedConfigVersion(u32),
    ConfigDigestMismatch {
        expected: String,
        actual: String,
    },
    NonMonotonicCommand {
        expected_sequence: u64,
        actual_sequence: u64,
    },
    CommandTickRewound {
        sequence: u64,
        current_tick: u64,
        command_tick: u64,
    },
    CommandRejected {
        sequence: u64,
        error: crate::game_state::player_command::PlayerCommandError,
    },
}

#[derive(Debug)]
pub(crate) struct ReplayRun {
    pub(crate) checkpoints: Vec<ReplayCheckpoint>,
    pub(crate) divergence: Option<ReplayDivergence>,
}

impl Replay {
    pub(crate) fn new(
        config: &GameConfig,
        seed: u64,
        commands: Vec<RecordedPlayerCommand>,
    ) -> Self {
        Self {
            schema_version: REPLAY_SCHEMA_VERSION,
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
            config_version: crate::config::GAME_CONFIG_VERSION,
            config_digest: config_digest(config),
            seed,
            commands,
            checkpoints: Vec::new(),
        }
    }

    pub(crate) fn from_game_state(game_state: &GameState) -> Self {
        Self {
            schema_version: REPLAY_SCHEMA_VERSION,
            rng_algorithm_version: RNG_ALGORITHM_VERSION,
            config_version: crate::config::GAME_CONFIG_VERSION,
            config_digest: config_digest(&game_state.config),
            seed: game_state.rng.seed,
            commands: game_state.player_commands.clone(),
            checkpoints: game_state.replay_checkpoints.clone(),
        }
    }

    pub(crate) fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub(crate) fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

pub(crate) fn run_replay(
    replay: &Replay,
    config: Arc<GameConfig>,
) -> Result<ReplayRun, ReplayError> {
    if replay.schema_version != REPLAY_SCHEMA_VERSION {
        return Err(ReplayError::UnsupportedSchemaVersion(replay.schema_version));
    }
    if replay.rng_algorithm_version != RNG_ALGORITHM_VERSION {
        return Err(ReplayError::UnsupportedRngAlgorithmVersion(
            replay.rng_algorithm_version,
        ));
    }
    if replay.config_version != crate::config::GAME_CONFIG_VERSION {
        return Err(ReplayError::UnsupportedConfigVersion(replay.config_version));
    }
    let actual_config_digest = config_digest(&config);
    if replay.config_digest != actual_config_digest {
        return Err(ReplayError::ConfigDigestMismatch {
            expected: replay.config_digest.clone(),
            actual: actual_config_digest,
        });
    }

    let mut game_state = super::create_game_state_with_config(config, replay.seed);
    game_state.headless = true;
    let mut actual_checkpoints = Vec::with_capacity(replay.commands.len());

    for (expected_sequence, recorded_command) in replay.commands.iter().enumerate() {
        let expected_sequence = expected_sequence as u64;
        if recorded_command.sequence != expected_sequence {
            return Err(ReplayError::NonMonotonicCommand {
                expected_sequence,
                actual_sequence: recorded_command.sequence,
            });
        }

        let current_tick = game_state.sim_tick().ticks();
        if recorded_command.completed_sim_tick < current_tick {
            return Err(ReplayError::CommandTickRewound {
                sequence: recorded_command.sequence,
                current_tick,
                command_tick: recorded_command.completed_sim_tick,
            });
        }
        while game_state.sim_tick().ticks() < recorded_command.completed_sim_tick {
            super::tick::step_simulation(&mut game_state, PresentationInstant::zero());
        }

        game_state
            .apply_player_command(recorded_command.command.clone())
            .map_err(|error| ReplayError::CommandRejected {
                sequence: recorded_command.sequence,
                error,
            })?;
        let checkpoint = game_state
            .replay_checkpoints
            .last()
            .cloned()
            .expect("accepted player command must create a replay checkpoint");
        actual_checkpoints.push(checkpoint);
    }

    let divergence = (!replay.checkpoints.is_empty())
        .then(|| first_divergence(&replay.checkpoints, &actual_checkpoints))
        .flatten();
    Ok(ReplayRun {
        checkpoints: actual_checkpoints,
        divergence,
    })
}

pub(crate) fn config_digest(config: &GameConfig) -> String {
    let bytes = toml::to_string(config)
        .expect("GameConfig serialization must succeed")
        .into_bytes();
    sha256_hex(&bytes)
}

const AUTHORITATIVE_HASH_VERSION: u32 = 2;

pub(crate) fn authoritative_hash(game_state: &GameState) -> String {
    let mut writer = CanonicalWriter::default();
    writer.u64(AUTHORITATIVE_HASH_VERSION as u64);
    writer.string(&config_digest(&game_state.config));
    writer.u64(game_state.sim_tick().ticks());
    writer.u64(game_state.stage as u64);
    writer.u64(game_state.gold as u64);
    writer.i64(game_state.hp.raw());
    writer.i64(game_state.shield.raw());
    writer.u64(game_state.left_dice as u64);
    writer.u64(game_state.rerolled_count as u64);
    writer.u64(game_state.left_quest_board_refresh_chance as u64);
    writer.bool(game_state.item_used);
    hash_flow(&mut writer, &game_state.flow);
    hash_route(&mut writer, &game_state.route);
    hash_hand(&mut writer, &game_state.hand);
    hash_deck(&mut writer, &game_state.deck);
    hash_upgrade_state(&mut writer, &game_state.upgrade_state);
    hash_stage_modifiers(&mut writer, &game_state.stage_modifiers);
    hash_items(&mut writer, &game_state.items);
    hash_monsters(&mut writer, &game_state.monsters);
    hash_monster_spawn_state(&mut writer, &game_state.monster_spawn_state);
    hash_towers(&mut writer, &game_state.towers);
    writer.u64(game_state.in_flight_attacks.len() as u64);
    for attack in &game_state.in_flight_attacks {
        hash_attack(&mut writer, attack);
    }
    writer.u64(game_state.user_status_effects.len() as u64);
    for status_effect in &game_state.user_status_effects {
        writer.debug(status_effect);
    }
    writer.u64(game_state.next_entity_id.next_id());
    hash_metrics(&mut writer, &game_state.metrics);
    hash_rng(&mut writer, &game_state.rng);
    sha256_hex(&writer.bytes)
}

fn hash_flow(writer: &mut CanonicalWriter, flow: &crate::game_state::flow::GameFlow) {
    use crate::game_state::flow::GameFlow;

    match flow {
        GameFlow::Initializing => writer.u64(0),
        GameFlow::Shopping(flow) => {
            writer.u64(1);
            writer.u64(flow.shop.slots.len() as u64);
            for slot in &flow.shop.slots {
                writer.bool(slot.purchased);
                hash_shop_slot(writer, &slot.slot);
            }
        }
        GameFlow::SelectingTower(_) => writer.u64(2),
        GameFlow::PlacingTower => writer.u64(3),
        GameFlow::Defense(flow) => {
            writer.u64(4);
            writer.i64(flow.stage_progress.start_total_hp.raw());
            writer.i64(flow.stage_progress.processed_hp.raw());
            writer.bool(flow.took_damage);
        }
        GameFlow::TreasureSelection(flow) => {
            writer.u64(5);
            writer.u64(flow.options.len() as u64);
            for option in &flow.options {
                writer.debug(option);
            }
            match flow.pending_selection {
                Some(selection) => {
                    writer.bool(true);
                    writer.u64(selection as u64);
                }
                None => writer.bool(false),
            }
        }
        GameFlow::Result { clear_rate } => {
            writer.u64(6);
            writer.debug(clear_rate);
        }
    }
}

fn hash_shop_slot(writer: &mut CanonicalWriter, slot: &crate::shop::ShopSlot) {
    match slot {
        crate::shop::ShopSlot::Item { item, cost } => {
            writer.u64(0);
            writer.debug(item);
            writer.u64(*cost as u64);
        }
        crate::shop::ShopSlot::Upgrade { upgrade, cost } => {
            writer.u64(1);
            writer.debug(upgrade);
            writer.u64(*cost as u64);
        }
        crate::shop::ShopSlot::CardService { card_service, cost } => {
            writer.u64(2);
            writer.debug(card_service);
            writer.u64(*cost as u64);
        }
    }
}

fn hash_route(writer: &mut CanonicalWriter, route: &crate::route::Route) {
    writer.u64(route.iter_coords().len() as u64);
    for coord in route.iter_coords() {
        writer.u64(coord.x as u64);
        writer.u64(coord.y as u64);
    }
}

fn hash_hand(writer: &mut CanonicalWriter, hand: &crate::hand::Hand<crate::hand::HandItem>) {
    let slot_ids = hand.active_slot_ids();
    writer.u64(slot_ids.len() as u64);
    for slot_id in slot_ids {
        let item = hand
            .get_item(slot_id)
            .expect("active hand slot must have an item");
        match item {
            crate::hand::HandItem::Card(card) => {
                writer.u64(0);
                hash_card(writer, card);
            }
            crate::hand::HandItem::Tower(template) => {
                writer.u64(1);
                writer.debug(template);
            }
        }
    }
}

fn hash_deck(writer: &mut CanonicalWriter, deck: &crate::card::Deck) {
    writer.u64(deck.next_card_id() as u64);
    hash_cards(writer, deck.all_cards());
    hash_cards(writer, deck.draw_pile());
    hash_cards(writer, deck.discard_pile());
}

fn hash_cards(writer: &mut CanonicalWriter, cards: &[crate::card::Card]) {
    writer.u64(cards.len() as u64);
    for card in cards {
        hash_card(writer, card);
    }
}

fn hash_card(writer: &mut CanonicalWriter, card: &crate::card::Card) {
    writer.u64(card.id.raw() as u64);
    writer.debug(&card.suit);
    writer.debug(&card.rank);
    writer.i64(card.effects.polish_pct.raw());
    writer.debug(&card.effects.engraving);
}

fn hash_upgrade_state(
    writer: &mut CanonicalWriter,
    state: &crate::game_state::upgrade::UpgradeState,
) {
    writer.u64(state.upgrades.len() as u64);
    for upgrade in &state.upgrades {
        writer.debug(&upgrade.upgrade);
    }
}

fn hash_stage_modifiers(
    writer: &mut CanonicalWriter,
    modifiers: &crate::game_state::stage_modifiers::StageModifiers,
) {
    writer.string(&modifiers.canonical_debug());
}

fn hash_items(writer: &mut CanonicalWriter, items: &[crate::game_state::item::ItemWithId]) {
    writer.u64(items.len() as u64);
    for item in items {
        writer.debug(&item.item);
    }
}

fn hash_monsters(writer: &mut CanonicalWriter, monsters: &[crate::game_state::Monster]) {
    let mut monsters = monsters.iter().collect::<Vec<_>>();
    monsters.sort_by_key(|monster| monster.id().raw());
    writer.u64(monsters.len() as u64);
    for monster in monsters {
        hash_monster(writer, monster);
    }
}

fn hash_monster(writer: &mut CanonicalWriter, monster: &crate::game_state::Monster) {
    writer.u64(monster.id().raw());
    writer.debug(&monster.kind);
    writer.u64(monster.move_on_route.route_index() as u64);
    writer.i64(monster.move_on_route.route_progress().raw());
    writer.i64(monster.move_on_route.world_xy().x);
    writer.i64(monster.move_on_route.world_xy().y);
    writer.i64(monster.move_on_route.velocity().raw());
    writer.i64(monster.move_on_route.movement_remainder());
    writer.u64(monster.move_on_route.motion_revision());
    writer.i64(monster.hp.raw());
    writer.i64(monster.max_hp.raw());
    writer.i64(monster.damage.raw());
    writer.u64(monster.reward as u64);
    writer.bool(monster.stage_progress_counted);
    writer.debug(&monster.skills);
    writer.debug(&monster.status_effects);
}

fn hash_monster_spawn_state(
    writer: &mut CanonicalWriter,
    spawn_state: &crate::game_state::monster_spawn::MonsterSpawnState,
) {
    writer.u64(spawn_state.monster_queue.len() as u64);
    for monster in &spawn_state.monster_queue {
        hash_monster(writer, monster);
    }
    match spawn_state.next_spawn_tick {
        Some(tick) => {
            writer.bool(true);
            writer.u64(tick.ticks());
        }
        None => writer.bool(false),
    }
    writer.u64(spawn_state.spawn_interval.ticks());
}

fn hash_towers(writer: &mut CanonicalWriter, towers: &crate::game_state::PlacedTowers) {
    let mut towers = towers.iter().collect::<Vec<_>>();
    towers.sort_by_key(|tower| tower.id().raw());
    writer.u64(towers.len() as u64);
    for tower in towers {
        writer.u64(tower.id().raw());
        writer.u64(tower.left_top.x as u64);
        writer.u64(tower.left_top.y as u64);
        writer.debug(&tower.template);
        writer.u64(tower.cooldown_ticks());
        writer.i64(tower.cached_upgrade_damage().raw());
        writer.debug(&tower.status_effects);
        writer.debug(&tower.skills);
    }
}

fn hash_attack(writer: &mut CanonicalWriter, attack: &crate::game_state::attack::InFlightAttack) {
    writer.u64(attack.id.raw());
    writer.i64(attack.damage.raw());
    match attack.source_tower {
        Some(source) => {
            writer.bool(true);
            writer.debug(&source);
        }
        None => writer.bool(false),
    }
    writer.debug(&attack.on_hit_splashes);
    match &attack.kind {
        crate::game_state::attack::InFlightAttackKind::Spatial(spatial) => {
            writer.u64(0);
            writer.i64(spatial.xy.x);
            writer.i64(spatial.xy.y);
            writer.i64(spatial.velocity.x);
            writer.i64(spatial.velocity.y);
            writer.u64(spatial.target_indicator.id().raw());
            writer.debug(&spatial.projectile_kind);
            writer.debug(&spatial.trail);
            writer.debug(&spatial.behavior);
            writer.debug(&spatial.hit_effect);
            writer.i64(spatial.movement_remainder);
            writer.u64(spatial.stable_key);
        }
        crate::game_state::attack::InFlightAttackKind::Timed(timed) => {
            writer.u64(1);
            writer.u64(timed.target_monster_id.raw());
            writer.u64(timed.execute_at.ticks());
        }
        crate::game_state::attack::InFlightAttackKind::Laser(laser) => {
            writer.u64(2);
            writer.i64(laser.start_xy.x);
            writer.i64(laser.start_xy.y);
            writer.i64(laser.end_xy.x);
            writer.i64(laser.end_xy.y);
            writer.u64(laser.created_at.ticks());
            writer.u64(laser.target_monster_id.raw());
        }
    }
}

fn hash_metrics(writer: &mut CanonicalWriter, metrics: &crate::game_state::GameMetrics) {
    writer.u64(metrics.total_gold_earned as u64);
    writer.i64(metrics.total_escaped_hp.raw());
    writer.i64(metrics.total_player_damage.raw());
    writer.u64(metrics.stage_damage.len() as u64);
    for (stage, damage) in &metrics.stage_damage {
        writer.u64(*stage as u64);
        writer.i64(damage.raw());
    }
    writer.u64(metrics.total_gold_spent as u64);
    writer.u64(metrics.current_consecutive_perfect_clears as u64);
    writer.u64(metrics.max_consecutive_perfect_clears as u64);
    writer.u64(metrics.total_rerolled_count as u64);
    let mut tower_damage_stats = metrics.tower_damage_stats.iter().collect::<Vec<_>>();
    tower_damage_stats.sort_by_key(|stats| stats.tower_id.raw());
    writer.u64(tower_damage_stats.len() as u64);
    for stats in tower_damage_stats {
        writer.u64(stats.tower_id.raw());
        writer.debug(&stats.tower_kind);
        writer.debug(&stats.rank);
        writer.debug(&stats.suit);
        writer.i64(stats.total_damage.raw());
    }
}

fn hash_rng(writer: &mut CanonicalWriter, rng: &crate::game_state::rng::GameRngState) {
    writer.u64(rng.seed);
    writer.u64(rng.shop.generation_sequence);
    writer.u64(rng.shop.config.category_bag_size as u64);
    hash_u32_values(writer, &rng.shop.config.category_weights);
    writer.u64(rng.shop.config.rarity_bag_size as u64);
    hash_u32_values(writer, &rng.shop.config.item_rarity_weights);
    hash_u32_values(writer, &rng.shop.config.card_service_rarity_weights);
    hash_u32_values(writer, &rng.shop.config.upgrade_rarity_weights);
    hash_bag(writer, &rng.shop.category_bag);
    for bag in &rng.shop.rarity_bags {
        hash_bag(writer, bag);
    }
    for bag in &rng.shop.content_bags {
        writer.u64(bag.entries.len() as u64);
        for entry in &bag.entries {
            writer.string(entry);
        }
        writer.u64(bag.cursor as u64);
        writer.u64(bag.cycle);
    }
    writer.u64(rng.domain_sequences.len() as u64);
    for (domain, sequence) in &rng.domain_sequences {
        writer.u64(*domain);
        writer.u64(*sequence);
    }
}

fn hash_u32_values(writer: &mut CanonicalWriter, values: &[u32]) {
    writer.u64(values.len() as u64);
    for value in values {
        writer.u64(u64::from(*value));
    }
}

fn hash_bag(writer: &mut CanonicalWriter, bag: &crate::game_state::rng::BagState) {
    writer.u64(bag.entries.len() as u64);
    for entry in &bag.entries {
        writer.u64(u64::from(*entry));
    }
    writer.u64(bag.cursor as u64);
    writer.u64(bag.cycle);
}

pub(crate) fn first_divergence(
    expected: &[ReplayCheckpoint],
    actual: &[ReplayCheckpoint],
) -> Option<ReplayDivergence> {
    expected
        .iter()
        .zip(actual)
        .find_map(|(expected, actual)| {
            (expected.sequence != actual.sequence
                || expected.completed_sim_tick != actual.completed_sim_tick
                || expected.state_hash != actual.state_hash)
                .then(|| ReplayDivergence {
                    sequence: expected.sequence,
                    completed_sim_tick: expected.completed_sim_tick,
                    expected_hash: expected.state_hash.clone(),
                    actual_hash: actual.state_hash.clone(),
                })
        })
        .or_else(|| {
            (expected.len() != actual.len()).then(|| {
                match (expected.get(actual.len()), actual.get(expected.len())) {
                    (Some(expected), None) => ReplayDivergence {
                        sequence: expected.sequence,
                        completed_sim_tick: expected.completed_sim_tick,
                        expected_hash: expected.state_hash.clone(),
                        actual_hash: String::new(),
                    },
                    (None, Some(actual)) => ReplayDivergence {
                        sequence: actual.sequence,
                        completed_sim_tick: actual.completed_sim_tick,
                        expected_hash: String::new(),
                        actual_hash: actual.state_hash.clone(),
                    },
                    _ => unreachable!("checkpoint lengths differ at exactly one boundary"),
                }
            })
        })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Default)]
struct CanonicalWriter {
    bytes: Vec<u8>,
}

impl CanonicalWriter {
    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn bool(&mut self, value: bool) {
        self.bytes.push(u8::from(value));
    }

    fn string(&mut self, value: &str) {
        self.u64(value.len() as u64);
        self.bytes.extend_from_slice(value.as_bytes());
    }

    fn debug<T: std::fmt::Debug>(&mut self, value: &T) {
        self.string(&format!("{value:?}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::PlayerCommand;

    #[test]
    fn replay_json_round_trips_versioned_metadata_and_commands() {
        let config = GameConfig::default_config();
        let replay = Replay::new(
            &config,
            42,
            vec![RecordedPlayerCommand {
                sequence: 0,
                completed_sim_tick: 7,
                command: PlayerCommand::StartDefense,
            }],
        );

        let decoded = Replay::from_json(&replay.to_json().unwrap()).unwrap();

        assert_eq!(decoded, replay);
        assert_eq!(decoded.schema_version, REPLAY_SCHEMA_VERSION);
        assert_eq!(decoded.rng_algorithm_version, RNG_ALGORITHM_VERSION);
        assert_eq!(decoded.config_version, crate::config::GAME_CONFIG_VERSION);
        assert_eq!(decoded.config_digest, config_digest(&config));
    }

    #[test]
    fn first_divergence_reports_the_first_mismatching_command() {
        let expected = vec![
            ReplayCheckpoint {
                sequence: 0,
                completed_sim_tick: 3,
                state_hash: "a".to_string(),
            },
            ReplayCheckpoint {
                sequence: 1,
                completed_sim_tick: 8,
                state_hash: "b".to_string(),
            },
        ];
        let actual = vec![
            expected[0].clone(),
            ReplayCheckpoint {
                state_hash: "c".to_string(),
                ..expected[1].clone()
            },
        ];

        let divergence = first_divergence(&expected, &actual).unwrap();

        assert_eq!(divergence.sequence, 1);
        assert_eq!(divergence.completed_sim_tick, 8);
        assert_eq!(divergence.expected_hash, "b");
        assert_eq!(divergence.actual_hash, "c");
    }

    #[test]
    fn replay_runner_reproduces_checkpoints_and_reports_mutated_checkpoint() {
        let mut source = crate::game_state::create_game_state_with_seed(42);
        source.headless = true;
        source
            .apply_player_command(PlayerCommand::StartSelectingTower)
            .unwrap();
        let replay = Replay::from_json(&source.export_replay_json().unwrap()).unwrap();
        let config = std::sync::Arc::new(GameConfig::default_config());

        let run = run_replay(&replay, config.clone()).unwrap();

        assert_eq!(run.checkpoints, source.replay_checkpoints);
        assert!(run.divergence.is_none());

        let mut mutated = replay;
        mutated.checkpoints[0].state_hash = "different".to_string();
        let run = run_replay(&mutated, config).unwrap();
        let divergence = run.divergence.unwrap();
        assert_eq!(divergence.sequence, 0);
        assert_eq!(divergence.completed_sim_tick, 0);
        assert_eq!(divergence.expected_hash, "different");
        assert_eq!(
            divergence.actual_hash,
            source.replay_checkpoints[0].state_hash
        );
    }

    #[test]
    fn command_only_replay_runs_without_checkpoint_divergence() {
        let config = GameConfig::default_config();
        let replay = Replay::new(
            &config,
            42,
            vec![RecordedPlayerCommand {
                sequence: 0,
                completed_sim_tick: 0,
                command: PlayerCommand::StartSelectingTower,
            }],
        );

        let run = run_replay(&replay, std::sync::Arc::new(config)).unwrap();

        assert_eq!(run.checkpoints.len(), 1);
        assert!(run.divergence.is_none());
    }

    #[test]
    fn tracking_revisions_do_not_change_authoritative_hash() {
        let mut deck_revision_changed = crate::game_state::create_game_state_with_seed(42);
        let mut upgrade_revision_changed = crate::game_state::create_game_state_with_seed(42);
        let baseline = crate::game_state::create_game_state_with_seed(42);

        deck_revision_changed.deck.discard(std::iter::empty());
        upgrade_revision_changed.upgrade_state.revision = upgrade_revision_changed
            .upgrade_state
            .revision
            .wrapping_add(1);

        assert_eq!(
            authoritative_hash(&baseline),
            authoritative_hash(&deck_revision_changed)
        );
        assert_eq!(
            authoritative_hash(&baseline),
            authoritative_hash(&upgrade_revision_changed)
        );
    }

    #[test]
    fn first_divergence_reports_the_extra_checkpoint_side() {
        let expected = vec![ReplayCheckpoint {
            sequence: 0,
            completed_sim_tick: 2,
            state_hash: "a".to_string(),
        }];
        let actual = vec![
            expected[0].clone(),
            ReplayCheckpoint {
                sequence: 1,
                completed_sim_tick: 5,
                state_hash: "b".to_string(),
            },
        ];

        let divergence = first_divergence(&expected, &actual).unwrap();

        assert_eq!(divergence.sequence, 1);
        assert_eq!(divergence.completed_sim_tick, 5);
        assert!(divergence.expected_hash.is_empty());
        assert_eq!(divergence.actual_hash, "b");
    }
}
