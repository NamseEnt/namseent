mod hover_area;
mod word;

use crate::animation::with_spring;
use crate::game_state::card_service::{
    CardServiceBehavior, CardServicePurchaseBlockReason, CardServicePurchaseContext,
};
use crate::game_state::item::{Item, ItemBehavior};
use crate::game_state::shop_purchase::ShopPurchaseBlockReason;
use crate::game_state::upgrade::{Upgrade, UpgradeBehavior};
use crate::game_state::use_game_state;
use crate::icon::IconKind;
use crate::l10n::ui::FabTooltipText;
use crate::l10n::word::Word;
use crate::l10n::{self, Locale};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{FontSize, TypographyBuilder, memoized_text};
pub use hover_area::WithHoverArea;
use namui::*;
use namui_prebuilt::table;
use std::sync::atomic::{AtomicU64, Ordering};

const PADDING: Px = px(12.0);
const MAX_WIDTH: Px = px(240.0);
const TITLE_GAP: Px = px(8.0);
const SECTION_GAP: Px = px(8.0);
const ANCHOR_GAP: Px = px(8.0);
const SCREEN_MARGIN: Px = px(8.0);

static NEXT_TOOLTIP_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct TooltipId(u64);

impl TooltipId {
    pub fn new() -> Self {
        Self(NEXT_TOOLTIP_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum TooltipPlacement {
    LeftOf,
    RightOf,
    Above,
    Below,
}

#[derive(Debug, Clone, PartialEq, State)]
pub enum TooltipContent {
    Item(Item),
    Upgrade(Upgrade),
    CardService(crate::game_state::card_service::CardService),
    Shop {
        content: Box<TooltipContent>,
        slot_id: crate::shop::ShopSlotId,
    },
    Reroll {
        health_cost: usize,
    },
    Word(crate::l10n::word::Word),
    Words(Vec<crate::l10n::word::Word>),
    Fab(FabTooltipText),
    Undiscovered,
}

#[derive(Debug, Clone, PartialEq, State)]
struct TooltipRequest {
    id: TooltipId,
    anchor: Rect<Px>,
    placement: TooltipPlacement,
    content: TooltipContent,
}

static TOOLTIP: Atom<Option<TooltipRequest>> = Atom::uninitialized();
static TOOLTIP_DISMISS_REVISION: Atom<u64> = Atom::uninitialized();

pub(crate) fn dismiss_revision<'a, 'rt>(ctx: &'a RenderCtx<'a, 'rt>) -> Sig<'a, u64> {
    ctx.init_atom(&TOOLTIP_DISMISS_REVISION, || 0).0
}

pub fn show_tooltip(
    id: TooltipId,
    anchor: Rect<Px>,
    placement: TooltipPlacement,
    content: TooltipContent,
) {
    TOOLTIP.set(Some(TooltipRequest {
        id,
        anchor,
        placement,
        content,
    }));
}

pub fn hide_tooltip(id: TooltipId) {
    TOOLTIP.mutate(move |current| {
        if current.as_ref().map(|request| request.id) == Some(id) {
            *current = None;
        }
    });
}

pub fn dismiss_all_tooltips() {
    TOOLTIP.set(None);
    TOOLTIP_DISMISS_REVISION.mutate(|revision| {
        *revision = revision.wrapping_add(1);
    });
}

pub struct SectionText<'a> {
    pub key: String,
    pub apply: Box<dyn Fn(&mut TypographyBuilder) + 'a>,
}

pub struct TooltipSection<'a> {
    pub title: Option<SectionText<'a>>,
    pub body: SectionText<'a>,
}

impl TooltipContent {
    pub(crate) fn shop(content: Self, slot_id: crate::shop::ShopSlotId) -> Self {
        Self::Shop {
            content: Box::new(content),
            slot_id,
        }
    }

    fn sections(
        &self,
        locale: Locale,
        game_state: &crate::game_state::GameState,
        purchase_context: &CardServicePurchaseContext,
    ) -> Vec<TooltipSection<'_>> {
        match self {
            TooltipContent::Item(item) => {
                let mut sections = Word::Item.tooltip_sections(locale);
                sections.extend(item.tooltip_sections(locale));
                sections
            }
            TooltipContent::Upgrade(upgrade) => {
                let mut sections = Word::Treasure.tooltip_sections(locale);
                sections.extend(upgrade.tooltip_sections(locale));
                sections
            }
            TooltipContent::CardService(card_service) => {
                let mut sections = Word::CardService.tooltip_sections(locale);
                sections.extend(card_service.tooltip_sections(locale));
                sections
            }
            TooltipContent::Shop { content, slot_id } => {
                let mut sections = content.sections(locale, game_state, purchase_context);
                let status =
                    game_state.shop_purchase_status_with_context(*slot_id, purchase_context);
                if !status.is_available() {
                    sections.extend(shop_purchase_unavailable_sections(status.reasons(), locale));
                }
                sections
            }
            TooltipContent::Reroll { health_cost } => {
                let health_cost = *health_cost;
                vec![TooltipSection {
                    title: None,
                    body: SectionText {
                        key: format!("reroll:{health_cost}"),
                        apply: Box::new(move |builder| {
                            builder.icon(IconKind::Warning).space().l10n(
                                l10n::ui::RerollHealthCostDetailText::Damage(health_cost),
                                &locale,
                            );
                        }),
                    },
                }]
            }
            TooltipContent::Word(word) => {
                let word = *word;
                word.tooltip_sections(locale)
            }
            TooltipContent::Words(words) => words
                .iter()
                .flat_map(|word| word.tooltip_sections(locale))
                .collect(),
            TooltipContent::Fab(text) => {
                let text = *text;
                vec![TooltipSection {
                    title: None,
                    body: SectionText {
                        key: format!("fab:{}", text.key()),
                        apply: Box::new(move |builder| {
                            builder.l10n(text, &locale);
                        }),
                    },
                }]
            }
            TooltipContent::Undiscovered => vec![TooltipSection {
                title: None,
                body: SectionText {
                    key: "encyclopedia:undiscovered".to_string(),
                    apply: Box::new(move |builder| {
                        builder.l10n(l10n::ui::EncyclopediaText::Undiscovered, &locale);
                    }),
                },
            }],
        }
    }
}

fn shop_purchase_unavailable_sections(
    reasons: &[ShopPurchaseBlockReason],
    locale: Locale,
) -> Vec<TooltipSection<'static>> {
    let reason_texts = reasons
        .iter()
        .map(|reason| match reason {
            ShopPurchaseBlockReason::InvalidSlot => {
                l10n::ui::ShopPurchaseBlockReasonText::Unavailable
            }
            ShopPurchaseBlockReason::AlreadyPurchased => {
                l10n::ui::ShopPurchaseBlockReasonText::AlreadyPurchased
            }
            ShopPurchaseBlockReason::NotEnoughGold {
                required,
                available,
            } => l10n::ui::ShopPurchaseBlockReasonText::NotEnoughGold {
                required: *required,
                available: *available,
            },
            ShopPurchaseBlockReason::PurchasesDisabled => {
                l10n::ui::ShopPurchaseBlockReasonText::PurchasesDisabled
            }
            ShopPurchaseBlockReason::CardService(reason) => match reason {
                CardServicePurchaseBlockReason::NoEngravedCard => {
                    l10n::ui::ShopPurchaseBlockReasonText::NoEngravedCard
                }
                CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                    required,
                    available,
                } => l10n::ui::ShopPurchaseBlockReasonText::NotEnoughUnengravedCards {
                    required: *required,
                    available: *available,
                },
            },
        })
        .collect::<Vec<_>>();

    vec![TooltipSection {
        title: Some(SectionText {
            key: "shop_purchase:unavailable:title".to_string(),
            apply: Box::new(move |builder| {
                builder
                    .icon(IconKind::Warning)
                    .space()
                    .l10n(l10n::ui::ShopPurchaseBlockReasonText::Unavailable, &locale);
            }),
        }),
        body: SectionText {
            key: "shop_purchase:unavailable:reasons".to_string(),
            apply: Box::new(move |builder| {
                for (index, reason) in reason_texts.iter().enumerate() {
                    if index > 0 {
                        builder.text("\n");
                    }
                    builder.l10n(*reason, &locale);
                }
            }),
        },
    }]
}

