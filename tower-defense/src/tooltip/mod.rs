use crate::animation::with_spring;
use crate::game_state::item::Item;
use crate::game_state::upgrade::Upgrade;
use crate::game_state::use_game_state;
use crate::icon::IconKind;
use crate::l10n::{self, Locale};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::{FontSize, TypographyBuilder, memoized_text};
use namui::*;
use namui_prebuilt::table;
use std::sync::atomic::{AtomicU64, Ordering};

const PADDING: Px = px(12.0);
const MAX_WIDTH: Px = px(320.0);
const TITLE_GAP: Px = px(8.0);
const SECTION_GAP: Px = px(8.0);
const ANCHOR_GAP: Px = px(8.0);
const SCREEN_MARGIN: Px = px(8.0);

static NEXT_TOOLTIP_ID: AtomicU64 = AtomicU64::new(0);

/// 각 hover 대상마다 1개씩 발급되는 고유 식별자.
/// 동시에 여러 컴포넌트가 atom을 건드릴 때 자기 것만 지우기 위해 사용.
#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct TooltipId(u64);

impl TooltipId {
    pub fn new() -> Self {
        Self(NEXT_TOOLTIP_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// anchor(대상) 기준으로 어느 쪽에 띄울지에 대한 선호. 화면 밖이면 반대편으로 뒤집힌다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum TooltipPlacement {
    LeftOf,
    RightOf,
    Above,
    Below,
}

/// tooltip으로 보여줄 대상. 새 section을 가진 대상은 여기에 variant를 추가하면 된다.
#[derive(Debug, Clone, PartialEq, State)]
pub enum TooltipContent {
    Item(Item),
    Upgrade(Upgrade),
    Reroll { health_cost: usize },
}

#[derive(Debug, Clone, PartialEq, State)]
struct TooltipRequest {
    id: TooltipId,
    anchor: Rect<Px>,
    placement: TooltipPlacement,
    content: TooltipContent,
}

static TOOLTIP: Atom<Option<TooltipRequest>> = Atom::uninitialized();

/// hover 시작 시 호출. anchor는 대상의 화면 절대 좌표 rect.
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

/// hover 종료 시 호출. 현재 떠 있는 tooltip이 자기 것일 때만 지운다.
pub fn hide_tooltip(id: TooltipId) {
    TOOLTIP.mutate(move |current| {
        if current.as_ref().map(|request| request.id) == Some(id) {
            *current = None;
        }
    });
}

/// 한 section의 제목/본문 텍스트를 빌더에 적용하는 클로저 + 캐시 키.
struct SectionText<'a> {
    key: String,
    apply: Box<dyn Fn(&mut TypographyBuilder) + 'a>,
}

/// stacked tooltip의 박스 1개.
struct TooltipSection<'a> {
    title: Option<SectionText<'a>>,
    body: SectionText<'a>,
}

impl TooltipContent {
    fn sections(&self, locale: Locale) -> Vec<TooltipSection<'_>> {
        match self {
            TooltipContent::Item(item) => vec![TooltipSection {
                title: Some(SectionText {
                    key: format!("item:{:?}:name", item.discriminant()),
                    apply: Box::new(move |builder| item.l10n_name(builder, &locale)),
                }),
                body: SectionText {
                    key: format!("item:{item:?}:desc"),
                    apply: Box::new(move |builder| item.l10n_description(builder, &locale)),
                },
            }],
            TooltipContent::Upgrade(upgrade) => vec![TooltipSection {
                title: Some(SectionText {
                    key: format!("upgrade:{upgrade:?}:name"),
                    apply: Box::new(move |builder| {
                        builder.l10n(l10n::upgrade::UpgradeTypeText::Name(upgrade), &locale);
                    }),
                }),
                body: SectionText {
                    key: format!("upgrade:{upgrade:?}:desc"),
                    apply: Box::new(move |builder| {
                        builder.l10n(
                            l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(upgrade),
                            &locale,
                        );
                    }),
                },
            }],
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
        }
    }
}

/// 화면 최상위 tooltip 레이어. `Game` 트리의 가장 마지막에 한 번 추가한다.
pub struct TooltipLayer;

impl Component for TooltipLayer {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);
        let locale = game_state.text().locale();

        let (request, _) = ctx.init_atom(&TOOLTIP, || None::<TooltipRequest>);
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
            let sections = request.content.sections(locale);
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

        ctx.compose(|ctx| {
            let mut y = 0.px();
            for (index, section) in sections.iter().enumerate() {
                let box_tree = ctx.ghost_add(
                    format!("tooltip-section-{index}"),
                    SectionBox {
                        section,
                        locale,
                        text_max,
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
}

impl Component for SectionBox<'_> {
    fn render(self, ctx: &RenderCtx) {
        let SectionBox {
            section,
            locale,
            text_max,
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
        let container_wh = content_wh + Wh::single(PADDING * 2.0);

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
