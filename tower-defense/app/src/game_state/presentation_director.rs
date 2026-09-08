use crate::game_state::card_notification::{
    CardServiceNotification, CardServiceNotificationPlayback,
};
use crate::game_state::monster::MonsterKind;
use crate::{PresentationInstant, SimTick};
use namui::*;

pub(crate) const CARD_NOTIFICATION_DURATION_SECS: f32 = 3.0;
pub(crate) const DEFENSE_INTRO_DURATION_SECS: f32 = 5.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub(crate) struct PresentationId(u64);

impl PresentationId {
    pub(crate) fn from_serial(serial: u64) -> Self {
        Self(serial)
    }

    pub(crate) fn for_core_event(sim_tick: SimTick, event_index: usize) -> Self {
        let tick = sim_tick.ticks().wrapping_mul(0x9e37_79b9_7f4a_7c15);
        Self((1_u64 << 63) | tick.wrapping_add(event_index as u64))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub(crate) enum PresentationSource {
    CoreEvent,
    PendingActionEffect,
    CompatibilityAction,
    RestoreBootstrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub(crate) enum PresentationPolicy {
    Blocking,
    NonBlocking,
}

#[derive(Debug, Clone, PartialEq, State)]
pub(crate) enum PresentationPayload {
    CardServiceNotification(CardServiceNotification),
    DefenseIntro {
        stage: usize,
        monster_kind: MonsterKind,
        boss: bool,
    },
}

#[derive(Debug, Clone, PartialEq, State)]
pub(crate) struct PresentationRequest {
    pub(crate) id: PresentationId,
    pub(crate) source: PresentationSource,
    pub(crate) policy: PresentationPolicy,
    pub(crate) payload: PresentationPayload,
    pub(crate) created_at: PresentationInstant,
    pub(crate) sim_tick: Option<SimTick>,
}

#[derive(Debug, Clone, PartialEq, State)]
struct ActivePresentation {
    request: PresentationRequest,
    started_at: PresentationInstant,
    card_playback: Option<CardServiceNotificationPlayback>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActiveDefenseIntro {
    pub(crate) stage: usize,
    pub(crate) monster_kind: MonsterKind,
    pub(crate) boss: bool,
    pub(crate) started_at: PresentationInstant,
}

#[derive(Debug, Clone, Default, PartialEq, State)]
pub(crate) struct PresentationDirector {
    pending: Vec<PresentationRequest>,
    active: Option<ActivePresentation>,
    completed: Vec<PresentationId>,
    serial: u64,
    last_presentation_instant: Option<PresentationInstant>,
}

impl PresentationDirector {
    pub(crate) fn enqueue(
        &mut self,
        source: PresentationSource,
        policy: PresentationPolicy,
        payload: PresentationPayload,
        created_at: PresentationInstant,
        sim_tick: Option<SimTick>,
    ) -> PresentationId {
        let id = self.allocate_id();
        self.enqueue_with_id(PresentationRequest {
            id,
            source,
            policy,
            payload,
            created_at,
            sim_tick,
        });
        id
    }

    pub(crate) fn enqueue_with_id(&mut self, request: PresentationRequest) -> bool {
        if self.contains_id(request.id) {
            return false;
        }
        self.pending.push(request);
        true
    }

    pub(crate) fn enqueue_card_notification(
        &mut self,
        notification: CardServiceNotification,
        sim_tick: SimTick,
    ) -> PresentationId {
        let created_at = self
            .last_presentation_instant
            .unwrap_or_else(PresentationInstant::zero);
        self.enqueue(
            PresentationSource::PendingActionEffect,
            PresentationPolicy::Blocking,
            PresentationPayload::CardServiceNotification(notification),
            created_at,
            Some(sim_tick),
        )
    }

    pub(crate) fn enqueue_defense_intro(
        &mut self,
        id: PresentationId,
        stage: usize,
        monster_kind: MonsterKind,
        created_at: PresentationInstant,
        sim_tick: SimTick,
    ) -> bool {
        self.enqueue_with_id(PresentationRequest {
            id,
            source: PresentationSource::CoreEvent,
            policy: PresentationPolicy::Blocking,
            payload: PresentationPayload::DefenseIntro {
                stage,
                monster_kind,
                boss: !monster_kind.is_normal_monster(),
            },
            created_at,
            sim_tick: Some(sim_tick),
        })
    }

    pub(crate) fn advance(&mut self, presentation_instant: PresentationInstant) {
        self.last_presentation_instant = Some(presentation_instant);
        if self
            .active
            .as_ref()
            .is_some_and(|active| self.is_finished(active, presentation_instant))
        {
            self.finish();
        }

        if self.active.is_none() {
            let Some(request) = self.pending.first().cloned() else {
                return;
            };
            self.pending.remove(0);
            let card_playback = match &request.payload {
                PresentationPayload::CardServiceNotification(notification) => {
                    Some(CardServiceNotificationPlayback::from_notification(
                        notification.clone(),
                        presentation_instant,
                    ))
                }
                PresentationPayload::DefenseIntro { .. } => None,
            };
            self.active = Some(ActivePresentation {
                request,
                started_at: presentation_instant,
                card_playback,
            });
        }
    }

    pub(crate) fn clear(&mut self) {
        self.pending.clear();
        self.active = None;
        self.completed.clear();
        self.last_presentation_instant = None;
    }

    pub(crate) fn finish(&mut self) {
        if let Some(active) = self.active.take() {
            self.remember_completed(active.request.id);
        }
    }

    pub(crate) fn is_blocking(&self) -> bool {
        self.active
            .as_ref()
            .is_some_and(|active| active.request.policy == PresentationPolicy::Blocking)
            || self
                .pending
                .iter()
                .any(|request| request.policy == PresentationPolicy::Blocking)
    }

    pub(crate) fn active_card_notification(&self) -> Option<&CardServiceNotificationPlayback> {
        self.active.as_ref()?.card_playback.as_ref()
    }

    pub(crate) fn active_defense_intro(&self) -> Option<ActiveDefenseIntro> {
        let active = self.active.as_ref()?;
        let PresentationPayload::DefenseIntro {
            stage,
            monster_kind,
            boss,
        } = &active.request.payload
        else {
            return None;
        };
        Some(ActiveDefenseIntro {
            stage: *stage,
            monster_kind: *monster_kind,
            boss: *boss,
            started_at: active.started_at,
        })
    }

    #[cfg(test)]
    pub(crate) fn pending_len(&self) -> usize {
        self.pending.len()
    }

    #[cfg(test)]
    pub(crate) fn active_id(&self) -> Option<PresentationId> {
        self.active.as_ref().map(|active| active.request.id)
    }

    #[cfg(test)]
    pub(crate) fn serial(&self) -> u64 {
        self.serial
    }

    fn allocate_id(&mut self) -> PresentationId {
        self.serial = self.serial.wrapping_add(1);
        PresentationId::from_serial(self.serial)
    }

    fn contains_id(&self, id: PresentationId) -> bool {
        self.active
            .as_ref()
            .is_some_and(|active| active.request.id == id)
            || self.pending.iter().any(|request| request.id == id)
            || self.completed.contains(&id)
    }

    fn remember_completed(&mut self, id: PresentationId) {
        self.completed.push(id);
    }

    fn is_finished(
        &self,
        active: &ActivePresentation,
        presentation_instant: PresentationInstant,
    ) -> bool {
        let duration = match &active.request.payload {
            PresentationPayload::CardServiceNotification(_) => CARD_NOTIFICATION_DURATION_SECS,
            PresentationPayload::DefenseIntro { .. } => DEFENSE_INTRO_DURATION_SECS,
        };
        presentation_instant
            .delta_since(active.started_at)
            .as_secs_f32()
            >= duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::card_notification::CardServiceNotification;
    fn instant(seconds: i64) -> PresentationInstant {
        PresentationInstant::from_namui(Instant::new(Duration::from_secs(seconds)))
    }

    fn card_request(id: PresentationId, created_at: PresentationInstant) -> PresentationRequest {
        PresentationRequest {
            id,
            source: PresentationSource::PendingActionEffect,
            policy: PresentationPolicy::Blocking,
            payload: PresentationPayload::CardServiceNotification(
                CardServiceNotification::default(),
            ),
            created_at,
            sim_tick: None,
        }
    }

    #[test]
    fn starts_fifo_and_advances_only_from_the_root() {
        let mut director = PresentationDirector::default();
        let first = PresentationId::from_serial(10);
        let second = PresentationId::from_serial(11);
        assert!(director.enqueue_with_id(card_request(first, instant(0))));
        assert!(director.enqueue_with_id(card_request(second, instant(0))));

        director.advance(instant(0));
        assert_eq!(director.active_id(), Some(first));
        assert_eq!(director.pending_len(), 1);
        assert!(director.is_blocking());

        director.advance(instant(3));
        assert_eq!(director.active_id(), Some(second));
        assert_eq!(director.pending_len(), 0);
    }

    #[test]
    fn duplicate_ids_are_ignored_even_after_completion() {
        let mut director = PresentationDirector::default();
        let request = card_request(PresentationId::from_serial(7), instant(0));
        assert!(director.enqueue_with_id(request.clone()));
        assert!(!director.enqueue_with_id(request.clone()));
        director.advance(instant(0));
        director.advance(instant(3));
        assert!(!director.enqueue_with_id(request));
        assert_eq!(director.serial(), 0);
    }

    #[test]
    fn clearing_drops_active_pending_and_completion_history() {
        let mut director = PresentationDirector::default();
        let request = card_request(PresentationId::from_serial(7), instant(0));
        assert!(director.enqueue_with_id(request.clone()));
        director.advance(instant(0));
        director.advance(instant(3));
        director.clear();
        assert!(director.enqueue_with_id(request));
        assert!(director.is_blocking());
    }

    #[test]
    fn defense_and_card_requests_never_overlap() {
        let mut director = PresentationDirector::default();
        assert!(director.enqueue_with_id(card_request(PresentationId::from_serial(1), instant(0))));
        assert!(director.enqueue_defense_intro(
            PresentationId::from_serial(2),
            3,
            MonsterKind::Boss01,
            instant(0),
            SimTick::from_ticks(1),
        ));

        director.advance(instant(0));
        assert!(director.active_card_notification().is_some());
        assert!(director.active_defense_intro().is_none());
        director.advance(instant(3));
        assert!(director.active_card_notification().is_none());
        assert!(director.active_defense_intro().is_some());
    }
}
