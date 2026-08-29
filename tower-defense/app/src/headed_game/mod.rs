use crate::game_state::GameState;
use namui::*;
use std::ops::{Deref, DerefMut};

#[derive(State)]
pub(crate) struct HeadedGame {
    pub(crate) state: GameState,
    pub(crate) locale: crate::l10n::Locale,
    pub(crate) play_history: crate::game_state::play_history::PlayHistory,
    pub(crate) discovery: crate::game_state::discovery::DiscoveryState,
    pub(crate) card_service_notifications:
        crate::game_state::card_notification::CardServiceNotificationState,
    pub(crate) opened_modals: crate::game_state::modal::OpenedModals,
    pub(crate) presentation_events: crate::game_state::PresentationEventQueue,
    pub(crate) base_animation_state: crate::game_state::BaseAnimationState,
    pub(crate) black_smoke_sources:
        Vec<crate::game_state::field_particle::emitter::BlackSmokeSource>,
    pub(crate) status_effect_particle_generator: crate::game_state::StatusEffectParticleGenerator,
    pub(crate) ui_state: crate::game_state::UIState,
    pub(crate) sim_scheduler: crate::game_state::tick::scheduler::FixedTickScheduler,
    pub(crate) backgrounds: Vec<crate::game_state::background::Background>,
    pub(crate) decorations: Vec<ImageSprite>,
    pub(crate) cursor_preview: crate::game_state::cursor_preview::CursorPreview,
    pub(crate) camera: crate::game_state::Camera,
    pub(crate) fast_forward_multiplier: crate::game_state::fast_forward::FastForwardMultiplier,
    pub(crate) sim_scheduler_report: crate::game_state::tick::scheduler::ScheduleReport,
}

impl HeadedGame {
    pub(crate) fn state(&self) -> &GameState {
        &self.state
    }

    pub(crate) fn locale(&self) -> crate::l10n::Locale {
        self.locale
    }

    pub(crate) fn text(&self) -> crate::l10n::TextManager {
        crate::l10n::TextManager::new(self.locale)
    }

    pub(crate) fn camera(&self) -> &crate::game_state::Camera {
        &self.camera
    }

    pub(crate) fn camera_mut(&mut self) -> &mut crate::game_state::Camera {
        &mut self.camera
    }

    pub(crate) fn ui_state(&self) -> &crate::game_state::UIState {
        &self.ui_state
    }

    pub(crate) fn opened_modals(&self) -> &crate::game_state::modal::OpenedModals {
        &self.opened_modals
    }

    pub(crate) fn opened_modals_mut(&mut self) -> &mut crate::game_state::modal::OpenedModals {
        &mut self.opened_modals
    }

    pub(crate) fn discovery(&self) -> &crate::game_state::discovery::DiscoveryState {
        &self.discovery
    }

    pub(crate) fn backgrounds(&self) -> &[crate::game_state::background::Background] {
        &self.backgrounds
    }

    pub(crate) fn cursor_preview(&self) -> &crate::game_state::cursor_preview::CursorPreview {
        &self.cursor_preview
    }

    pub(crate) fn cursor_preview_mut(
        &mut self,
    ) -> &mut crate::game_state::cursor_preview::CursorPreview {
        &mut self.cursor_preview
    }

    pub(crate) fn decorations(&self) -> &[ImageSprite] {
        &self.decorations
    }

