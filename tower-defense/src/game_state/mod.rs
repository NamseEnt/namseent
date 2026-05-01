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
mod event_handlers;
pub mod fast_forward;
pub mod field_particle;
pub mod flow;
pub mod item;
mod modal;
pub mod monster;
pub(crate) mod monster_spawn;
mod placed_towers;
pub mod play_history;
pub mod poker_action;
pub mod projectile;
mod render;
pub mod stage_modifiers;
mod status_effect_particle_generator;
pub(crate) mod tick;
pub mod tower;
mod tower_info_popup;
mod ui_state;
pub mod upgrade;
mod user_status_effect;

use crate::card::Deck;
use crate::config::GameConfig;
use crate::game_state::item::ItemKind;
use crate::game_state::stage_modifiers::StageModifiers;
use crate::hand::{Hand, HandItem, HandSlotId};
use crate::route::*;
use crate::*;
use background::{Background, generate_backgrounds};
pub use base::*;
pub(crate) use camera::Camera;
use cursor_preview::CursorPreview;
pub use effect_event::*;
use fast_forward::FastForwardMultiplier;
use flow::GameFlow;
use item::{Effect, Item};
pub use modal::Modal;
pub use monster::*;
use monster_spawn::*;
use namui::*;
use placed_towers::PlacedTowers;
use play_history::PlayHistory;
use projectile::*;
use rand::Rng;
pub use render::*;
pub(crate) use status_effect_particle_generator::StatusEffectParticleGenerator;
use std::sync::Arc;
use tower::*;
pub use ui_state::UIState;
use upgrade::UpgradeState;
use user_status_effect::UserStatusEffect;

/// The size of a tile in pixels, with zoom level 1.0.
pub const TILE_PX_SIZE: Wh<Px> = Wh::new(px(128.0), px(128.0));
pub const MAP_SIZE: Wh<BlockUnit> = Wh::new(36, 36);
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
    pub rank: Rank,
    pub suit: Suit,
    pub total_damage: f32,
}

#[derive(Debug, Clone, State)]
pub struct GameMetrics {
    pub total_gold_earned: usize,
    pub total_gold_spent: usize,
    pub current_consecutive_perfect_clears: usize,
    pub max_consecutive_perfect_clears: usize,
    pub tower_damage_stats: Vec<TowerDamageStats>,
}

#[derive(State)]
pub struct GameState {
    pub monsters: Vec<Monster>,
    pub towers: PlacedTowers,
    pub camera: Camera,
    pub route: Arc<Route>,
    pub backgrounds: Vec<Background>,
    pub upgrade_state: UpgradeState,
    pub flow: GameFlow,
    pub hand: Hand<HandItem>,
    pub deck: Deck,
    /// one-based
    pub stage: usize,
    pub left_dice: usize,
    pub monster_spawn_state: MonsterSpawnState,
    pub projectiles: Vec<Projectile>,
    pub delayed_hits: Vec<attack::DelayedHit>,
    pub items: Vec<item::Item>,
    pub gold: usize,
    pub cursor_preview: CursorPreview,
    pub hp: f32,
    pub shield: f32,
    pub user_status_effects: Vec<UserStatusEffect>,
    pub left_quest_board_refresh_chance: usize,
    pub item_used: bool,
    pub(crate) game_now: Instant,
    pub fast_forward_multiplier: FastForwardMultiplier,
    pub rerolled_count: usize,
    pub metrics: GameMetrics,
    pub locale: crate::l10n::Locale,
    pub play_history: PlayHistory,
    pub config: Arc<GameConfig>,
    pub opened_modal: Option<Modal>,
    pub stage_modifiers: StageModifiers,
    pub ui_state: UIState,
    pub status_effect_particle_generator: StatusEffectParticleGenerator,
    pub black_smoke_sources: Vec<field_particle::emitter::BlackSmokeSource>,
    pub effect_events: EffectEventQueue,
    pub base_animation_state: BaseAnimationState,

    // panel open states controlled by input/flow
    pub hand_panel_forced_open: bool,
    pub shop_panel_forced_open: bool,
}
impl GameState {
    /// 현대적인 텍스트 매니저 반환
    pub fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub fn max_shop_slot(&self) -> usize {
        self.upgrade_state.shop_slot_expand() + 2
    }