pub struct TooltipLayer;

impl Component for TooltipLayer {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();
        let _dismiss_revision = dismiss_revision(ctx);
        let (request, _) = ctx.init_atom(&TOOLTIP, || None::<TooltipRequest>);
        let modal_open = (
            game_state.opened_modals.user.is_some(),
            game_state.opened_modals.system.is_some(),
        );
        let modal_open = ctx.track_eq(&modal_open);
        ctx.effect("dismiss tooltip when modal opens", || {
            modal_open.record_as_used();
            if modal_open.0 || modal_open.1 {
                dismiss_all_tooltips();
            }
        });
        let deck_revision = ctx.track_eq(&game_state.deck.revision());
        let purchase_context = ctx.memo(|| {
            deck_revision.record_as_used();
            CardServicePurchaseContext::from_game_state(&game_state)
        });

        let (last, set_last) = ctx.state(|| None::<TooltipRequest>);

        let showing = request.is_some();
        if let Some(request) = request.as_ref() {
            let changed = match (*last).as_ref() {
                Some(last) => last != request,
                None => true,
            };
            if changed {
                set_last.set(Some(request.clone()));
            }
        }

        let scale = with_spring(ctx, if showing { 1.0 } else { 0.0 }, 0.0, |v| v * v, || 0.0);
        if scale < 0.01 {
            return;
        }