    pub(crate) fn new(mut state: GameState) -> Self {
        let mut play_history = crate::game_state::play_history::PlayHistory::new();
        play_history
            .events
            .extend(state.take_pending_history_events());
        let presentation_events = std::mem::take(state.pending_presentation_events_mut());
        let mut sim_scheduler = crate::game_state::tick::scheduler::FixedTickScheduler::default();
        let mut presentation_metadata = std::mem::take(&mut state.presentation_metadata);
        presentation_metadata.refresh_from_core(state.raw_core_state());
        state.presentation_metadata = presentation_metadata;
        let raw_snapshot = state.raw_render_snapshot();
        let monster_metadata = state
            .presentation_metadata
            .monsters
            .iter()
            .map(|monster| (monster.id, monster.rotation, monster.y_offset))
            .collect::<Vec<_>>();
        let projectile_metadata = state
            .presentation_metadata
            .projectiles
            .iter()
            .map(|projectile| (projectile.id, projectile.projectile_kind))
            .collect::<Vec<_>>();
        let tower_metadata = state
            .presentation_metadata
            .towers
            .iter()
            .map(|tower| (tower.id, tower.animation_kind, tower.y_ratio_offset))
            .collect::<Vec<_>>();
        sim_scheduler.rebase_render_snapshot(
            crate::game_state::render_snapshot::WorldRenderSnapshot::capture_from_raw_snapshot(
                &raw_snapshot,
                &monster_metadata,
                &projectile_metadata,
                &tower_metadata,
                (Xy::single(1.0), Xy::single(1.0)),
            ),
        );
        Self {
            locale: state.locale(),
            state,
            play_history,
            discovery: Default::default(),
            card_service_notifications: Default::default(),
            opened_modals: Default::default(),
            presentation_events,
            base_animation_state: crate::game_state::BaseAnimationState::new(crate::SimTick::ZERO),
            black_smoke_sources: Default::default(),
            status_effect_particle_generator: crate::game_state::StatusEffectParticleGenerator::new(
                crate::PresentationInstant::capture(),
            ),
            ui_state: crate::game_state::UIState::new(),
            sim_scheduler,
            backgrounds: crate::game_state::background::generate_backgrounds(),
            decorations: crate::game_state::background::generate_decorations(),
            cursor_preview: Default::default(),
            camera: crate::game_state::Camera::new(),
            fast_forward_multiplier: Default::default(),
            sim_scheduler_report: Default::default(),
        }
    }

    fn append_pending_presentation_events(&mut self) {
        self.presentation_events
            .events
            .append(&mut self.state.pending_presentation_events_mut().events);
    }

    pub(crate) fn flush_pending_action_effects(&mut self) {
        let effects = self.state.take_pending_action_effects();
        self.play_history.events.extend(effects.history_events);
        self.card_service_notifications
            .queue
            .extend(effects.card_service_notifications);
        self.presentation_events
            .events
            .extend(effects.presentation_events.events);
        self.merge_pending_discoveries(effects.discoveries);
    }

    pub(crate) fn push_presentation_event(&mut self, event: crate::game_state::PresentationEvent) {
        self.presentation_events.push(event);
    }

    pub(crate) fn consume_core_events(&mut self, presentation_instant: crate::PresentationInstant) {
        let events = self.state.drain_core_events();
        if self.state.headless {
            crate::game_state::core_event_bridge::consume_headless(events);
        } else {
            crate::game_state::core_event_bridge::consume_headed(
                &mut self.state,
                events,
                presentation_instant,
            );
        }
    }
}