    pub fn max_hp(&self) -> f32 {
        self.config.player.max_hp + self.upgrade_state.pea_max_hp_plus() as f32
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

    /// Returns whether the hand panel is allowed to be opened based on current flow.
    pub fn can_open_hand_panel(&self) -> bool {
        matches!(
            self.flow,
            GameFlow::SelectingTower(_) | GameFlow::PlacingTower
        )
    }

    /// Returns whether the shop panel is allowed to be opened based on current flow.
    pub fn can_open_shop_panel(&self) -> bool {
        matches!(self.flow, GameFlow::SelectingTower(_))
    }

    /// Toggle panels according to rules described in UI feature request.
    ///
    /// * If any panel that is allowed is currently open, close both.
    /// * Otherwise open all allowed panels (disallowed ones stay closed).
    pub fn toggle_panels(&mut self) {
        let hand_allowed = self.can_open_hand_panel();
        let shop_allowed = self.can_open_shop_panel();
        let hand_open = hand_allowed && self.hand_panel_forced_open;
        let shop_open = shop_allowed && self.shop_panel_forced_open;
        if hand_open || shop_open {
            self.hand_panel_forced_open = false;
            self.shop_panel_forced_open = false;
        } else {
            if hand_allowed {
                self.hand_panel_forced_open = true;
            }
            if shop_allowed {
                self.shop_panel_forced_open = true;
            }
        }
    }

    pub fn now(&self) -> Instant {
        self.game_now
    }

    pub fn record_tower_damage(
        &mut self,
        tower_id: usize,
        tower_kind: TowerKind,
        rank: Rank,
        suit: Suit,
        damage: f32,
    ) {
        if damage <= 0.0 {
            return;
        }

        if let Some(entry) = self
            .metrics
            .tower_damage_stats
            .iter_mut()
            .find(|entry| entry.tower_id == tower_id)
        {
            entry.total_damage += damage;
        } else {
            self.metrics.tower_damage_stats.push(TowerDamageStats {
                tower_id,
                tower_kind,
                rank,
                suit,
                total_damage: damage,
            });
        }
    }

    pub fn advance_time(&mut self, dt: Duration) {
        self.game_now += dt;
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
                    now,
                } => match trail {
                    ProjectileTrail::Burning => {
                        field_particle::emitter::spawn_burning_trail(start_xy, end_xy, count, now);
                    }
                    ProjectileTrail::Sparkle => {
                        field_particle::emitter::spawn_sparkle_trail(start_xy, end_xy, count, now);
                    }
                    ProjectileTrail::WindCurve => {
                        field_particle::emitter::spawn_wind_curve_trail(
                            start_xy, end_xy, count, now,
                        );
                    }
                    ProjectileTrail::Heart => {
                        field_particle::emitter::spawn_heart_trail(start_xy, end_xy, count, now);
                    }
                    ProjectileTrail::LightningSparkle => {
                        field_particle::emitter::spawn_lightning_trail(
                            start_xy, end_xy, count, now,
                        );
                        field_particle::emitter::spawn_sparkle_trail(start_xy, end_xy, count, now);
                    }
                    ProjectileTrail::None => {}
                },
                GameEffectEvent::SpawnProjectileHitEffect(hit_effect, impact_xy, now) => {
                    use crate::game_state::attack::ProjectileHitEffect;
                    match hit_effect {
                        ProjectileHitEffect::CardBurst => {
                            field_particle::emitter::spawn_card_burst(impact_xy, now);
                        }
                        ProjectileHitEffect::SparkleBurst => {
                            field_particle::emitter::spawn_sparkle_burst(impact_xy, now);
                        }
                        ProjectileHitEffect::HeartBurst => {
                            field_particle::emitter::spawn_heart_burst(impact_xy, now);
                        }
                        ProjectileHitEffect::TrashBounce => {
                            // Trash bounce is handled as direct projectile activity elsewhere.
                        }
                    }
                }
                GameEffectEvent::SpawnLaserBeam(start_xy, end_xy, now) => {
                    field_particle::emitter::spawn_laser_beam(start_xy, end_xy, now);
                }
                GameEffectEvent::SpawnTowerRemoveDustBurst(center_xy, now) => {
                    field_particle::emitter::spawn_tower_remove_dust_burst(center_xy, now);
                }
                GameEffectEvent::SyncProjectileTrailState {
                    projectile_id,
                    trail,
                    start_xy,
                    end_xy,
                    moved_distance,
                    dt_secs,
                    now,
                } => {
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
                                        now,
                                    );
                                }
                                ProjectileTrail::Sparkle => {
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        now,
                                    );
                                }
                                ProjectileTrail::WindCurve => {
                                    field_particle::emitter::spawn_wind_curve_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        now,
                                    );
                                }
                                ProjectileTrail::Heart => {
                                    field_particle::emitter::spawn_heart_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        now,
                                    );
                                }
                                ProjectileTrail::LightningSparkle => {
                                    field_particle::emitter::spawn_lightning_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        now,
                                    );
                                    field_particle::emitter::spawn_sparkle_trail(
                                        start_xy,
                                        end_xy,
                                        spawn_count,
                                        now,
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

    pub fn set_selected_tower(&mut self, tower_id: Option<usize>) {
        self.ui_state.set_selected_tower(tower_id, self.now());
    }

    pub fn cleanup_unused_tower_popup_states(&mut self) {
        let existing_tower_ids: std::collections::HashSet<usize> =
            self.towers.iter().map(|tower| tower.id()).collect();

        self.ui_state.cleanup_unused_states(&existing_tower_ids);
    }

    pub fn update_camera_shake(&mut self, dt: Duration) {
        self.camera
            .update_shake(dt, self.game_now - Instant::new(Duration::ZERO));
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
    let config = Arc::new(GameConfig::default_config());
    let now = Instant::now();
    let mut game_state = GameState {
        monsters: Default::default(),
        towers: Default::default(),
        camera: Camera::new(),
        route: calculate_routes(&[], &TRAVEL_POINTS, MAP_SIZE).unwrap(),
        backgrounds: generate_backgrounds(),
        upgrade_state: Default::default(),
        flow: GameFlow::Initializing,
        hand: Hand::new(std::iter::empty::<HandItem>()),
        stage: 1,
        left_dice: config.player.base_dice_chance,
        monster_spawn_state: MonsterSpawnState::idle(),
        projectiles: Default::default(),
        delayed_hits: Default::default(),
        items: vec![
            Item {
                kind: ItemKind::LumpSugar,
                effect: Effect::ExtraDice,
            },
            Item {
                kind: ItemKind::LumpSugar,
                effect: Effect::ExtraDice,
            },
            Item {
                kind: ItemKind::GrantBarricades,
                effect: Effect::AddTowerCardToPlacementHand {
                    tower_kind: TowerKind::Barricade,
                    suit: Suit::Spades,
                    rank: Rank::Ace,
                    count: 1,
                },
            },
        ],
        gold: config.player.starting_gold,
        cursor_preview: Default::default(),
        hp: config.player.starting_hp,
        shield: 0.0,
        user_status_effects: Default::default(),
        left_quest_board_refresh_chance: 0,
        item_used: false,
        game_now: now,
        fast_forward_multiplier: Default::default(),
        rerolled_count: 0,
        locale: crate::l10n::Locale::KOREAN,
        deck: Deck::new(0),
        play_history: PlayHistory::new(),
        config: Arc::clone(&config),
        opened_modal: None,
        stage_modifiers: StageModifiers::new(),
        ui_state: UIState::new(),
        status_effect_particle_generator: StatusEffectParticleGenerator::new(now),
        black_smoke_sources: Default::default(),
        effect_events: EffectEventQueue::default(),
        base_animation_state: BaseAnimationState::new(now),
        metrics: GameMetrics {
            total_gold_earned: config.player.starting_gold,
            total_gold_spent: 0,
            current_consecutive_perfect_clears: 0,
            max_consecutive_perfect_clears: 0,
            tower_damage_stats: Vec::new(),
        },

        // start panels in opened state by default (if flow allows later)
        hand_panel_forced_open: true,
        shop_panel_forced_open: true,
    };

    // Start with selecting tower flow and default shop mode (normal shop).
    game_state.goto_selecting_tower();
    game_state.record_game_start();
    game_state
}

pub fn init_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, create_initial_game_state).0
}

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.atom(&GAME_STATE_ATOM).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

