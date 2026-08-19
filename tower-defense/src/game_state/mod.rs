pub(crate) mod action;
pub mod attack;
pub mod background;
mod base;
mod camera;
pub mod can_place_tower;
pub mod cursor_preview;
#[cfg(feature = "debug-tools")]
mod debug_tools;
pub mod difficulty;
pub mod effect;
pub mod effect_event;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
#[allow(unused)]
mod map_decoration_atlas;
pub mod modal;
pub mod monster;
pub(crate) mod monster_spawn;
mod placed_towers;
pub(crate) mod rng;
pub(crate) use action::GameStateAction;
pub mod card_notification;
pub mod card_service;
pub(crate) mod discovery;
pub(crate) mod play_history;
pub mod poker_action;
pub mod projectile;
mod render;
pub(crate) mod shop_purchase;
pub mod stage_modifiers;
mod status_effect_particle_generator;
pub(crate) mod tick;
pub mod tower;
mod tower_info_popup;
mod ui_state;
pub mod upgrade;
mod user_status_effect;

use crate::card::{Deck, Rank, Suit};
use crate::combat_number::RATIO_SCALE;
use crate::config::GameConfig;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::hand::{Hand, HandItem};
use crate::route::*;
use crate::*;
use background::Background;
pub use background::generate_backgrounds;
pub use base::*;
pub(crate) use camera::Camera;
use cursor_preview::CursorPreview;
pub use effect_event::*;
use fast_forward::FastForwardMultiplier;
use flow::GameFlow;
use item::{LumpSugarItem, RubberConeItem};
pub use modal::UserModal;
pub use monster::*;
use monster_spawn::*;
use namui::*;
use placed_towers::PlacedTowers;
use play_history::PlayHistory;
use projectile::*;
use rand::Rng;
pub use render::*;
use rng::GameRngState;
pub(crate) use status_effect_particle_generator::StatusEffectParticleGenerator;
use std::sync::Arc;
use tower::*;
pub use ui_state::UIState;
use upgrade::UpgradeState;
use user_status_effect::UserStatusEffect;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
pub const MAP_SIZE: Wh<BlockUnit> = Wh::new(36, 36);
pub const MAP_OUTSIDE_MARGIN_TILES: f32 = 4.0;

pub const TRAVEL_POINTS: [MapCoord; 7] = [
    MapCoord::new(5, 0),
    MapCoord::new(5, 17),
    MapCoord::new(31, 17),
    MapCoord::new(31, 5),
    MapCoord::new(18, 5),
    MapCoord::new(18, 31),
    MapCoord::new(35, 31),
];

const PROJECTILE_WHOOSH_INTERVAL_MIN_SECS: f32 = 0.5;
const PROJECTILE_WHOOSH_INTERVAL_MAX_SECS: f32 = 0.75;

#[derive(Debug, Clone, State)]
pub struct TowerDamageStats {
    pub tower_id: usize,
    pub tower_kind: TowerKind,
    pub rank: Option<Rank>,
    pub suit: Option<Suit>,
    pub total_damage: Damage,
}

#[derive(Debug, Clone, State)]
pub struct GameMetrics {
    pub total_gold_earned: usize,
    pub total_gold_spent: usize,
    pub current_consecutive_perfect_clears: usize,
    pub max_consecutive_perfect_clears: usize,
    pub tower_damage_stats: Vec<TowerDamageStats>,
    pub total_rerolled_count: usize,
}