impl Deref for HeadedGame {
    type Target = GameState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for HeadedGame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl HeadedGame {
    pub(crate) fn restore_loaded_state(
        &mut self,
        loaded: crate::game_state::persistence::LoadedGameState,
    ) -> Result<(), crate::game_state::persistence::PersistenceError> {
        match loaded {
            crate::game_state::persistence::LoadedGameState::Current(persisted) => {
                crate::game_state::persistence::restore(&mut self.state, *persisted)?;
            }
            crate::game_state::persistence::LoadedGameState::LegacyCoreSnapshot(snapshot) => {
                crate::game_state::persistence::LegacyGameStateMigration::load_core_into(
                    &mut self.state,
                    *snapshot,
                )?;
            }
        }
        self.locale = self.state.locale();

        let raw_snapshot = self.state.raw_render_snapshot();
        let monster_metadata = self
            .state
            .presentation_metadata
            .monsters
            .iter()
            .map(|monster| (monster.id, monster.rotation, monster.y_offset))
            .collect::<Vec<_>>();
        let projectile_metadata = self
            .state
            .presentation_metadata
            .projectiles
            .iter()
            .map(|projectile| (projectile.id, projectile.projectile_kind))
            .collect::<Vec<_>>();
        let tower_metadata = self
            .state
            .presentation_metadata
            .towers
            .iter()
            .map(|tower| (tower.id, tower.animation_kind, tower.y_ratio_offset))
            .collect::<Vec<_>>();
        self.sim_scheduler.rebase_render_snapshot(
            crate::game_state::render_snapshot::WorldRenderSnapshot::capture_from_raw_snapshot(
                &raw_snapshot,
                &monster_metadata,
                &projectile_metadata,
                &tower_metadata,
                self.render_base_scales(),
            ),
        );
        self.base_animation_state =
            crate::game_state::BaseAnimationState::new(self.state.sim_tick());
        self.sim_scheduler_report = Default::default();
        Ok(())
    }

    pub(crate) fn take_presentation_events(&mut self) -> Vec<crate::game_state::PresentationEvent> {
        self.presentation_events.drain().collect()
    }

    pub(crate) fn replace_presentation_events(
        &mut self,
        events: Vec<crate::game_state::PresentationEvent>,
    ) {
        self.presentation_events.events = events;
    }

    pub(crate) fn apply_presentation_triggers(
        &mut self,
        presentation_instant: crate::PresentationInstant,
    ) {
        self.state.apply_presentation_triggers(
            presentation_instant,
            &mut self.base_animation_state,
            &mut self.black_smoke_sources,
        );
        self.append_pending_presentation_events();
        let events = self.take_presentation_events();
        let mut remaining = Vec::with_capacity(events.len());
        for event in events {
            match event {
                crate::game_state::PresentationEvent::ShakeCamera { intensity } => {
                    self.camera.add_shake_intensity(intensity);
                }
                event => remaining.push(event),
            }
        }
        self.replace_presentation_events(remaining);
    }

    pub(crate) fn update_base_animations(&mut self, sim_tick: crate::SimTick) {
        self.state
            .update_base_animations(sim_tick, &mut self.base_animation_state);
    }

    pub(crate) fn render_base_scales(&self) -> (Xy<f32>, Xy<f32>) {
        self.state.render_base_scales(&self.base_animation_state)
    }

    pub(crate) fn capture_render_snapshot(
        &mut self,
    ) -> crate::game_state::render_snapshot::WorldRenderSnapshot {
        let raw_snapshot = self.state.raw_render_snapshot();
        let mut presentation_metadata = std::mem::take(&mut self.state.presentation_metadata);
        presentation_metadata.refresh_from_core(self.state.raw_core_state());
        self.state.presentation_metadata = presentation_metadata;
        let monster_metadata = self
            .state
            .presentation_metadata
            .monsters
            .iter()
            .map(|monster| (monster.id, monster.rotation, monster.y_offset))
            .collect::<Vec<_>>();
        let projectile_metadata = self
            .state
            .presentation_metadata
            .projectiles
            .iter()
            .map(|projectile| (projectile.id, projectile.projectile_kind))
            .collect::<Vec<_>>();
        let tower_metadata = self
            .state
            .presentation_metadata
            .towers
            .iter()
            .map(|tower| (tower.id, tower.animation_kind, tower.y_ratio_offset))
            .collect::<Vec<_>>();
        crate::game_state::render_snapshot::WorldRenderSnapshot::capture_from_raw_snapshot(
            &raw_snapshot,
            &monster_metadata,
            &projectile_metadata,
            &tower_metadata,
            self.render_base_scales(),
        )
    }

    pub(crate) fn render_frame(
        &self,
    ) -> Option<crate::game_state::tick::scheduler::RenderFrame<'_>> {
        self.sim_scheduler.render_frame()
    }

    pub(crate) fn flush_presentation_events(
        &mut self,
        presentation_instant: crate::PresentationInstant,
    ) {
        self.state
            .pending_presentation_events_mut()
            .events
            .append(&mut self.presentation_events.events);
        self.state.flush_presentation_events(presentation_instant);
    }

    pub(crate) fn clear_presentation_events(&mut self) {
        self.presentation_events.clear();
        self.state.clear_presentation_events();
    }

    pub(crate) fn set_selected_tower(
        &mut self,
        tower_id: Option<crate::TowerId>,
        presentation_instant: crate::PresentationInstant,
    ) {
        self.ui_state
            .set_selected_tower(tower_id, presentation_instant);
    }

    pub(crate) fn cleanup_unused_tower_popup_states(&mut self) {
        let existing_tower_ids: std::collections::HashSet<crate::TowerId> = self
            .state
            .raw_core_state()
            .towers()
            .iter()
            .filter_map(|tower| tower.id.map(crate::TowerId::from_raw))
            .collect();
        self.ui_state.cleanup_unused_states(&existing_tower_ids);
    }
}