pub fn set_modal(modal: Option<Modal>) {
    mutate_game_state(|game_state| {
        game_state.opened_modal = modal;
    });
}

pub fn restart_game() {
    GAME_STATE_ATOM.mutate(|game_state| {
        *game_state = create_initial_game_state();
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
            upgrade_state: self.upgrade_state.clone(),
            flow: self.flow.clone(),
            hand: self.hand.clone(),
            deck: self.deck.clone(),
            stage: self.stage,
            left_dice: self.left_dice,
            monster_spawn_state: self.monster_spawn_state.clone(),
            projectiles: self.projectiles.clone(),
            delayed_hits: self.delayed_hits.clone(),
            items: self.items.clone(),
            gold: self.gold,
            cursor_preview: self.cursor_preview.clone(),
            hp: self.hp,
            shield: self.shield,
            user_status_effects: self.user_status_effects.clone(),
            left_quest_board_refresh_chance: self.left_quest_board_refresh_chance,
            item_used: self.item_used,
            game_now: self.game_now,
            fast_forward_multiplier: self.fast_forward_multiplier,
            rerolled_count: self.rerolled_count,
            locale: self.locale,
            play_history: self.play_history.clone(),
            config: Arc::clone(&self.config),
            opened_modal: None,
            stage_modifiers: self.stage_modifiers.clone(),
            ui_state: self.ui_state.clone(),
            status_effect_particle_generator: StatusEffectParticleGenerator::new(self.game_now),
            black_smoke_sources: Default::default(),
            effect_events: self.effect_events.clone(),
            base_animation_state: self.base_animation_state.clone(),
            metrics: GameMetrics {
                total_gold_earned: self.metrics.total_gold_earned,
                total_gold_spent: self.metrics.total_gold_spent,
                current_consecutive_perfect_clears: self.metrics.current_consecutive_perfect_clears,
                max_consecutive_perfect_clears: self.metrics.max_consecutive_perfect_clears,
                tower_damage_stats: self.metrics.tower_damage_stats.clone(),
            },
            hand_panel_forced_open: self.hand_panel_forced_open,
            shop_panel_forced_open: self.shop_panel_forced_open,
        }
    }

    /// 현재 스테이지의 클리어율을 계산합니다.
    /// 각 스테이지는 2% (100/50), 스테이지 내에서는 (총 체력 - 남은 체력) / 총 체력 비율로 계산
    /// 체력 회복을 고려하여 실제 남은 몬스터 체력을 기준으로 계산합니다.
    pub fn calculate_clear_rate(&self) -> f32 {
        let total_stages = 50.0;
        let stage_weight = 100.0 / total_stages; // 2%

        // 이전 스테이지 완료율
        let previous_stages_progress = (self.stage.saturating_sub(1) as f32) * stage_weight;

        // 스테이지 진행 데이터는 DefenseFlow에 저장되어 있음
        let (start_total_hp, processed_hp_so_far) = match &self.flow {
            crate::game_state::flow::GameFlow::Defense(defense_flow) => (
                defense_flow.stage_progress.start_total_hp,
                defense_flow.stage_progress.processed_hp,
            ),
            _ => (
                Self::calculate_stage_total_hp(self.stage, &self.config, &self.stage_modifiers),
                0.0,
            ),
        };

        // 현재 남아있는 몬스터들의 이미 소모된 체력(= max_hp - 현재 hp)을 합산
        let remaining_processed_hp: f32 = self
            .monsters
            .iter()
            .map(|monster| (monster.max_hp - monster.hp.max(0.0)).max(0.0))
            .sum();

        let total_processed_hp = processed_hp_so_far + remaining_processed_hp;

        let current_stage_progress = if start_total_hp > 0.0 {
            (total_processed_hp / start_total_hp).min(1.0) * stage_weight
        } else {
            0.0
        };

        (previous_stages_progress + current_stage_progress).min(100.0)
    }

    /// 특정 스테이지의 총 몬스터 체력을 계산합니다.
    pub fn calculate_stage_total_hp(
        stage: usize,
        config: &GameConfig,
        stage_modifiers: &StageModifiers,
    ) -> f32 {
        let health_multiplier = stage_modifiers.get_enemy_health_multiplier();
        let (template_queue, _) = monster_spawn::monster_template_queue_table(stage, config);
        template_queue
            .iter()
            .map(|t| t.max_hp * health_multiplier)
            .sum()
    }
}

