//! Namui-facing projection adapter for [`td_core::CoreState`].
//!
//! This module preserves presentation state and legacy persistence codecs.
//! Authoritative simulation remains in `td-core`.

use crate::SimTick;
use crate::card::Deck;
use crate::config::GameConfig;
use crate::game_state::GameRngState;
use crate::game_state::flow::GameFlow;
use crate::game_state::monster_spawn::MonsterSpawnState;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::game_state::upgrade::UpgradeState;
use crate::hand::{Hand, HandItem};
use crate::route::Route;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub use td_core::{CoreEvent, CoreEventQueue};

/// Headed compatibility projection used by Namui presentation and legacy
/// codecs. The authoritative state is always [`td_core::CoreState`]; callers
/// cross the boundary through the snapshot conversion methods below.
#[derive(Clone)]
/// Compatibility-only representation of the pre-core Namui save payload.
///
/// This type is deliberately not stored by [`GameState`]. It is decoded only
/// by the legacy migration path and may be used by test fixtures that exercise
/// that old format.
#[allow(dead_code)]
pub struct LegacyProjectionCodec {
    pub(crate) core_events: CoreEventQueue,
    pub(crate) progress: td_core::CoreProgress,
    pub(crate) player_commands: Vec<crate::game_state::RecordedPlayerCommand>,
    pub(crate) replay_checkpoints: Vec<crate::game_state::replay::ReplayCheckpoint>,
    pub(crate) sim_tick: td_core::SimTick,
    pub(crate) rng: GameRngState,
    pub(crate) route: Arc<Route>,
    pub(crate) config: Arc<GameConfig>,
    pub(crate) stage_modifiers: StageModifiers,
    pub(crate) upgrade_state: UpgradeState,
    pub(crate) hand: Hand<HandItem>,
    pub(crate) deck: Deck,
    pub(crate) items: Vec<crate::game_state::item::ItemWithId>,
    pub(crate) monster_spawn_state: MonsterSpawnState,
    pub(crate) in_flight_attacks: Vec<crate::game_state::attack::InFlightAttack>,
    pub(crate) user_status_effects: Vec<td_core::UserStatusEffect>,
    pub(crate) next_entity_id: crate::game_state::EntityIdAllocator,
    pub(crate) metrics: td_core::GameMetrics,
    pub(crate) flow: GameFlow,
    pub(crate) hp: crate::Health,
    pub(crate) shield: i64,
    pub(crate) monsters: Vec<crate::game_state::Monster>,
    pub(crate) towers: crate::game_state::PlacedTowers,
    pub(crate) pending_card_service_kind: Option<u8>,
    pub(crate) card_service_selection: Option<td_core::CardServiceSelectionState>,
}

// Kept only for the existing test/debug fixture surface. Production callers
// use explicit snapshots so this compatibility deref cannot become a new
// authoritative access path.
impl Deref for LegacyProjectionCodec {
    type Target = td_core::CoreProgress;

    fn deref(&self) -> &Self::Target {
        &self.progress
    }
}

impl DerefMut for LegacyProjectionCodec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.progress
    }
}

#[allow(dead_code)]
impl LegacyProjectionCodec {
    pub(crate) fn host_sim_tick(&self) -> SimTick {
        SimTick::from_ticks(self.sim_tick.ticks())
    }

