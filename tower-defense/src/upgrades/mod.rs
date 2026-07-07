use crate::{
    animation::xy_with_spring, card::Card, flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template, game_state::{
        upgrade::{SelectedTowerContext, UpgradeBehavior},
        use_game_state,
    }, hand::HandSlotId,
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const PADDING: Px = px(8.);
const ITEM_SIZE: Px = px(64.);
const ITEM_GAP: Px = px(12.);
const ITEM_MARGIN: Px = px(6.);

pub struct Upgrades {
    pub wh: Wh<Px>,
}

impl Component for Upgrades {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let game_state = use_game_state(ctx);
        let selected_slot_ids = ctx.track_eq(&game_state.hand.selected_slot_ids());
        let active_slot_ids = ctx.track_eq(&game_state.hand.active_slot_ids());
        let active_tower_context = ctx.track_eq(&get_active_tower_context(
            &game_state,
            &selected_slot_ids,
            &active_slot_ids,
        ));

        let upgrades = &game_state.upgrade_state.upgrades;
        let upgrade_revision = ctx.track_eq(&game_state.upgrade_state.revision);
        let upgrade_infos = ctx.memo(move || {
            upgrade_revision.record_as_used();
            active_tower_context.record_as_used();
            let mut upgrade_infos = upgrades
                .iter()
                .map(|upgrade| {
                    let is_applicable = active_tower_context
                        .as_ref()
                        .is_some_and(|context| is_upgrade_applicable(&upgrade.upgrade, &context));
                    (upgrade.id.0 as u128, upgrade.upgrade, is_applicable)
                })
                .collect::<Vec<_>>();

            if active_tower_context.is_some() {
                upgrade_infos.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));
            }

            upgrade_infos
        });

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            let item_offset = ITEM_SIZE + ITEM_GAP;
            let total_height = item_offset * upgrade_infos.len() as f32;

            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |ctx| {
                    for (index, (upgrade_id, upgrade_kind, is_applicable)) in
                        upgrade_infos.iter().cloned().enumerate()
                    {
                        let target_xy = Xy::new(0.px(), item_offset * index as f32);

                        ctx.add_with_key(
                            upgrade_id,
                            UpgradeThumbnailItem {
                                wh: Wh::new(ITEM_SIZE, ITEM_SIZE),
                                upgrade_kind,
                                is_applicable,
                                target_xy,
                            },
                        );
                    }

                    ctx.add(simple_rect(
                        Wh::new(wh.width, total_height),
                        Color::TRANSPARENT,
                        0.px(),
                        Color::TRANSPARENT,
                    ));
                },
            });
        };

        ctx.compose(|ctx| {
            table::horizontal([table::fixed_no_clip(
                wh.width,
                table::padding_no_clip(PADDING, scroll_view),
            )])(wh, ctx);
        });
    }
}

fn get_active_tower_context(
    game_state: &crate::game_state::GameState,
    selected_slot_ids: &[HandSlotId],
    active_slot_ids: &[HandSlotId],
) -> Option<SelectedTowerContext> {
    if let Some(selected_tower_id) = game_state.ui_state.selected_tower_id
        && let Some(tower) = game_state
            .towers
            .iter()
            .find(|tower| tower.id() == selected_tower_id)
    {
        return Some(SelectedTowerContext::from_tower(tower));
    }

    let slot_ids = if !selected_slot_ids.is_empty() {
        selected_slot_ids
    } else {
        active_slot_ids
    };

    if let Some(template) = game_state
        .hand
        .get_items(slot_ids)
        .find_map(|item| item.as_tower().cloned())
    {
        return Some(SelectedTowerContext::from_template(
            &template,
            Some(game_state.rerolled_count),
        ));
    }

    let cards = game_state
        .hand
        .get_items(slot_ids)
        .filter_map(|item| item.as_card().copied())
        .collect::<Vec<Card>>();

    if cards.is_empty() {
        return None;
    }

    Some(SelectedTowerContext::from_template(
        &get_highest_tower_template(
            &cards,
            &game_state.upgrade_state,
            game_state.rerolled_count,
            &game_state.config,
        ),
        Some(game_state.rerolled_count),
    ))
}

fn is_upgrade_applicable(
    upgrade: &crate::game_state::upgrade::Upgrade,
    context: &SelectedTowerContext,
) -> bool {
    upgrade.is_applicable(context)
}

struct UpgradeThumbnailItem {
    wh: Wh<Px>,
    upgrade_kind: crate::game_state::upgrade::Upgrade,
    is_applicable: bool,
    target_xy: Xy<Px>,
}

impl Component for UpgradeThumbnailItem {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade_kind,
            is_applicable,
            target_xy,
        } = self;

        let game_state = use_game_state(ctx);
        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);
        let (tooltip_id, _) = ctx.state(crate::tooltip::TooltipId::new);

        let animated_xy = xy_with_spring(ctx, target_xy, target_xy);
        let ctx = ctx.translate(animated_xy);

        let should_wobble = *hovering || is_applicable;
        if should_wobble && (*hover_start).is_none() {
            set_hover_start.set(Some(Instant::now()));
        }
        if !should_wobble {
            set_hover_start.set(None);
        }

        let hover_rotation = if let Some(start) = *hover_start {
            ((Instant::now() - start).as_secs_f32() * 25.0).sin() * 3.0
        } else {
            0.0
        };

        let ctx = ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN));
        let thumbnail_wh = Wh::new(ITEM_SIZE - PADDING * 2.0, ITEM_SIZE - PADDING * 2.0);

        ctx.translate(Xy::single(PADDING)).compose(|ctx| {
            let pivot = Xy::new(thumbnail_wh.width * 0.5, thumbnail_wh.height * 0.5);
            let ctx = ctx
                .translate(pivot)
                .rotate(hover_rotation.deg())
                .translate(Xy::new(-pivot.x, -pivot.y));

            if let Some(overlay) = upgrade_kind.thumbnail_overlay(thumbnail_wh, &game_state) {
                ctx.add(overlay);
            }
            ctx.add(upgrade_kind.thumbnail(thumbnail_wh, true));
        });

        ctx.add(
            simple_rect(
                Wh::new(ITEM_SIZE, ITEM_SIZE),
                Color::TRANSPARENT,
                0.px(),
                Color::TRANSPARENT,
            )
            .attach_event(move |event| {
                let Event::MouseMove { event } = event else {
                    return;
                };
                if event.is_local_xy_in() {
                    if !*hovering {
                        set_hovering.set(true);
                        let origin = event.global_xy - event.local_xy();
                        crate::tooltip::show_tooltip(
                            *tooltip_id,
                            Rect::from_xy_wh(origin, wh),
                            crate::tooltip::TooltipPlacement::RightOf,
                            crate::tooltip::TooltipContent::Upgrade(upgrade_kind),
                        );
                    }
                } else if *hovering {
                    set_hovering.set(false);
                    crate::tooltip::hide_tooltip(*tooltip_id);
                }
            }),
        );
    }
}
