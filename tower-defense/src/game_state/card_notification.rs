use std::sync::atomic::AtomicUsize;

use crate::animation::{with_spring, xy_with_spring};
use crate::card::Card;
use crate::card::RenderCard;
use crate::game_state::use_game_state;
use namui::*;
use rand::{Rng, thread_rng};

const CARD_WIDTH: Px = px(120.0);
const CARD_HEIGHT: Px = px(162.0);
const TOTAL_DURATION_SECS: f32 = 3.0;
const CARD_LIFETIME_SECS: f32 = 2.0;
const STAGGER_WINDOW_SECS: f32 = 1.0;
const EXIT_SCREEN_FACTOR: f32 = 0.65;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
enum CardServiceNotificationEntry {
    Added { card: Card },
    Removed { card: Card },
    Enhanced { from: Card, to: Card },
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct CardServiceNotification {
    entries: Vec<CardServiceNotificationEntry>,
}
impl CardServiceNotification {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn added(&mut self, card: Card) -> &mut Self {
        self.entries
            .push(CardServiceNotificationEntry::Added { card });
        self
    }

    pub fn removed(&mut self, card: Card) -> &mut Self {
        self.entries
            .push(CardServiceNotificationEntry::Removed { card });
        self
    }

    pub fn enhanced(&mut self, from: Card, to: Card) -> &mut Self {
        self.entries
            .push(CardServiceNotificationEntry::Enhanced { from, to });
        self
    }
}

type CardServiceNotificationPlaybackEntryId = usize;

#[derive(Debug, Clone, PartialEq, State)]
struct CardServiceNotificationPlaybackEntry {
    id: CardServiceNotificationPlaybackEntryId,
    notification_entry: CardServiceNotificationEntry,
    position_offset: Xy<f32>,
}
impl CardServiceNotificationPlaybackEntry {
    fn new(notification_entry: CardServiceNotificationEntry) -> Self {
        let mut rng = thread_rng();
        static ENTRY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ENTRY_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id,
            notification_entry,
            position_offset: Xy::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct CardServiceNotificationPlayback {
    entries: Vec<CardServiceNotificationPlaybackEntry>,
    start_time: Instant,
}

#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct CardServiceNotificationState {
    pub current: Option<CardServiceNotificationPlayback>,
    pub queue: Vec<CardServiceNotification>,
}

impl CardServiceNotificationState {
    pub fn enqueue(&mut self, now: Instant, notification: CardServiceNotification) {
        if self.current.is_none() {
            self.current = Some(CardServiceNotificationPlayback {
                entries: notification
                    .entries
                    .into_iter()
                    .map(CardServiceNotificationPlaybackEntry::new)
                    .collect(),
                start_time: now,
            });
        } else {
            self.queue.push(notification);
        }
    }

    pub fn advance(&mut self, now: Instant) {
        match self.current.as_ref() {
            Some(current) if (now - current.start_time).as_secs_f32() < TOTAL_DURATION_SECS => {
                return;
            }
            _ => {}
        }

        if let Some(notification) = self.queue.first().cloned() {
            self.queue.remove(0);
            self.current = Some(CardServiceNotificationPlayback {
                entries: notification
                    .entries
                    .into_iter()
                    .map(CardServiceNotificationPlaybackEntry::new)
                    .collect(),
                start_time: now,
            });
        } else {
            self.current = None;
        }
    }
}

pub struct CardServiceNotificationLayer;

impl Component for CardServiceNotificationLayer {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let now = game_state.now();
        let screen_wh = screen::size().map(IntPx::into_px);

        let should_advance = match game_state.card_service_notifications.current.as_ref() {
            Some(current) => (now - current.start_time).as_secs_f32() >= TOTAL_DURATION_SECS,
            None => !game_state.card_service_notifications.queue.is_empty(),
        };
        if should_advance {
            ctx.effect("advance card service notifications", || {
                crate::game_state::mutate_game_state(move |game_state| {
                    game_state.card_service_notifications.advance(now);
                });
            });
        }

        let Some(playback) = game_state.card_service_notifications.current.as_ref() else {
            return;
        };

        let move_distance = screen_wh * EXIT_SCREEN_FACTOR;
        let delay_per_entry = STAGGER_WINDOW_SECS / playback.entries.len() as f32;
        let card_wh = Wh::new(CARD_WIDTH, CARD_HEIGHT);
        let one_third_screen_wh = (screen_wh / 3.0).to_xy();
        for (index, entry) in playback.entries.iter().enumerate() {
            let elapsed = (now - playback.start_time).as_secs_f32();
            let entry_start_time = index as f32 * delay_per_entry;
            let entry_end_time = entry_start_time + CARD_LIFETIME_SECS;
            let progress = ((elapsed - entry_start_time) / (entry_end_time - entry_start_time))
                .clamp(0.0, 1.0);
            if progress <= 0.0 || progress >= 1.0 {
                continue;
            }

            let offset_xy = one_third_screen_wh + (one_third_screen_wh * entry.position_offset);
            ctx.translate(offset_xy).add_with_key(
                entry.id,
                CardNotificationCard {
                    kind: entry.notification_entry,
                    progress,
                    card_wh,
                    move_distance,
                },
            );
        }
    }
}

#[derive(Clone, Copy)]
struct CardNotificationCard {
    kind: CardServiceNotificationEntry,
    progress: f32,
    card_wh: Wh<Px>,
    move_distance: Wh<Px>,
}

impl Component for CardNotificationCard {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            kind,
            progress,
            card_wh,
            move_distance,
        } = self;

        let animated_xy = xy_with_spring(
            ctx,
            if progress < 0.6 {
                Xy::single(0.px())
            } else {
                match kind {
                    CardServiceNotificationEntry::Added { .. } => {
                        Xy::new(move_distance.width, 0.px())
                    }
                    CardServiceNotificationEntry::Removed { .. } => {
                        Xy::new(-move_distance.width, 0.px())
                    }
                    CardServiceNotificationEntry::Enhanced { .. } => {
                        Xy::new(0.px(), -move_distance.height)
                    }
                }
            },
            Xy::single(0.px()),
        );
        let animated_scale = with_spring(
            ctx,
            if progress < 0.6 { 1.0 } else { 0.0 },
            0.0,
            |x| x * x,
            || 0.0,
        );

        let half_card = (card_wh * 0.5).to_xy();
        ctx.translate(animated_xy)
            .scale(Xy::single(animated_scale))
            .translate(-half_card)
            .compose(|ctx| match kind {
                CardServiceNotificationEntry::Added { card } => {
                    ctx.add(RenderCard {
                        wh: card_wh,
                        card: &card,
                        selected: false,
                    });
                }
                CardServiceNotificationEntry::Removed { card } => {
                    ctx.add(RenderCard {
                        wh: card_wh,
                        card: &card,
                        selected: false,
                    });
                }
                CardServiceNotificationEntry::Enhanced { from, to } => {
                    ctx.add(RenderCard {
                        wh: card_wh,
                        card: &from,
                        selected: false,
                    });
                }
            });
    }
}
