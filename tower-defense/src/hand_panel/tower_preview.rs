use crate::game_state::flow::GameFlow;
use crate::game_state::tower::render::TowerImage;
use crate::game_state::tower::{AnimationKind, TowerKind, TowerTemplate};
use crate::icon::IconKind;
use crate::rarity::Rarity;
use crate::theme::typography::{FontSize, memoized_text};
use crate::theme::{
    halo::Halo,
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;
use namui_prebuilt::{scroll_view::AutoScrollViewWithCtx, table};

use crate::animation::with_spring;

const EXIT_ANIMATION_DURATION: f32 = 0.5;

const PLACEHOLDER_FLAVOR_TEXT: &str = "대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트 대충 긴 플레이버 텍스트";

fn halo_config_for_tower_kind(kind: TowerKind) -> Option<(Color, f32)> {
    match kind {
        TowerKind::Barricade | TowerKind::High | TowerKind::OnePair => None,
        TowerKind::TwoPair => Some((Rarity::Common.color(), 0.05)),
        TowerKind::ThreeOfAKind => Some((Rarity::Rare.color(), 0.1)),
        TowerKind::Straight | TowerKind::Flush => Some((Rarity::Epic.color(), 0.15)),
        TowerKind::FullHouse => Some((Rarity::Epic.color(), 0.2)),
        TowerKind::FourOfAKind | TowerKind::StraightFlush => Some((Rarity::Legendary.color(), 0.3)),
        TowerKind::RoyalFlush => Some((Rarity::Legendary.color(), 0.4)),
    }
}

#[derive(Debug, Clone, Copy, State)]
struct ExitAnimation {
    start_time: Instant,
}

impl ExitAnimation {
    fn new(start_time: Instant) -> Self {
        Self { start_time }
    }

    fn is_complete(self, now: Instant) -> bool {
        (now - self.start_time).as_secs_f32() >= EXIT_ANIMATION_DURATION
    }
}

#[derive(Debug, Clone, State)]
struct PreviewEntry {
    id: usize,
    template: TowerTemplate,
    exit_animation: Option<ExitAnimation>,
}

struct PreviewEntryComponent {
    wh: Wh<Px>,
    template: TowerTemplate,
    active: bool,
}

impl Component for PreviewEntryComponent {
    fn render(self, ctx: &RenderCtx) {
        let game_state = crate::game_state::use_game_state(ctx);

        let target = if self.active { 1.0 } else { 0.0 };
        let position: f32 = with_spring(ctx, target, 0.0f32, |v| v * v, || 0.0f32);
        let scale = position.max(0.0001);
        let this_wh = self.wh;
        let template = self.template.clone();
        let tower_name = game_state.text().tower(template.kind.to_text());
        let flavor = PLACEHOLDER_FLAVOR_TEXT.to_string();

        ctx.compose(|ctx| {
            let anchor = Xy::new(this_wh.width / 2.0, this_wh.height);
            let ctx = ctx
                .translate(anchor)
                .scale(Xy::single(scale))
                .translate(-anchor);

            let img_wh = Wh::new(this_wh.height * 2.0, this_wh.height * 2.0);
            let img_offset = Xy::new(0.px(), -this_wh.height);
            let halo_config = halo_config_for_tower_kind(template.kind);

            ctx.compose(|ctx| {
                let ctx = ctx.translate(img_offset);
                let ctx = ctx.add(image(ImageParam {
                    rect: Rect::Xywh {
                        x: px(0.0),
                        y: px(0.0),
                        width: img_wh.width,
                        height: img_wh.height,
                    },
                    image: (template.kind, AnimationKind::Idle1).image(),
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }));

                if let Some((color, strength)) = halo_config {
                    ctx.add(Halo {
                        wh: img_wh,
                        radius: 96.px(),
                        color,
                        strength,
                        rotation_deg_per_sec: 45.0,
                    });
                }
            });

            let divider_x = img_wh.width + px(8.0);
            let mut path = Path::new();
            let mut y = px(8.0);
            let dash = px(16.0);
            let gap = px(8.0);
            while y < this_wh.height {
                if y + dash > this_wh.height {
                    break;
                }
                path = path.move_to(divider_x, y);
                path = path.line_to(divider_x, y + dash);
                y += dash + gap;
            }
            ctx.add(namui::path(
                path,
                Paint::new(palette::ON_PRIMARY.with_alpha(64))
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(4.px())
                    .set_stroke_cap(StrokeCap::Round),
            ));

            let text_region_width = this_wh.width - (divider_x + px(8.0));
            ctx.compose(|ctx| {
                table::padding_no_clip(
                    16.px(),
                    table::vertical([
                        table::fixed_no_clip(24.px(), move |wh, ctx| {
                            ctx.add(memoized_text(
                                (&wh.width, &template.kind, &template.suit),
                                |mut builder| {
                                    let rank_text = template.rank.to_string();
                                    let mut builder = builder
                                        .headline()
                                        .size(FontSize::Medium)
                                        .max_width(wh.width);
                                    if !matches!(
                                        template.kind,
                                        crate::game_state::tower::TowerKind::Barricade
                                    ) {
                                        builder = builder
                                            .icon(IconKind::Suit {
                                                suit: template.suit,
                                            })
                                            .text(&rank_text)
                                            .space();
                                    }
                                    builder.text(tower_name).render_left_center(wh.height * 0.4)
                                },
                            ));
                        }),
                        table::ratio_no_clip(1, move |wh, ctx| {
                            ctx.add(AutoScrollViewWithCtx {
                                wh,
                                scroll_bar_width: 8.px(),
                                content: |scroll_ctx| {
                                    scroll_ctx.add(memoized_text(
                                        (&flavor, &wh.width),
                                        |mut builder| {
                                            builder
                                                .paragraph()
                                                .size(FontSize::Medium)
                                                .max_width(wh.width - 8.px())
                                                .text(flavor.clone())
                                                .render_left_top()
                                        },
                                    ));
                                },
                            });
                        }),
                    ]),
                )(
                    Wh::new(text_region_width, this_wh.height),
                    ctx.translate(Xy::new(divider_x + px(8.0), 0.px())),
                );
            });

            ctx.add(PaperContainerBackground {
                width: this_wh.width,
                height: this_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Tape,
                color: palette::PRIMARY,
                shadow: true,
                arrow: None,
            });
        });
    }
}

pub struct HandTowerPreview {
    pub wh: Wh<Px>,
    pub tower_template: Option<TowerTemplate>,
    pub panel_open: bool,
}

impl Component for HandTowerPreview {
    fn render(self, ctx: &RenderCtx) {
        let now = Instant::now();
        let game_state = crate::game_state::use_game_state(ctx);
        let (entries_sig, set_entries) = ctx.state(Vec::<PreviewEntry>::new);
        let (next_id_sig, set_next_id) = ctx.state(|| 0_usize);

        let mut entries = entries_sig.clone_inner();
        let mut next_id = next_id_sig.clone_inner();

        if let Some(template) = self.tower_template.clone()
            && entries
                .last()
                .is_none_or(|entry| entry.template != template)
        {
            if let Some(previous_entry) = entries.last_mut()
                && previous_entry.exit_animation.is_none()
            {
                previous_entry.exit_animation = Some(ExitAnimation::new(now));
            }

            entries.push(PreviewEntry {
                id: next_id,
                template: template.clone(),
                exit_animation: None,
            });
            next_id += 1;
        }

        entries.retain(|entry| {
            entry
                .exit_animation
                .is_none_or(|exit_animation| !exit_animation.is_complete(now))
        });

        let active_id = if self.panel_open && matches!(game_state.flow, GameFlow::SelectingTower(_))
        {
            entries
                .iter()
                .rev()
                .find(|entry| entry.exit_animation.is_none())
                .map(|entry| entry.id)
        } else {
            None
        };

        for entry in entries.iter().rev() {
            let active = Some(entry.id) == active_id;
            ctx.add_with_key(
                entry.id,
                PreviewEntryComponent {
                    wh: self.wh,
                    template: entry.template.clone(),
                    active,
                },
            );
        }

        set_entries.set(entries);
        set_next_id.set(next_id);
    }
}