pub fn is_boss_stage(stage: usize) -> bool {
    stage.is_multiple_of(5) || (46..=49).contains(&stage)
}

/// Make sure that the tower can be placed at the given coord.
pub fn place_tower(tower: Tower, placing_tower_slot_id: HandSlotId) {
    crate::game_state::mutate_game_state(move |game_state| {
        game_state.place_tower(tower);
        game_state.hand.delete_slots(&[placing_tower_slot_id]);

        // Auto-select the first card (tower or barricade) if available
        if let Some(first_slot_id) = game_state.hand.get_slot_id_by_index(0)
            && game_state
                .hand
                .get_item(first_slot_id)
                .and_then(|item| item.as_tower())
                .is_some()
        {
            game_state.hand.select_slot(first_slot_id);
        }
    });
}

// Unit tests that exercise panel toggle behavior.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_panels_basic_scenarios() {
        let mut gs = create_initial_game_state();

        // initially flow is SelectingTower (round 0 default shop pick)
        assert!(gs.can_open_hand_panel());
        assert!(gs.can_open_shop_panel()); // shop panel is allowed in selecting tower flow

        // selecting tower flow: hand and shop panels are both allowed
        assert!(gs.hand_panel_forced_open);
        assert!(gs.shop_panel_forced_open);

        gs.toggle_panels();
        // toggling when one or both allowed should close both
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // reopening when none open should open both allowed panels again
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(gs.shop_panel_forced_open);

        // enter selecting tower flow - both hand and shop are allowed in this flow.
        gs.goto_selecting_tower();
        assert!(gs.can_open_hand_panel());
        assert!(gs.can_open_shop_panel());
        // forced flags were true already; both panels open
        assert!(gs.hand_panel_forced_open && gs.can_open_hand_panel());
        assert!(gs.shop_panel_forced_open && gs.can_open_shop_panel());

        // space should close both allowed panels
        gs.toggle_panels();
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // closing again should reopen both panels since they are allowed
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(gs.shop_panel_forced_open);

        // go to placing tower flow: hand allowed, shop not
        gs.goto_placing_tower(crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::Barricade,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        ));
        assert!(gs.can_open_hand_panel());
        assert!(!gs.can_open_shop_panel());

        // forced flags remain whatever they were; toggle logic should respect permissions
        // shop is not allowed in placing flow, so it may still be forced open in state
        assert!(gs.hand_panel_forced_open);
        assert!(gs.shop_panel_forced_open);

        // space when hand is open should close both (shop will be closed but can't open anyway)
        gs.toggle_panels();
        assert!(!gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);

        // toggle again should reopen only the hand panel since shop is not allowed
        gs.toggle_panels();
        assert!(gs.hand_panel_forced_open);
        assert!(!gs.shop_panel_forced_open);
    }

    #[test]
    fn selecting_tower_allows_shop_panel() {
        let mut gs = create_initial_game_state();
        gs.flow = GameFlow::SelectingTower(crate::game_state::flow::SelectingTowerFlow::new(&gs));
        assert!(gs.can_open_shop_panel());
    }

    #[test]
    fn boss_stage_logic_is_every_fifth_stage_with_final_45_to_50() {
        for stage in [5, 10, 15, 20, 25, 30, 35, 40, 45, 46, 47, 48, 49, 50] {
            assert!(is_boss_stage(stage), "expected stage {} to be boss", stage);
        }
        assert!(!is_boss_stage(51));
    }
}
