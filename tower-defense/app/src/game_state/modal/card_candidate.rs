use crate::card::{Card, CardId, RenderCard};
use crate::game_state::{HeadedPlayerCommand, UserModal, mutate_headed_game, use_game_state};
use crate::hand::xy_with_spring;
use crate::thumbnail::{ThumbnailRenderOptions, render_thumbnail};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use namui::*;
use namui_prebuilt::simple_rect;
use std::sync::Arc;

const PADDING: Px = px(36.0);
const CARD_GAP: Px = px(16.0);
const CARD_MAX_WIDTH: Px = px(180.0);
const CARD_HEIGHT_RATIO: f32 = 1.35;
const CARD_SERVICE_THUMBNAIL_SIZE: Px = px(72.0);

#[derive(Debug, Clone, State)]
pub struct CardCandidateModal {
    pub card_service: crate::game_state::card_service::CardService,
    pub candidate_card_ids: Vec<CardId>,
}

impl Component for CardCandidateModal {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            card_service,
            candidate_card_ids,
        } = self;
        let game_state = use_game_state(ctx);
        let screen_wh = screen::size().into_type::<Px>();
        let deck = ctx.track_eq(&game_state.state().presentation_deck_snapshot());
        let (hovered_card_id, set_hovered_card_id) = ctx.state::<Option<CardId>>(|| None);
        let cards = candidate_card_ids
            .iter()
            .filter_map(|card_id| deck.get_card(*card_id))
            .collect::<Vec<_>>();

        let card_width =
            ((screen_wh.width - PADDING * 2.0 - CARD_GAP * 2.0) / 3.0).min(CARD_MAX_WIDTH);
        let card_wh = Wh::new(card_width, card_width * CARD_HEIGHT_RATIO);
        let total_width =
            card_wh.width * cards.len() as f32 + CARD_GAP * (cards.len().saturating_sub(1) as f32);
        let card_start_x = (screen_wh.width - total_width) * 0.5;
        let card_y = (screen_wh.height - card_wh.height) * 0.5;

        ctx.translate((PADDING, PADDING)).add(CardServiceThumbnail {
            card_service: &card_service,
        });

        let candidate_card_ids_for_click = candidate_card_ids.clone();
        let on_card_click = Arc::new(move |card_id: CardId| {
            if !candidate_card_ids_for_click.contains(&card_id) {
                return;
            }
            mutate_headed_game(move |game_state| {
                if !matches!(
                    &game_state.opened_modals.user,
                    Some(UserModal::CardCandidate(modal))
                        if modal.candidate_card_ids.contains(&card_id)
                ) {
                    return;
                }
                if game_state
                    .apply_player_command(HeadedPlayerCommand::ConfirmCardServiceSelection {
                        selected_card_ids: vec![vec![card_id.raw() as u64]],
                    })
                    .is_ok()
                {
                    game_state.opened_modals.user = None;
                    game_state.consume_core_events(crate::PresentationInstant::capture());
                }
            });
        });

        for (index, card) in cards.iter().enumerate() {
            ctx.add_with_key(
                card.id,
                CardCandidateView {
                    card,
                    card_wh,
                    target_xy: Xy::new(
                        card_start_x + (card_wh.width + CARD_GAP) * index as f32,
                        card_y,
                    ),
                    hovered_card_id: *hovered_card_id,
                    set_hovered_card_id: &|card_id| set_hovered_card_id.set(card_id),
                    on_card_click: on_card_click.clone(),
                },
            );
        }

        ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Default))
            .add(
                simple_rect(
                    screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::BLACK.with_alpha(180),
                )
                .attach_event(|event| match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        event.stop_propagation();
                    }
                    _ => {}
                }),
            );
    }
}

struct CardCandidateView<'a> {
    card: &'a Card,
    card_wh: Wh<Px>,
    target_xy: Xy<Px>,
    hovered_card_id: Option<CardId>,
    set_hovered_card_id: &'a dyn Fn(Option<CardId>),
    on_card_click: Arc<dyn Fn(CardId) + Send + Sync>,
}

impl Component for CardCandidateView<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            card,
            card_wh,
            target_xy,
            hovered_card_id,
            set_hovered_card_id,
            on_card_click,
        } = self;
        let card_id = card.id;
        let hovering = hovered_card_id == Some(card_id);
        let target_scale = Xy::single(if hovering { 1.08 } else { 1.0 });
        let animated_xy = xy_with_spring(
            ctx,
            target_xy,
            Xy::new(target_xy.x, target_xy.y + card_wh.height),
        );
        let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(1.0));
        let half_card = card_wh.to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_card)
            .scale(animated_scale)
            .translate(-half_card);
        let set_hovered_card_id_on_enter = set_hovered_card_id;
        let set_hovered_card_id_on_exit = set_hovered_card_id;
        let on_card_click = on_card_click.clone();

        ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
            .compose(|ctx| {
                ctx.add(RenderCard {
                    wh: card_wh,
                    card,
                    selected: false,
                    opacity: 1.0,
                });
                ctx.add(
                    WithHoverArea {
                        component_key: "candidate card hover",
                        component: simple_rect(
                            card_wh,
                            Color::TRANSPARENT,
                            0.px(),
                            Color::TRANSPARENT,
                        ),
                        placement: TooltipPlacement::Above,
                        on_enter: move || {
                            set_hovered_card_id_on_enter(Some(card_id));
                            None
                        },
                        on_exit: move || {
                            if hovered_card_id == Some(card_id) {
                                set_hovered_card_id_on_exit(None);
                            }
                        },
                    }
                    .attach_event(move |event| match event {
                        Event::MouseDown { event }
                            if event.is_local_xy_in()
                                && matches!(event.button, Some(MouseButton::Left)) =>
                        {
                            event.stop_propagation();
                            on_card_click(card_id);
                        }
                        _ => {}
                    }),
                );
            });
    }
}

struct CardServiceThumbnail<'a> {
    card_service: &'a crate::game_state::card_service::CardService,
}

impl Component for CardServiceThumbnail<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { card_service } = self;
        let thumbnail_wh = Wh::single(CARD_SERVICE_THUMBNAIL_SIZE);
        let tooltip_card_service = (*card_service).clone();

        ctx.add(render_thumbnail(
            card_service.thumbnail_source(),
            thumbnail_wh,
            ThumbnailRenderOptions::sticker(crate::thumbnail::STICKER_THUMBNAIL_STROKE, true, 1.0),
        ));
        ctx.add(WithHoverArea {
            component_key: "candidate card service tooltip",
            component: simple_rect(thumbnail_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT),
            placement: TooltipPlacement::Above,
            on_enter: move || Some(TooltipContent::CardService(tooltip_card_service.clone())),
            on_exit: || {},
        });
    }
}