        // 표시 중이면 현재 요청, 퇴장 애니메이션 중이면 마지막 요청을 그린다.
        let shown = match request.as_ref() {
            Some(request) => Some(request.clone()),
            None => (*last).clone(),
        };
        let Some(request) = shown else {
            return;
        };

        ctx.compose(|ctx| {
            let sections = request
                .content
                .sections(locale, &game_state, &purchase_context);
            let tooltip = ctx.ghost_add("stacked-tooltip", StackedTooltip { sections, locale });
            let Some(tooltip_wh) = tooltip.bounding_box().map(|rect| rect.wh()) else {
                return;
            };

            let pos = compute_position(request.anchor, request.placement, tooltip_wh);
            let pivot = tooltip_wh.to_xy() * 0.5;
            ctx.absolute(pos + pivot)
                .scale(Xy::new(scale, scale))
                .translate(-pivot)
                .add(tooltip);
        });
    }
}

fn compute_position(anchor: Rect<Px>, placement: TooltipPlacement, tooltip_wh: Wh<Px>) -> Xy<Px> {
    let screen = screen::size().into_type::<Px>();
    let w = tooltip_wh.width;
    let h = tooltip_wh.height;
    let center_x = anchor.left() + anchor.width() / 2.0;

    // sts2 방식: 대상 옆(LeftOf/RightOf)에 띄울 때 묶음 상단을 대상 상단에 맞추고 아래로 쌓는다.
    // 위/아래(Above/Below)에 띄울 때는 가로 중앙에 맞춘다.
    let mut pos = match placement {
        TooltipPlacement::LeftOf => Xy::new(anchor.left() - ANCHOR_GAP - w, anchor.top()),
        TooltipPlacement::RightOf => Xy::new(anchor.right() + ANCHOR_GAP, anchor.top()),
        TooltipPlacement::Above => Xy::new(center_x - w / 2.0, anchor.top() - ANCHOR_GAP - h),
        TooltipPlacement::Below => Xy::new(center_x - w / 2.0, anchor.bottom() + ANCHOR_GAP),
    };

    // 선호 방향이 화면 밖으로 나가면 반대편으로 뒤집는다.
    match placement {
        TooltipPlacement::LeftOf if pos.x < SCREEN_MARGIN => {
            pos.x = anchor.right() + ANCHOR_GAP;
        }
        TooltipPlacement::RightOf if pos.x + w > screen.width - SCREEN_MARGIN => {
            pos.x = anchor.left() - ANCHOR_GAP - w;
        }
        TooltipPlacement::Above if pos.y < SCREEN_MARGIN => {
            pos.y = anchor.bottom() + ANCHOR_GAP;
        }
        TooltipPlacement::Below if pos.y + h > screen.height - SCREEN_MARGIN => {
            pos.y = anchor.top() - ANCHOR_GAP - h;
        }
        _ => {}
    }

    // 교차축은 화면 안으로 밀어 넣는다.
    pos.x = clamp_px(pos.x, SCREEN_MARGIN, screen.width - SCREEN_MARGIN - w);
    pos.y = clamp_px(pos.y, SCREEN_MARGIN, screen.height - SCREEN_MARGIN - h);
    pos
}

