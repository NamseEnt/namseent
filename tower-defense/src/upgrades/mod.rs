use crate::{
    animation::{with_spring, xy_with_spring},
    card::{Card, Rank, Suit},
    flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template,
    game_state::{
        tower::{Tower, TowerKind, TowerTemplate},
        use_game_state,
    },
    hand::HandSlotId,
    l10n::Locale,
    palette,
    theme::{
        paper_container::{
            ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
        },
        typography::{FontSize, memoized_text},
    },
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, simple_rect, table};

const PADDING: Px = px(8.);
const ITEM_SIZE: Px = px(64.);
const ITEM_GAP: Px = px(12.);
const ITEM_MARGIN: Px = px(6.);

mod tooltip {
    use namui::*;
    pub const PADDING: Px = px(12.0);
    pub const MAX_WIDTH: Px = px(320.0);
    pub const ARROW_WIDTH: Px = px(10.0);
    pub const ARROW_HEIGHT: Px = px(18.0);
    pub const OFFSET_X: Px = px(4.0);
}

pub struct Upgrades {
    pub wh: Wh<Px>,
}

impl Component for Upgrades {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();
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
                .map(|upgrade| upgrade.kind)
                .map(|upgrade_kind| {
                    let is_applicable = active_tower_context
                        .as_ref()
                        .is_some_and(|context| is_upgrade_applicable(&upgrade_kind, &context));
                    (upgrade_kind, is_applicable)
                })
                .collect::<Vec<_>>();

            if active_tower_context.is_some() {
                upgrade_infos.sort_by(|(_, a), (_, b)| b.cmp(a));
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
                    for (index, (upgrade_kind, is_applicable)) in
                        upgrade_infos.iter().cloned().enumerate()
                    {
                        let key = upgrade_kind_key(upgrade_kind);
                        let target_xy = Xy::new(0.px(), item_offset * index as f32);

                        ctx.add_with_key(
                            key,
                            UpgradeThumbnailItem {
                                wh: Wh::new(ITEM_SIZE, ITEM_SIZE),
                                upgrade_kind,
                                locale,
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

#[derive(Clone, Copy, PartialEq, Eq, State)]
struct SelectedTowerContext {
    kind: TowerKind,
    suit: Suit,
    rank: Rank,
    rerolled_count: Option<usize>,
}

impl SelectedTowerContext {
    fn from_tower(tower: &Tower) -> Self {
        Self {
            kind: tower.kind,
            suit: tower.suit,
            rank: tower.rank,
            rerolled_count: None,
        }
    }

    fn from_template(template: &TowerTemplate, rerolled_count: Option<usize>) -> Self {
        Self {
            kind: template.kind,
            suit: template.suit,
            rank: template.rank,
            rerolled_count,
        }
    }

    fn is_low_card_tower(&self) -> bool {
        self.kind.is_low_card_tower()
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
    upgrade_kind: &crate::game_state::upgrade::UpgradeKind,
    context: &SelectedTowerContext,
) -> bool {
    match upgrade_kind {
        crate::game_state::upgrade::UpgradeKind::CainSword { .. } => context.suit == Suit::Diamonds,
        crate::game_state::upgrade::UpgradeKind::LongSword { .. } => context.suit == Suit::Spades,
        crate::game_state::upgrade::UpgradeKind::Mace { .. } => context.suit == Suit::Hearts,
        crate::game_state::upgrade::UpgradeKind::ClubSword { .. } => context.suit == Suit::Clubs,
        crate::game_state::upgrade::UpgradeKind::SingleChopstick { .. } => !context.rank.is_even(),
        crate::game_state::upgrade::UpgradeKind::PairChopsticks { .. } => context.rank.is_even(),
        crate::game_state::upgrade::UpgradeKind::FountainPen { .. } => !context.rank.is_face(),
        crate::game_state::upgrade::UpgradeKind::Brush { .. } => context.rank.is_face(),
        crate::game_state::upgrade::UpgradeKind::Tricycle { .. } => context.is_low_card_tower(),
        crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. } => {
            context.rerolled_count == Some(0)
        }
        crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => {
            context.rerolled_count.is_some_and(|count| count > 0)
        }
        _ => false,
    }
}

fn upgrade_kind_key(upgrade_kind: crate::game_state::upgrade::UpgradeKind) -> u128 {
    match upgrade_kind {
        crate::game_state::upgrade::UpgradeKind::Cat => 0,
        crate::game_state::upgrade::UpgradeKind::CainSword { .. } => 1,
        crate::game_state::upgrade::UpgradeKind::LongSword { .. } => 2,
        crate::game_state::upgrade::UpgradeKind::Mace { .. } => 3,
        crate::game_state::upgrade::UpgradeKind::ClubSword { .. } => 4,
        crate::game_state::upgrade::UpgradeKind::Backpack => 5,
        crate::game_state::upgrade::UpgradeKind::DiceBundle => 6,
        crate::game_state::upgrade::UpgradeKind::Tricycle { .. } => 7,
        crate::game_state::upgrade::UpgradeKind::EnergyDrink => 8,
        crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. } => 9,
        crate::game_state::upgrade::UpgradeKind::SingleChopstick { .. } => 10,
        crate::game_state::upgrade::UpgradeKind::PairChopsticks { .. } => 11,
        crate::game_state::upgrade::UpgradeKind::FountainPen { .. } => 12,
        crate::game_state::upgrade::UpgradeKind::Brush { .. } => 13,
        crate::game_state::upgrade::UpgradeKind::FourLeafClover => 14,
        crate::game_state::upgrade::UpgradeKind::Rabbit => 15,
        crate::game_state::upgrade::UpgradeKind::BlackWhite => 16,
        crate::game_state::upgrade::UpgradeKind::Eraser => 17,
        crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => 18,
    }
}

struct UpgradeThumbnailItem {
    wh: Wh<Px>,
    upgrade_kind: crate::game_state::upgrade::UpgradeKind,
    locale: Locale,
    is_applicable: bool,
    target_xy: Xy<Px>,
}

impl Component for UpgradeThumbnailItem {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            upgrade_kind,
            locale,
            is_applicable,
            target_xy,
        } = self;

        let (hovering, set_hovering) = ctx.state(|| false);
        let (hover_start, set_hover_start) = ctx.state(|| None::<Instant>);
        let tooltip_scale = with_spring(
            ctx,
            if *hovering { 1.0 } else { 0.0 },
            0.0,
            |v| v * v,
            || 0.0,
        );

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

        ctx.compose(|ctx| {
            if tooltip_scale > 0.01 {
                let tooltip = ctx.ghost_add(
                    "upgrade-tooltip",
                    UpgradeTooltip {
                        upgrade_kind,
                        locale,
                    },
                );
                if let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) {
                    let y = (wh.height - tooltip_wh.height) / 2.0;
                    let pivot = Xy::new(0.px(), tooltip_wh.height / 2.0);
                    let base = Xy::new(
                        wh.width
                            + tooltip::OFFSET_X
                            + tooltip::ARROW_WIDTH
                            + ITEM_MARGIN * 2.0
                            + PADDING,
                        y,
                    );
                    ctx.translate(base + pivot)
                        .scale(Xy::new(tooltip_scale, tooltip_scale))
                        .translate(Xy::new(-pivot.x, -pivot.y))
                        .on_top()
                        .add(tooltip);
                }
            }
        });

        let ctx = ctx.translate(Xy::new(ITEM_MARGIN, ITEM_MARGIN));
        let thumbnail_wh = Wh::new(ITEM_SIZE - PADDING * 2.0, ITEM_SIZE - PADDING * 2.0);

        ctx.translate(Xy::single(PADDING)).compose(|ctx| {
            let pivot = Xy::new(thumbnail_wh.width * 0.5, thumbnail_wh.height * 0.5);
            ctx.translate(pivot)
                .rotate(hover_rotation.deg())
                .translate(Xy::new(-pivot.x, -pivot.y))
                .add(upgrade_kind.thumbnail(thumbnail_wh));
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
                    set_hovering.set(true);
                } else {
                    set_hovering.set(false);
                }
            }),
        );
    }
}