#[derive(State)]
pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub backgrounds: Vec<Background>,
    pub decorations: Vec<ImageSprite>,
    pub upgrade_state: UpgradeState,
    pub flow: GameFlow,
    pub hand: Hand<HandItem>,
    pub deck: Deck,
    /// one-based
    pub stage: usize,
    pub left_dice: usize,
    pub monster_spawn_state: MonsterSpawnState,
    pub in_flight_attacks: Vec<attack::InFlightAttack>,
    pub items: Vec<item::ItemWithId>,
    pub gold: usize,
    pub cursor_preview: CursorPreview,
    pub hp: Health,
    pub shield: Shield,
    pub user_status_effects: Vec<UserStatusEffect>,
    pub left_quest_board_refresh_chance: usize,
    pub item_used: bool,
    pub(crate) sim_tick: SimTick,
    pub(crate) sim_scheduler: tick::scheduler::FixedTickScheduler,
    pub(crate) sim_scheduler_report: tick::scheduler::ScheduleReport,
    pub fast_forward_multiplier: FastForwardMultiplier,
    pub rerolled_count: usize,
    pub metrics: GameMetrics,
    pub locale: crate::l10n::Locale,
    pub play_history: PlayHistory,
    pub card_service_notifications: card_notification::CardServiceNotificationState,
    pub config: Arc<GameConfig>,
    pub opened_modals: modal::OpenedModals,
    pub stage_modifiers: StageModifiers,
    pub ui_state: UIState,
    pub status_effect_particle_generator: StatusEffectParticleGenerator,
    pub black_smoke_sources: Vec<field_particle::emitter::BlackSmokeSource>,
    pub effect_events: EffectEventQueue,
    pub base_animation_state: BaseAnimationState,
    pub(crate) discovery: discovery::DiscoveryState,
    pub(crate) rng: GameRngState,

    // headless mode for simulator (no UI side-effects like modals, tooltips, notifications)
    pub(crate) headless: bool,
}
impl GameState {
    /// 현대적인 텍스트 매니저 반환
    pub fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub fn max_shop_slot(&self) -> usize {
        self.upgrade_state.shop_slot_expand() + 2
    }

    pub fn max_hp(&self) -> Health {
        self.config
            .player
            .max_hp
            .saturating_add_delta(self.upgrade_state.max_hp_plus())
    }

    pub fn max_dice_chance(&self) -> usize {
        (self.upgrade_state.dice_chance_plus()
            + self.config.player.base_dice_chance
            + self.stage_modifiers.get_max_rerolls_bonus())
        .saturating_sub(self.stage_modifiers.get_max_rerolls_penalty())
    }

    pub fn generate_rarity(&self) -> crate::rarity::Rarity {
        crate::rarity::Rarity::Common
    }

    /// Returns whether the shop panel is allowed to be opened based on current flow.
    pub fn can_open_shop_panel(&self) -> bool {
        matches!(self.flow, GameFlow::Shopping(_))
    }
    pub fn sim_tick(&self) -> SimTick {
        self.sim_tick
    }

    pub fn sim_scheduler_report(&self) -> tick::scheduler::ScheduleReport {
        self.sim_scheduler_report
    }

    pub fn sim_scheduler_discarded_units(&self) -> u64 {
        self.sim_scheduler.discarded_units()
    }

    pub fn sim_scheduler_backlog(&self) -> SimTickSpan {
        self.sim_scheduler.backlog()
    }

    pub fn is_headless(&self) -> bool {
        self.headless
    }

    pub fn record_tower_damage(&mut self, tower: &attack::TowerInfo, damage: Damage) {
        if damage.is_zero() {
            return;
        }

        if let Some(entry) = self
            .metrics
            .tower_damage_stats
            .iter_mut()
            .find(|entry| entry.tower_id == tower.id)
        {
            entry.total_damage = entry.total_damage.saturating_add(damage);
        } else {
            self.metrics.tower_damage_stats.push(TowerDamageStats {
                tower_id: tower.id,
                tower_kind: tower.kind,
                rank: tower.rank,
                suit: tower.suit,
                total_damage: damage,
            });
        }
    }