fn clamp_px(value: Px, min: Px, max: Px) -> Px {
    let max = if max < min { min } else { max };
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

struct StackedTooltip<'a> {
    sections: Vec<TooltipSection<'a>>,
    locale: Locale,
}

impl Component for StackedTooltip<'_> {
    fn render(self, ctx: &RenderCtx) {
        let StackedTooltip { sections, locale } = self;
        let text_max = MAX_WIDTH - PADDING * 2.0;
        let width = sections
            .iter()
            .enumerate()
            .filter_map(|(index, section)| {
                let section = ctx.ghost_add(
                    format!("tooltip-section-measure-{index}"),
                    SectionBox {
                        section,
                        locale,
                        text_max,
                        width: None,
                    },
                );
                section.bounding_box().map(|rect| rect.width())
            })
            .fold(0.px(), |width, section_width| width.max(section_width));

        ctx.compose(|ctx| {
            let mut y = 0.px();
            for (index, section) in sections.iter().enumerate() {
                let box_tree = ctx.ghost_add(
                    format!("tooltip-section-{index}"),
                    SectionBox {
                        section,
                        locale,
                        text_max,
                        width: Some(width),
                    },
                );
                let Some(box_wh) = box_tree.bounding_box().map(|rect| rect.wh()) else {
                    continue;
                };
                ctx.translate(Xy::new(0.px(), y)).add(box_tree);
                y += box_wh.height + SECTION_GAP;
            }
        });
    }
}

struct SectionBox<'a> {
    section: &'a TooltipSection<'a>,
    locale: Locale,
    text_max: Px,
    width: Option<Px>,
}

impl Component for SectionBox<'_> {
    fn render(self, ctx: &RenderCtx) {
        let SectionBox {
            section,
            locale,
            text_max,
            width,
        } = self;

        let content = ctx.ghost_compose("section-content", |ctx| {
            let mut cells: Vec<table::TableCell> = Vec::new();
            if let Some(title) = &section.title {
                cells.push(table::fit(table::FitAlign::LeftTop, move |ctx| {
                    ctx.add(memoized_text(
                        (&title.key, &text_max, &locale.language),
                        move |mut builder| {
                            builder
                                .headline()
                                .size(FontSize::Medium)
                                .max_width(text_max)
                                .color(palette::WHITE)
                                .stroke(2.px(), palette::DARK_CHARCOAL);
                            (title.apply)(&mut builder);
                            builder.render_left_top()
                        },
                    ));
                }));
                cells.push(table::fixed_no_clip(TITLE_GAP, |_, _| {}));
            }

            let body = &section.body;
            cells.push(table::fit(table::FitAlign::LeftTop, move |ctx| {
                ctx.add(memoized_text(
                    (&body.key, &text_max, &locale.language),
                    move |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Large)
                            .max_width(text_max)
                            .color(palette::WHITE)
                            .stroke(2.px(), palette::DARK_CHARCOAL);
                        (body.apply)(&mut builder);
                        builder.render_left_top()
                    },
                ));
            }));

            table::vertical(cells)(Wh::new(text_max, f32::MAX.px()), ctx);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };
        let container_wh = Wh::new(
            width.unwrap_or(content_wh.width + PADDING * 2.0),
            content_wh.height + PADDING * 2.0,
        );

        ctx.translate(Xy::new(PADDING, PADDING)).add(content);
        ctx.add(PaperContainerBackground {
            width: container_wh.width,
            height: container_wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER,
            outline_color: Some(palette::SURFACE_CONTAINER_OUTLINE),
            shadow: true,
            arrow: None,
        });
    }
}