struct UpgradeTooltip {
    upgrade_kind: crate::game_state::upgrade::UpgradeKind,
    locale: Locale,
}

impl Component for UpgradeTooltip {
    fn render(self, ctx: &RenderCtx) {
        let UpgradeTooltip {
            upgrade_kind,
            locale,
        } = self;

        let max_width = tooltip::MAX_WIDTH;
        let text_max = max_width - (tooltip::PADDING * 2.0);

        let content = ctx.ghost_compose("tooltip-content", |ctx| {
            table::vertical([table::fit(table::FitAlign::LeftTop, |compose_ctx| {
                compose_ctx.add(memoized_text(
                    (&upgrade_kind, &text_max, &locale.language),
                    |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Large)
                            .max_width(text_max)
                            .l10n(upgrade_kind.description_text(), &locale)
                            .render_left_top()
                    },
                ));
            })])(Wh::new(text_max, f32::MAX.px()), ctx);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        let container_wh = content_wh + Wh::single(tooltip::PADDING * 2.0);

        ctx.translate((tooltip::PADDING, tooltip::PADDING))
            .add(content);
        Self::render_background(ctx, container_wh);
    }
}

impl UpgradeTooltip {
    fn render_background(ctx: &RenderCtx, wh: Wh<Px>) {
        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER,
            shadow: true,
            arrow: Some(PaperArrow {
                side: ArrowSide::Left,
                width: tooltip::ARROW_WIDTH,
                height: tooltip::ARROW_HEIGHT,
                offset: wh.height / 2.0,
            }),
        });
    }
}