    pub(crate) fn drain_events(&mut self) -> std::vec::Drain<'_, CoreEvent> {
        self.core_events.drain()
    }

    pub(crate) fn to_td_core_state(&self) -> td_core::CoreState {
        let mut parts = td_core::CoreSnapshotParts {
            progress: td_core::CoreProgress {
                player_command_sequence: self.progress.player_command_sequence,
                stage: self.progress.stage,
                gold: self.progress.gold,
                left_dice: self.progress.left_dice,
                rerolled_count: self.progress.rerolled_count,
                left_quest_board_refresh_chance: self.progress.left_quest_board_refresh_chance,
                item_used: self.progress.item_used,
            },
            sim_tick: self.sim_tick,
            rng: self.rng.clone(),
            route: self.route.to_core_state(),
            config: self.config.to_core_state(),
            stage_modifiers: self.stage_modifiers.to_core_state(),
            upgrades: self.upgrade_state.to_core_state(),
            hand: self.hand.to_core_state(),
            deck: self.deck.to_core_state(),
            items: self
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| item.to_core_state().with_id(index as u64 + 1))
                .collect(),
            monster_spawn: self.monster_spawn_state.to_core_state(),
            in_flight_attacks: self
                .in_flight_attacks
                .iter()
                .map(|attack| attack.to_core_state())
                .collect(),
            user_status_effects: self.user_status_effects.clone(),
            next_entity_id: self.next_entity_id,
            metrics: self.metrics.clone(),
            flow: self.flow.to_core_state(),
            hp_raw: self.hp.raw(),
            shield_raw: self.shield,
            monsters: self.monster_snapshots(),
            towers: self.tower_snapshots(),
            player_commands: self.player_commands.clone(),
            replay_checkpoints: self.replay_checkpoints.clone(),
            pending_card_service_kind: self.pending_card_service_kind,
            card_service_selection: self.card_service_selection.clone(),
        };
        let max_entity_id = parts
            .monster_spawn
            .monster_queue
            .iter()
            .map(|monster| monster.id)
            .chain(parts.monsters.iter().map(|monster| monster.id))
            .chain(parts.towers.iter().filter_map(|tower| tower.id))
            .chain(parts.in_flight_attacks.iter().map(|attack| attack.id))
            .max()
            .unwrap_or(0);
        if parts.next_entity_id.next_id() <= max_entity_id {
            parts.next_entity_id =
                td_core::EntityIdAllocator::from_next_id(max_entity_id.saturating_add(1));
        }
        td_core::CoreState::from_snapshot_parts(parts)
            .expect("headed projection must produce a valid core snapshot")
    }

    pub(crate) fn from_td_core_state(
        state: td_core::CoreState,
        presentation_source: Option<&Self>,
    ) -> Option<Self> {
        let snapshot = state.to_snapshot().into_parts();
        let config = Arc::new(crate::config::GameConfig::from_core_state(snapshot.config)?);
        let route = Arc::new(crate::route::Route::from_core_state(snapshot.route)?);
        let stage_modifiers = StageModifiers::from_core_state(snapshot.stage_modifiers)?;
        let upgrade_state = UpgradeState::from_core_state(snapshot.upgrades)?;
        let hand = Hand::from_core_state(
            snapshot.hand,
            presentation_source.map(|source| &source.hand),
        )?;
        let deck = Deck::from_core_state(snapshot.deck)?;
        let items = snapshot
            .items
            .entries()
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, item_state)| {
                let mut item = crate::game_state::item::ItemWithId::from_core_state(item_state)?;
                if let Some(source) = presentation_source.and_then(|source| source.items.get(index))
                {
                    item.id = source.id;
                }
                Some(item)
            })
            .collect::<Option<Vec<_>>>()?;
        let monster_spawn_state = MonsterSpawnState::from_core_state(
            snapshot.monster_spawn,
            presentation_source.map(|source| &source.monster_spawn_state),
        )?;
        let monsters = snapshot
            .monsters
            .into_iter()
            .map(crate::game_state::Monster::from_core_state)
            .collect::<Option<Vec<_>>>()?;
        let monsters = monsters
            .into_iter()
            .map(|mut monster| {
                if let Some(source) = presentation_source.and_then(|source| {
                    source
                        .monsters
                        .iter()
                        .find(|candidate| candidate.id() == monster.id())
                }) {
                    monster.restore_presentation_state(source.presentation_state());
                }
                monster
            })
            .collect();
        let mut towers = crate::game_state::PlacedTowers::default();
        for tower_state in snapshot.towers {
            let mut tower = crate::game_state::tower::Tower::from_core_state(
                tower_state,
                SimTick::from_ticks(snapshot.sim_tick.ticks()),
            )?;
            if let Some(source) = presentation_source.and_then(|source| {
                source
                    .towers
                    .iter()
                    .find(|candidate| candidate.id() == tower.id())
            }) {
                tower.restore_presentation_state(source.presentation_state());
            }
            if !towers.place_tower(tower) {
                return None;
            }
        }
        let in_flight_attacks = snapshot
            .in_flight_attacks
            .into_iter()
            .map(|attack| {
                let presentation = presentation_source.and_then(|source| {
                    source
                        .in_flight_attacks
                        .iter()
                        .find(|candidate| candidate.id == crate::AttackId::from_raw(attack.id))
                        .and_then(|candidate| match &candidate.kind {
                            crate::game_state::attack::InFlightAttackKind::Spatial(spatial) => {
                                Some((spatial.projectile_kind, spatial.trail, spatial.hit_effect))
                            }
                            _ => None,
                        })
                });
                let presentation = presentation.or(Some((
                    crate::game_state::projectile::ProjectileKind::Trash01,
                    crate::game_state::projectile::ProjectileTrail::None,
                    crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                )));
                crate::game_state::attack::InFlightAttack::from_core_state(attack, presentation)
            })
            .collect::<Option<Vec<_>>>()?;
        let flow = GameFlow::from_core_state(
            snapshot.flow,
            presentation_source.map(|source| &source.flow),
        )?;
        let mut restored = Self::new(
            SimTick::from_ticks(snapshot.sim_tick.ticks()),
            snapshot.rng,
            route,
            config,
            stage_modifiers,
            upgrade_state,
            hand,
            deck,
            items,
            monster_spawn_state,
            in_flight_attacks,
            snapshot.user_status_effects,
            snapshot.next_entity_id,
            snapshot.metrics,
            flow,
            snapshot.progress.stage,
            snapshot.progress.gold,
            crate::Health::from_raw(snapshot.hp_raw),
            crate::Shield::from_raw(snapshot.shield_raw),
            snapshot.progress.left_dice,
            snapshot.progress.rerolled_count,
            snapshot.progress.left_quest_board_refresh_chance,
            snapshot.progress.item_used,
            monsters,
            towers,
            snapshot.progress.player_command_sequence,
            snapshot.player_commands,
            snapshot.replay_checkpoints,
        );
        restored.pending_card_service_kind = snapshot.pending_card_service_kind;
        restored.card_service_selection = snapshot.card_service_selection;
        restored.refresh_tower_upgrade_damage_cache();
        Some(restored)
    }

    pub(crate) fn monster_snapshots(&self) -> Vec<td_core::MonsterState> {
        self.monsters
            .iter()
            .map(crate::game_state::Monster::to_core_state)
            .collect()
    }

    pub(crate) fn restore_monster_snapshots(
        &mut self,
        snapshots: Vec<td_core::MonsterState>,
    ) -> bool {
        let presentations = self
            .monsters
            .iter()
            .map(|monster| (monster.id(), monster.presentation_state()))
            .collect::<Vec<_>>();
        let Some(monsters) = snapshots
            .into_iter()
            .map(crate::game_state::Monster::from_core_state)
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let mut monster_ids = Vec::with_capacity(monsters.len());
        for monster in &monsters {
            if monster_ids.contains(&monster.id()) {
                return false;
            }
            monster_ids.push(monster.id());
        }
        self.monsters = monsters
            .into_iter()
            .map(|mut monster| {
                if let Some((_, presentation)) =
                    presentations.iter().find(|(id, _)| *id == monster.id())
                {
                    monster.restore_presentation_state(presentation.clone());
                }
                monster
            })
            .collect();
        true
    }

    pub(crate) fn tower_snapshots(&self) -> Vec<td_core::TowerState> {
        self.towers
            .iter()
            .map(|tower| {
                let mut snapshot = tower.to_core_state();
                let raw_upgrades = self.upgrade_state.to_core_state();
                snapshot.damage_multiplier_raw = td_core::RATIO_SCALE
                    .saturating_add(raw_upgrades.tower_damage_bonus_raw(&snapshot));
                snapshot
            })
            .collect()
    }

    pub(crate) fn entity_snapshots(&self) -> td_core::EntitySnapshots {
        td_core::EntitySnapshots {
            monsters: self.monster_snapshots(),
            towers: self.tower_snapshots(),
        }
    }

    pub(crate) fn monsters_for_legacy_serialization(&self) -> Vec<crate::game_state::Monster> {
        let presentations = self
            .monsters
            .iter()
            .map(|monster| (monster.id(), monster.presentation_state()))
            .collect::<Vec<_>>();
        self.monster_snapshots()
            .into_iter()
            .map(crate::game_state::Monster::from_core_state)
            .map(|monster| {
                let mut monster = monster.expect("valid monster snapshot");
                if let Some((_, presentation)) =
                    presentations.iter().find(|(id, _)| *id == monster.id())
                {
                    monster.restore_presentation_state(presentation.clone());
                }
                monster
            })
            .collect()
    }

    pub(crate) fn towers_for_legacy_serialization(&self) -> crate::game_state::PlacedTowers {
        let presentations = self
            .towers
            .iter()
            .map(|tower| (tower.id(), tower.presentation_state()))
            .collect::<Vec<_>>();
        let mut towers = crate::game_state::PlacedTowers::default();
        for snapshot in self.tower_snapshots() {
            let mut tower =
                crate::game_state::tower::Tower::from_core_state(snapshot, self.host_sim_tick())
                    .expect("valid tower snapshot");
            if let Some((_, presentation)) = presentations.iter().find(|(id, _)| *id == tower.id())
            {
                tower.restore_presentation_state(presentation.clone());
            }
            assert!(towers.place_tower(tower));
        }
        towers
    }

    pub(crate) fn normalize_decoded_entity_snapshots(&mut self) -> bool {
        let snapshots = self.entity_snapshots();
        self.restore_monster_snapshots(snapshots.monsters)
            && self.restore_tower_snapshots(snapshots.towers, self.host_sim_tick())
    }

    pub(crate) fn normalize_decoded_attack_snapshots(&mut self) -> bool {
        for attack in &mut self.in_flight_attacks {
            let presentation = match &attack.kind {
                crate::game_state::attack::InFlightAttackKind::Spatial(spatial) => {
                    Some((spatial.projectile_kind, spatial.trail, spatial.hit_effect))
                }
                _ => None,
            };
            let raw = attack.to_core_state();
            let Some(normalized) =
                crate::game_state::attack::InFlightAttack::from_core_state(raw, presentation)
            else {
                return false;
            };
            *attack = normalized;
        }
        true
    }

    pub(crate) fn attacks_for_legacy_serialization(
        &self,
    ) -> Vec<crate::game_state::attack::InFlightAttack> {
        let mut attacks = self.in_flight_attacks.clone();
        for attack in &mut attacks {
            let presentation = match &attack.kind {
                crate::game_state::attack::InFlightAttackKind::Spatial(spatial) => {
                    Some((spatial.projectile_kind, spatial.trail, spatial.hit_effect))
                }
                _ => None,
            };
            let raw = attack.to_core_state();
            *attack = crate::game_state::attack::InFlightAttack::from_core_state(raw, presentation)
                .expect("valid in-flight attack state");
        }
        attacks
    }

    pub(crate) fn config_for_legacy_serialization(&self) -> crate::config::GameConfig {
        crate::config::GameConfig::from_core_state(self.config.to_core_state())
            .expect("valid game config state")
    }

    fn refresh_tower_upgrade_damage_cache(&mut self) {
        let upgrade_revision = self.upgrade_state.revision;
        let raw_upgrades = self.upgrade_state.to_core_state();
        for tower in self.towers.iter_mut() {
            let snapshot = tower.to_core_state();
            tower.refresh_cached_upgrade_damage_raw(
                upgrade_revision,
                raw_upgrades.tower_upgrade_bonus_raw(&snapshot),
            );
        }
    }

    pub(crate) fn restore_tower_snapshots(
        &mut self,
        snapshots: Vec<td_core::TowerState>,
        sim_tick: SimTick,
    ) -> bool {
        let presentations = self
            .towers
            .iter()
            .map(|tower| (tower.id(), tower.presentation_state()))
            .collect::<Vec<_>>();
        let Some(towers) = snapshots
            .into_iter()
            .map(|snapshot| crate::game_state::tower::Tower::from_core_state(snapshot, sim_tick))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let mut placed_towers = crate::game_state::PlacedTowers::default();
        let mut tower_ids = Vec::with_capacity(towers.len());
        for mut tower in towers {
            if tower_ids.contains(&tower.id()) {
                return false;
            }
            tower_ids.push(tower.id());
            if let Some((_, presentation)) = presentations.iter().find(|(id, _)| *id == tower.id())
            {
                tower.restore_presentation_state(presentation.clone());
            }
            placed_towers.place_tower(tower);
        }
        self.towers = placed_towers;
        true
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        sim_tick: SimTick,
        rng: GameRngState,
        route: Arc<Route>,
        config: Arc<GameConfig>,
        stage_modifiers: StageModifiers,
        upgrade_state: UpgradeState,
        hand: Hand<HandItem>,
        deck: Deck,
        items: Vec<crate::game_state::item::ItemWithId>,
        monster_spawn_state: MonsterSpawnState,
        in_flight_attacks: Vec<crate::game_state::attack::InFlightAttack>,
        user_status_effects: Vec<td_core::UserStatusEffect>,
        next_entity_id: crate::game_state::EntityIdAllocator,
        metrics: td_core::GameMetrics,
        flow: GameFlow,
        stage: usize,
        gold: usize,
        hp: crate::Health,
        shield: crate::Shield,
        left_dice: usize,
        rerolled_count: usize,
        left_quest_board_refresh_chance: usize,
        item_used: bool,
        monsters: Vec<crate::game_state::Monster>,
        towers: crate::game_state::PlacedTowers,
        player_command_sequence: u64,
        player_commands: Vec<crate::game_state::RecordedPlayerCommand>,
        replay_checkpoints: Vec<crate::game_state::replay::ReplayCheckpoint>,
    ) -> Self {
        Self {
            core_events: CoreEventQueue::default(),
            progress: td_core::CoreProgress {
                player_command_sequence,
                stage,
                gold,
                left_dice,
                rerolled_count,
                left_quest_board_refresh_chance,
                item_used,
            },
            player_commands,
            replay_checkpoints,
            sim_tick: td_core::SimTick::from_ticks(sim_tick.ticks()),
            rng,
            route,
            config,
            stage_modifiers,
            upgrade_state,
            hand,
            deck,
            items,
            monster_spawn_state,
            in_flight_attacks,
            user_status_effects,
            next_entity_id,
            metrics,
            flow,
            hp,
            shield: shield.raw(),
            monsters,
            towers,
            pending_card_service_kind: None,
            card_service_selection: None,
        }
    }
}