    pub fn flush_effect_events(&mut self) {
        let mut active_trail_sound_projectiles = std::collections::HashSet::new();
        let mut active_projectile_sound_ids = PROJECTILE_TRAIL_SOUND_IDS.lock().unwrap();

        for event in self.effect_events.drain() {
            match event {
                GameEffectEvent::SpawnParticle(request) => match request {
                    ParticleSpawnRequest::DamageText(p) => {
                        field_particle::DAMAGE_TEXTS.spawn(p);
                    }
                    ParticleSpawnRequest::Projectile(p) => {
                        field_particle::PROJECTILES.spawn(p);
                    }
                    ParticleSpawnRequest::Trash(p) => {
                        field_particle::TRASHES.spawn(p);
                    }
                    ParticleSpawnRequest::MonsterSoul(p) => {
                        field_particle::MONSTER_SOULS.spawn(p);
                    }
                    ParticleSpawnRequest::MonsterCorpse(p) => {
                        field_particle::MONSTER_CORPSES.spawn(p);
                    }
                    ParticleSpawnRequest::Card(p) => {
                        field_particle::CARDS.spawn(p);
                    }
                    ParticleSpawnRequest::Icon(p) => {
                        field_particle::ICONS.spawn(p);
                    }
                    ParticleSpawnRequest::Heart(p) => {
                        field_particle::HEARTS.spawn(p);
                    }
                    ParticleSpawnRequest::BlackSmoke(p) => {
                        field_particle::BLACK_SMOKES.spawn(p);
                    }
                    ParticleSpawnRequest::Dust(p) => {
                        field_particle::DUSTS.spawn(p);
                    }
                    ParticleSpawnRequest::Attack(p) => {
                        field_particle::ATTACK_PARTICLES.spawn(p);
                    }
                },
                GameEffectEvent::PlaySound(params) => {
                    crate::sound::emit_sound(params);
                }
                GameEffectEvent::PlaySoundDelayed(params, delay) => {
                    crate::sound::emit_sound_after(params, delay);
                }
                GameEffectEvent::SpawnProjectileTrail {
                    trail,
                    start_xy,
                    end_xy,
                    count,
                    presentation_instant,
                } => {
                    let presentation_now = presentation_instant.as_namui();
                    match trail {
                        ProjectileTrail::Burning => {
                            field_particle::emitter::spawn_burning_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::Sparkle => {
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::WindCurve => {
                            field_particle::emitter::spawn_wind_curve_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::Heart => {
                            field_particle::emitter::spawn_heart_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::LightningSparkle => {
                            field_particle::emitter::spawn_lightning_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                            field_particle::emitter::spawn_sparkle_trail(
                                start_xy,
                                end_xy,
                                count,
                                presentation_now,
                            );
                        }
                        ProjectileTrail::None => {}
                    }
                }
                GameEffectEvent::SpawnProjectileHitEffect(
                    hit_effect,
                    impact_xy,
                    presentation_instant,
                ) => {
                    let presentation_now = presentation_instant.as_namui();
                    use crate::game_state::attack::ProjectileHitEffect;
                    match hit_effect {
                        ProjectileHitEffect::CardBurst => {
                            field_particle::emitter::spawn_card_burst(impact_xy, presentation_now);
                        }
                        ProjectileHitEffect::SparkleBurst => {
                            field_particle::emitter::spawn_sparkle_burst(
                                impact_xy,
                                presentation_now,
                            );
                        }
                        ProjectileHitEffect::HeartBurst => {
                            field_particle::emitter::spawn_heart_burst(impact_xy, presentation_now);
                        }
                        ProjectileHitEffect::TrashBounce => {
                            // Trash bounce is handled as direct projectile activity elsewhere.
                        }
                    }
                }
                GameEffectEvent::SpawnLaserBeam(start_xy, end_xy, presentation_instant) => {
                    field_particle::emitter::spawn_laser_beam(
                        start_xy,
                        end_xy,
                        presentation_instant.as_namui(),
                    );
                }
                GameEffectEvent::SpawnTowerRemoveDustBurst(center_xy, presentation_instant) => {
                    field_particle::emitter::spawn_tower_remove_dust_burst(
                        center_xy,
                        presentation_instant.as_namui(),
                    );
                }
                GameEffectEvent::SyncProjectileTrailState {
                    projectile_id,
                    trail,
                    start_xy,
                    end_xy,
                    moved_distance,
                    dt_secs,
                    presentation_instant,
                } => {
                    let presentation_now = presentation_instant.as_namui();
                    active_trail_sound_projectiles.insert(projectile_id);
                    let mut effect_states = PROJECTILE_TRAIL_EFFECT_STATE.lock().unwrap();
                    let state = effect_states.entry(projectile_id).or_default();

                    state.trail_distance_remainder += moved_distance;
                    let spawn_distance = match trail {
                        ProjectileTrail::None => None,
                        ProjectileTrail::Burning => {
                            Some(field_particle::emitter::BURNING_TRAIL_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::Sparkle => {
                            Some(field_particle::emitter::SPARKLE_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::WindCurve => {
                            Some(field_particle::emitter::WIND_CURVE_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::Heart => {
                            Some(field_particle::emitter::HEART_SPAWN_DISTANCE)
                        }
                        ProjectileTrail::LightningSparkle => {
                            Some(field_particle::emitter::LIGHTNING_TRAIL_SPAWN_DISTANCE)
                        }
                    };

                    if let Some(spawn_distance) = spawn_distance {
                        let spawn_count =
                            (state.trail_distance_remainder / spawn_distance).floor() as usize;
                        if spawn_count > 0 {
                            state.trail_distance_remainder -= spawn_count as f32 * spawn_distance;
                            match trail {
                                ProjectileTrail::Burning => {
                                    field_particle::emitter::spawn_burning_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::Sparkle => {
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::WindCurve => {
                                    field_particle::emitter::spawn_wind_curve_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::Heart => {
                                    field_particle::emitter::spawn_heart_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::LightningSparkle => {
                                    field_particle::emitter::spawn_lightning_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        presentation_now,
                                    );
                                }
                                ProjectileTrail::None => {}
                            }
                        }
                    }

                    state.whoosh_cooldown_secs -= dt_secs;
                    if state.whoosh_cooldown_secs <= 0.0 {
                        crate::sound::emit_sound(sound::EmitSoundParams::one_shot(
                            sound::random_whoosh(),
                            sound::SoundGroup::Sfx,
                            sound::VolumePreset::Minimum,
                            sound::SpatialMode::Spatial { position: end_xy },
                        ));
                        state.whoosh_cooldown_secs = rand::thread_rng().gen_range(
                            PROJECTILE_WHOOSH_INTERVAL_MIN_SECS
                                ..=PROJECTILE_WHOOSH_INTERVAL_MAX_SECS,
                        );
                    }

                    let existing_entry = active_projectile_sound_ids.get_mut(&projectile_id);
                    match trail {
                        ProjectileTrail::Burning => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::Burning =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_crackling_fire(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::Burning;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_crackling_fire(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::Burning, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::Sparkle => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::Sparkle =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_shining_ringing(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::Sparkle;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_shining_ringing(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::Sparkle, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::WindCurve => {
                            let sound_id = match existing_entry {
                                Some((existing_trail, sound_id))
                                    if *existing_trail == ProjectileTrail::WindCurve =>
                                {
                                    crate::sound::update_sound_position(*sound_id, end_xy);
                                    *sound_id
                                }
                                Some((existing_trail, sound_id)) => {
                                    crate::sound::stop_sound(*sound_id);
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_wind(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let new_sound_id = crate::sound::emit_sound(params);
                                    *existing_trail = ProjectileTrail::WindCurve;
                                    *sound_id = new_sound_id;
                                    new_sound_id
                                }
                                None => {
                                    let params = sound::EmitSoundParams::looping(
                                        sound::random_wind(),
                                        sound::SoundGroup::Sfx,
                                        sound::VolumePreset::Minimum,
                                        sound::SpatialMode::Spatial { position: end_xy },
                                    )
                                    .with_max_duration(Duration::from_secs(32));
                                    let sound_id = crate::sound::emit_sound(params);
                                    active_projectile_sound_ids.insert(
                                        projectile_id,
                                        (ProjectileTrail::WindCurve, sound_id),
                                    );
                                    sound_id
                                }
                            };
                            let _ = sound_id;
                        }
                        ProjectileTrail::Heart
                        | ProjectileTrail::LightningSparkle
                        | ProjectileTrail::None => {
                            if let Some((_, sound_id)) =
                                active_projectile_sound_ids.remove(&projectile_id)
                            {
                                crate::sound::stop_sound(sound_id);
                            }
                        }
                    }
                }
            }
        }

        let stale_keys: Vec<u64> = active_projectile_sound_ids
            .keys()
            .filter(|key| !active_trail_sound_projectiles.contains(key))
            .cloned()
            .collect();
        for stale_key in stale_keys {
            if let Some((_, sound_id)) = active_projectile_sound_ids.remove(&stale_key) {
                crate::sound::stop_sound(sound_id);
            }
        }
    }

    pub fn set_selected_tower(
        &mut self,
        tower_id: Option<usize>,
        presentation_instant: crate::PresentationInstant,
    ) {
        self.ui_state
            .set_selected_tower(tower_id, presentation_instant);
    }

    pub fn cleanup_unused_tower_popup_states(&mut self) {
        let existing_tower_ids: std::collections::HashSet<usize> =
            self.towers.iter().map(|tower| tower.id()).collect();

        self.ui_state.cleanup_unused_states(&existing_tower_ids);
    }

    pub fn update_camera_shake(
        &mut self,
        dt: Duration,
        presentation_instant: crate::PresentationInstant,
    ) {
        self.camera
            .update_shake(dt, presentation_instant - PresentationInstant::zero());
    }
}

#[derive(Clone, Copy, State)]
pub struct FloorTile {
    pub coord: MapCoord,
}
impl Component for &FloorTile {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(simple_rect(
            TILE_PX_SIZE,
            palette::OUTLINE,
            1.px(),
            Color::TRANSPARENT,
        ));
    }
}

static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

fn create_initial_game_state() -> GameState {
    create_game_state_with_seed(rand::thread_rng().r#gen())
}

pub fn create_game_state_with_seed(seed: u64) -> GameState {
    let config = Arc::new(GameConfig::default_config());
    let presentation_instant = PresentationInstant::capture();
    let decorations = background::generate_decorations();
    let mut game_state = GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera::new(),
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        backgrounds: generate_backgrounds(),
        decorations,
        upgrade_state: Default::default(),
        flow: GameFlow::Initializing,
        hand: Hand::new(std::iter::empty::<HandItem>()),
        stage: 1,
        left_dice: config.player.base_dice_chance,
        monster_spawn_state: MonsterSpawnState::idle(),
        in_flight_attacks: Default::default(),
        items: vec![
            LumpSugarItem::standard().into_item().with_unique_id(),
            LumpSugarItem::standard().into_item().with_unique_id(),
            RubberConeItem::standard().into_item().with_unique_id(),
        ],
        gold: config.player.starting_gold,
        cursor_preview: Default::default(),
        hp: config.player.starting_hp,
        shield: Shield::ZERO,
        user_status_effects: Default::default(),
        left_quest_board_refresh_chance: 0,
        item_used: false,
        sim_tick: SimTick::ZERO,
        sim_scheduler: tick::scheduler::FixedTickScheduler::default(),
        sim_scheduler_report: tick::scheduler::ScheduleReport::default(),
        fast_forward_multiplier: Default::default(),
        rerolled_count: 0,
        locale: crate::l10n::Locale::KOREAN,
        deck: Deck::new(),
        play_history: PlayHistory::new(),
        card_service_notifications: card_notification::CardServiceNotificationState::default(),
        config: Arc::clone(&config),
        opened_modals: modal::OpenedModals::default(),
        stage_modifiers: StageModifiers::new(),
        ui_state: UIState::new(),
        status_effect_particle_generator: StatusEffectParticleGenerator::new(presentation_instant),
        black_smoke_sources: Default::default(),
        effect_events: EffectEventQueue::default(),
        base_animation_state: BaseAnimationState::new(SimTick::ZERO),
        discovery: Default::default(),
        metrics: GameMetrics {
            total_gold_earned: 0,
            total_gold_spent: 0,
            current_consecutive_perfect_clears: 0,
            max_consecutive_perfect_clears: 0,
            tower_damage_stats: Vec::new(),
            total_rerolled_count: 0,
        },

        rng: GameRngState::new(seed),

        headless: false,
    };

    // Start with selecting tower flow and default shop mode (normal shop).
    game_state.action(crate::game_state::GameStateAction::StartStage {
        stage: game_state.stage,
    });
    game_state.action(GameStateAction::GameStart);
    game_state
}

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, create_initial_game_state).0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(move |game_state| {
        f(game_state);
        game_state.persist_discoveries_if_dirty();
    });
}

pub fn set_modal(modal: Option<UserModal>) {
    mutate_game_state(|game_state| {
        game_state.opened_modals.user = modal;
    });
}

pub fn set_overlay_modal(modal: Option<modal::SystemModal>) {
    mutate_game_state(|game_state| {
        game_state.opened_modals.system = modal;
    });
}

pub fn restart_game() {
    mutate_game_state(|game_state| {
        let previous_discoveries = game_state.discovery.clone();
        *game_state = create_initial_game_state();
        game_state.preserve_discoveries_from(&previous_discoveries);
    });
}

impl GameState {
    /// Create a deep-ish clone of the current state for debug snapshotting.
    /// Particle systems are cleared and opened modal is dropped to avoid UI leakage.
    pub fn clone_for_debug(&self) -> GameState {
        GameState {
            monsters: self.monsters.clone(),
            towers: self.towers.clone(),
            camera: self.camera.clone(),
            route: Arc::clone(&self.route),
            backgrounds: self.backgrounds.clone(),
            decorations: self.decorations.clone(),
            upgrade_state: self.upgrade_state.clone(),
            flow: self.flow.clone(),
            hand: self.hand.clone(),
            deck: self.deck.clone(),
            stage: self.stage,
            left_dice: self.left_dice,
            monster_spawn_state: self.monster_spawn_state.clone(),
            in_flight_attacks: self.in_flight_attacks.clone(),
            items: self.items.clone(),
            gold: self.gold,
            cursor_preview: self.cursor_preview.clone(),
            hp: self.hp,
            shield: self.shield,
            user_status_effects: self.user_status_effects.clone(),
            left_quest_board_refresh_chance: self.left_quest_board_refresh_chance,
            item_used: self.item_used,
            sim_tick: self.sim_tick,
            sim_scheduler: self.sim_scheduler,
            sim_scheduler_report: self.sim_scheduler_report,
            fast_forward_multiplier: self.fast_forward_multiplier,
            rerolled_count: self.rerolled_count,
            locale: self.locale,
            play_history: self.play_history.clone(),
            config: Arc::clone(&self.config),
            opened_modals: modal::OpenedModals::default(),
            stage_modifiers: self.stage_modifiers.clone(),
            ui_state: self.ui_state.clone(),
            status_effect_particle_generator: StatusEffectParticleGenerator::new(
                crate::PresentationInstant::capture(),
            ),
            black_smoke_sources: Default::default(),
            effect_events: self.effect_events.clone(),
            base_animation_state: self.base_animation_state.clone(),
            metrics: GameMetrics {
                total_gold_earned: self.metrics.total_gold_earned,
                total_gold_spent: self.metrics.total_gold_spent,
                current_consecutive_perfect_clears: self.metrics.current_consecutive_perfect_clears,
                max_consecutive_perfect_clears: self.metrics.max_consecutive_perfect_clears,
                tower_damage_stats: self.metrics.tower_damage_stats.clone(),
                total_rerolled_count: self.metrics.total_rerolled_count,
            },
            card_service_notifications: self.card_service_notifications.clone(),
            rng: self.rng.clone(),
            headless: self.headless,
            discovery: self.discovery.clone(),
        }
    }

    /// 현재 스테이지의 클리어율을 계산합니다.
    /// 각 스테이지는 2% (100/50), 스테이지 내에서는 누적 처리 체력 / 총 체력으로 계산합니다.
    /// 처리 체력은 피해와 기지 도달 시점에만 증가하므로 몬스터 회복으로 감소하지 않습니다.
    pub fn calculate_clear_rate(&self) -> ClearRate {
        let total_stages = 50_i128;
        let stage_weight_raw = FixedRatio::ONE.div_integer(total_stages as i64).raw() as i128;

        let previous_stages_progress = (self.stage.saturating_sub(1) as i128)
            .min(total_stages)
            .saturating_mul(stage_weight_raw);

        // 스테이지 진행 데이터는 DefenseFlow에 저장되어 있음
        let (start_total_hp, processed_hp_so_far) = match &self.flow {
            crate::game_state::flow::GameFlow::Defense(defense_flow) => (
                defense_flow.stage_progress.start_total_hp,
                defense_flow.stage_progress.processed_hp,
            ),
            _ => (
                Self::calculate_stage_total_hp(self.stage, &self.config, &self.stage_modifiers),
                Health::ZERO,
            ),
        };

        let current_stage_progress = if !start_total_hp.is_zero() {
            let stage_ratio = processed_hp_so_far.ratio_of(start_total_hp);
            RatioProduct::one()
                .with(FixedRatio::from_raw(stage_weight_raw as i64))
                .apply_raw(stage_ratio.raw()) as i128
        } else {
            0
        };

        let total_raw = previous_stages_progress
            .saturating_add(current_stage_progress)
            .min(RATIO_SCALE as i128) as i64;
        ClearRate::from_ratio(FixedRatio::from_raw(total_raw))
    }

    /// 특정 스테이지의 총 몬스터 체력을 계산합니다.
    pub fn calculate_stage_total_hp(
        stage: usize,
        config: &GameConfig,
        stage_modifiers: &StageModifiers,
    ) -> Health {
        let health_multipliers = stage_modifiers.enemy_health_multipliers();
        let (template_queue, _) = monster_spawn::monster_template_queue_table(stage, config);
        template_queue
            .iter()
            .map(|t| t.max_hp.scaled_by_product(health_multipliers))
            .fold(Health::ZERO, Health::saturating_add)
    }
}

pub fn is_boss_stage(stage: usize) -> bool {
    stage.is_multiple_of(5) || (46..=49).contains(&stage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shopping_allows_shop_panel() {
        let mut gs = create_initial_game_state();
        gs.flow = GameFlow::Shopping(crate::game_state::flow::ShoppingFlow::new(&mut gs));
        assert!(gs.can_open_shop_panel());
    }

    #[test]
    fn boss_stage_logic_is_every_fifth_stage_with_final_45_to_50() {
        for stage in [5, 10, 15, 20, 25, 30, 35, 40, 45, 46, 47, 48, 49, 50] {
            assert!(is_boss_stage(stage), "expected stage {} to be boss", stage);
        }
        assert!(!is_boss_stage(51));
    }

    #[test]
    fn authoritative_clone_replay_is_bit_exact() {
        let mut left = create_game_state_with_seed(0x5eed);
        let mut right = left.clone_for_debug();
        let apply_commands = |game_state: &mut GameState| {
            game_state.action(GameStateAction::TakeDamage(Damage::from_raw(7_125)));
            game_state.action(GameStateAction::GainShield(Shield::from_raw(2_500)));
            game_state.action(GameStateAction::TakeDamage(Damage::from_raw(3_250)));
            game_state.action(GameStateAction::Heal(Health::from_raw(1_125)));
            crate::game_state::effect::run_effect(
                game_state,
                &crate::game_state::effect::Effect::IncreaseEnemyHealthPercent {
                    percentage: FixedRatio::from_integer(20),
                },
            );
            crate::game_state::effect::run_effect(
                game_state,
                &crate::game_state::effect::Effect::DecreaseIncomingDamage {
                    multiplier: FixedRatio::from_raw(750_001),
                },
            );
        };
        apply_commands(&mut left);
        apply_commands(&mut right);
        for _ in 0..120 {
            tick::step_simulation(&mut left, PresentationInstant::zero());
            tick::step_simulation(&mut right, PresentationInstant::zero());
        }

        assert_eq!(left.hp, right.hp);
        assert_eq!(left.shield, right.shield);
        assert_eq!(left.max_hp(), right.max_hp());
        assert_eq!(left.calculate_clear_rate(), right.calculate_clear_rate());
        assert_eq!(left.stage, right.stage);
        assert_eq!(left.left_dice, right.left_dice);
        assert_eq!(left.gold, right.gold);
        assert_eq!(left.sim_tick, right.sim_tick);
        assert_eq!(
            left.stage_modifiers.get_enemy_health_multiplier(),
            right.stage_modifiers.get_enemy_health_multiplier()
        );
        assert_eq!(
            left.stage_modifiers.get_damage_reduction_multiplier(),
            right.stage_modifiers.get_damage_reduction_multiplier()
        );
        assert_eq!(left.rng.seed, right.rng.seed);
        assert_eq!(
            left.rng.shop.generation_sequence,
            right.rng.shop.generation_sequence
        );
        assert_eq!(left.monsters.len(), right.monsters.len());
        for (left_monster, right_monster) in left.monsters.iter().zip(&right.monsters) {
            assert_eq!(left_monster.hp, right_monster.hp);
            assert_eq!(left_monster.max_hp, right_monster.max_hp);
            assert_eq!(left_monster.damage, right_monster.damage);
            assert_eq!(
                left_monster.stage_progress_counted,
                right_monster.stage_progress_counted
            );
        }
    }

    #[test]
    fn clear_rate_is_monotonic_and_bounded() {
        let mut game_state = create_game_state_with_seed(0xc1ea);
        let defense = flow::DefenseFlow::new(&game_state);
        game_state.flow = GameFlow::Defense(defense);
        let mut previous = game_state.calculate_clear_rate();
        for processed in [1, 7, 19, 37, 61] {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp = Health::from_integer(processed);
            }
            let current = game_state.calculate_clear_rate();
            assert!(current >= previous);
            assert!(current <= ClearRate::FULL);
            previous = current;
        }
    }

    #[test]
    fn representative_stage_hp_matches_integer_migration_baseline() {
        let config = GameConfig::default_config();
        let modifiers = StageModifiers::new();
        for (stage, expected_raw) in [(1, 338_285), (25, 60_058_670), (50, 179_198_724_000)] {
            assert_eq!(
                GameState::calculate_stage_total_hp(stage, &config, &modifiers).raw(),
                expected_raw,
                "stage {stage} total HP changed"
            );
        }
    }
}
