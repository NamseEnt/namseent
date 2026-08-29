use crate::thumbnail::{ThumbnailRenderOptions, render_thumbnail, render_thumbnail_overlays};
use crate::{
    PresentationInstant,
    animation::xy_with_spring,
    card::Card,
    config::GameConfig,
    game_state::tower_selection::get_highest_tower_template,
    game_state::{
        upgrade::{SelectedTowerContext, UpgradePresentation},
        use_game_state,
    },
    hand::{Hand, HandItem, HandSlotId},
    tooltip::WithHoverArea,
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
        let hand = game_state.state().presentation_hand_snapshot();
        let upgrade_state = game_state.state().presentation_upgrade_state_snapshot();
        let selected_slot_ids = ctx.track_eq(&hand.selected_slot_ids());
        let active_slot_ids = ctx.track_eq(&hand.active_slot_ids());
        let active_tower_context = ctx.track_eq(&get_active_tower_context(
            &game_state,
            &hand,
            &upgrade_state,
            &selected_slot_ids,
            &active_slot_ids,
        ));

        let upgrades = game_state.state().presentation_upgrade_entries_snapshot();
        let mut active_upgrades = upgrades
            .iter()
            .filter(|entry| !entry.is_exiting())
            .collect::<Vec<_>>();
        if active_tower_context.is_some() {
            active_upgrades.sort_by_key(|entry| {
                !active_tower_context
                    .as_ref()
                    .is_some_and(|context| is_upgrade_applicable(&entry.upgrade.upgrade, &context))
            });
        }

        let mut upgrade_infos = active_upgrades
            .iter()
            .enumerate()
            .map(|(order, entry)| {
                let is_applicable = active_tower_context
                    .as_ref()
                    .is_some_and(|context| is_upgrade_applicable(&entry.upgrade.upgrade, &context));
                (
                    entry.upgrade.id.0 as u128,
                    entry.upgrade.upgrade,
                    is_applicable,
                    false,
                    order,
                )
            })
            .collect::<Vec<_>>();
        upgrade_infos.extend(
            upgrades
                .iter()
                .filter(|entry| entry.is_exiting())
                .map(|entry| {
                    (
                        entry.upgrade.id.0 as u128,
                        entry.upgrade.upgrade,
                        false,
                        true,
                        entry.order,
                    )
                }),
        );

        let scroll_view = |wh: Wh<Px>, ctx: ComposeCtx| {
            let item_offset = ITEM_SIZE + ITEM_GAP;
            let total_height = item_offset * upgrade_infos.len() as f32;

            ctx.add(AutoScrollViewWithCtx {
                wh,
                scroll_bar_width: PADDING,
                content: |ctx| {
                    for (upgrade_id, upgrade_kind, is_applicable, exiting, order) in
                        upgrade_infos.iter().cloned()
                    {
                        let target_xy = Xy::new(0.px(), item_offset * order as f32);

                        ctx.add_with_key(
                            upgrade_id,
                            UpgradeThumbnailItem {
                                wh: Wh::new(ITEM_SIZE, ITEM_SIZE),
                                upgrade_kind,
                                is_applicable,
                                exiting,
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
    game_state: &crate::headed_game::HeadedGame,
    hand: &Hand<HandItem>,
    upgrade_state: &crate::game_state::upgrade::UpgradeState,
    selected_slot_ids: &[HandSlotId],
    active_slot_ids: &[HandSlotId],
) -> Option<SelectedTowerContext> {
    if let Some(selected_tower_id) = game_state.ui_state().selected_tower_id
        && let Some(tower) = game_state
            .raw_core_state()
            .towers()
            .iter()
            .find(|tower| tower.id == Some(selected_tower_id.raw()))
        && let Some(template) =
            crate::game_state::tower::TowerTemplate::from_core_state(tower.template.clone())
    {
        return Some(SelectedTowerContext {
            tower_id: crate::game_state::upgrade::SelectedTowerId::Placed(selected_tower_id),
            kind: template.kind,
            suit: template.suit,
            rank: template.rank,
            rerolled_count: Some(template.rerolled_count),
        });
    }

    let slot_ids = if !selected_slot_ids.is_empty() {
        selected_slot_ids
    } else {
        active_slot_ids
    };

    if let Some(template) = hand
        .get_items(slot_ids)
        .find_map(|item| item.as_tower().cloned())
    {
        return Some(SelectedTowerContext::from_template(
            &template,
            Some(game_state.raw_core_state().progress().rerolled_count),
        ));
    }

    let cards = hand
        .get_items(slot_ids)
        .filter_map(|item| item.as_card().copied())
        .collect::<Vec<Card>>();

    if cards.is_empty() {
        return None;
    }

    Some(SelectedTowerContext::from_template(
        &get_highest_tower_template(
            &cards,
            upgrade_state,
            game_state.raw_core_state().progress().rerolled_count,
            &GameConfig::from_core_state(game_state.raw_core_state().config().clone())
                .expect("raw game config must be restorable for upgrade presentation"),
        ),
        Some(game_state.raw_core_state().progress().rerolled_count),
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
    exiting: bool,
    target_xy: Xy<Px>,
}

impl Component for UpgradeThumbnailItem {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade_kind,
            is_applicable,
            exiting,
            target_xy,
        } = self;

        let game_state = use_game_state(ctx);
        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<PresentationInstant>);

        let animation_target = if exiting {
            Xy::new(target_xy.x, target_xy.y + ITEM_SIZE)
        } else {
            target_xy
        };
        let animated_xy = xy_with_spring(
            ctx,
            animation_target,
            Xy::new(target_xy.x, target_xy.y + ITEM_SIZE),
        );
        let animated_scale = xy_with_spring(
            ctx,
            if exiting {
                Xy::single(0.0)
            } else {
                Xy::single(1.0)
            },
            Xy::single(0.0),
        );
        let half_item = wh.to_xy() * 0.5;
        let ctx = ctx
            .translate(animated_xy)
            .translate(half_item)
            .scale(animated_scale)
            .translate(-half_item);

        let should_wobble = *hovering || is_applicable;
        if should_wobble && (*hover_start).is_none() {
            set_hover_start.set(Some(PresentationInstant::capture()));
        }
        if !should_wobble {
            set_hover_start.set(None);
        }

        let hover_rotation = if let Some(start) = *hover_start {
            ((PresentationInstant::capture() - start).as_secs_f32() * 25.0).sin() * 3.0
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

            let overlays = upgrade_kind.thumbnail_overlays(&game_state);
            if !overlays.is_empty() {
                ctx.add(render_thumbnail_overlays(&overlays, thumbnail_wh));
            }
            ctx.add(render_thumbnail(
                upgrade_kind.thumbnail_source(),
                thumbnail_wh,
                ThumbnailRenderOptions::sticker(
                    crate::thumbnail::STICKER_THUMBNAIL_STROKE,
                    true,
                    1.0,
                ),
            ));
        });

        if !exiting {
            ctx.add(WithHoverArea {
                component_key: "upgrade tooltip",
                component: simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT),
                placement: crate::tooltip::TooltipPlacement::RightOf,
                on_enter: || {
                    set_hovering.set(true);
                    Some(crate::tooltip::TooltipContent::Upgrade(upgrade_kind))
                },
                on_exit: || {
                    set_hovering.set(false);
                },
            });
        }
    }
}
