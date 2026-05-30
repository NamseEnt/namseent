use crate::format_compact_number;
use crate::game_state::flow::GameFlow;
use crate::game_state::tower::render::{TowerImage, TowerSpriteWithOverlay};
use crate::game_state::tower::{AnimationKind, TowerKind, TowerTemplate};
use crate::rarity::Rarity;
use crate::theme::typography::{FontSize, memoized_text};
use crate::theme::{
    halo::Halo,
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;
use namui_prebuilt::table;

use crate::animation::with_spring;

const EXIT_ANIMATION_DURATION: f32 = 0.5;

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
        let this_wh = self.wh;
        let template = self.template;
        let tower_upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

        let tracked_upgrade_revision = ctx.track_eq(&game_state.upgrade_state.revision);
        let tracked_template = ctx.track_eq(&(
            template.kind,
            template.suit,
            template.rank,
            template.rerolled_count,
        ));

        let attack_power_text_signal = ctx.memo(|| {
            tracked_upgrade_revision.record_as_used();
            tracked_template.record_as_used();

            let attack_power = template.attack_power_with_upgrade_bonuses(&tower_upgrade_bonuses);
            format_compact_number(attack_power)
        });
        let attack_power_text = attack_power_text_signal.as_ref();

        let target = if self.active { 1.0 } else { 0.0 };
        let position: f32 = with_spring(ctx, target, 0.0f32, |v| v * v, || 0.0f32);
        let scale = position.max(0.0001);
        let tower_name = game_state.text().tower(template.kind.to_text());

        ctx.compose(|ctx| {
            let anchor = Xy::new(this_wh.width, this_wh.height / 2.0);
            let ctx = ctx
                .translate(anchor)
                .scale(Xy::single(scale))
                .translate(-anchor);

            let halo_config = halo_config_for_tower_kind(template.kind);

            ctx.compose(|ctx| {
                table::padding_no_clip(
                    8.px(),
                    table::vertical([
                        table::ratio_no_clip(1, move |wh, ctx| {
                            let img_wh = wh * 1.2;
                            let row_center = wh.to_xy() * 0.5;
                            let image_center = Xy::new(row_center.x, row_center.y - 8.px());

                            let badge_height = 28.px();
                            let badge_origin = Xy::new(
                                image_center.x - (img_wh.width / 2.0) + 16.px(),
                                image_center.y - (wh.height / 2.0) + 8.px(),
                            );
                            let _ = render_attack_power_badge(
                                &ctx.translate(badge_origin),
                                attack_power_text,
                                img_wh.width,
                                badge_height,
                            );

                            ctx.translate(image_center - (img_wh.to_xy() * 0.5)).add(
                                TowerSpriteWithOverlay {
                                    image: (template.kind, AnimationKind::Idle1).image(),
                                    wh: img_wh,
                                    suit: template.suit,
                                    rank: template.rank,
                                    alpha: 1.0,
                                },
                            );

                            if let Some((color, strength)) = halo_config {
                                ctx.translate(image_center - (img_wh.to_xy() * 0.5))
                                    .add(Halo {
                                        wh: img_wh,
                                        radius: 40.px(),
                                        color,
                                        strength,
                                        rotation_deg_per_sec: 45.0,
                                    });
                            }
                        }),
                        table::fixed_no_clip(4.px(), |_, _| {}),
                        table::fixed_no_clip(22.px(), move |wh, ctx| {
                            ctx.add(memoized_text((&wh.width, &template.kind), |mut builder| {
                                builder
                                    .headline()
                                    .size(FontSize::Medium)
                                    .max_width(wh.width);
                                builder.text(tower_name).render_center_bottom(wh)
                            }));
                        }),
                    ]),
                )(this_wh, ctx);
            });

            ctx.add(PaperContainerBackground {
                width: this_wh.width,
                height: this_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Tape,
                color: palette::PRIMARY,
                outline_color: None,
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

        let active_id = if self.panel_open
            && matches!(
                game_state.flow,
                GameFlow::SelectingTower(_) | GameFlow::PlacingTower
            ) {
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

fn render_attack_power_badge(
    ctx: &ComposeCtx<'_, '_>,
    attack_power_text: &str,
    container_width: Px,
    container_height: Px,
) -> Px {
    let badge_text_string = attack_power_text.to_string();
    let badge_text_ref: &String = &badge_text_string;

    let badge_height = container_height;
    let badge_text = ctx.ghost_add(
        "attack-power-text",
        memoized_text((badge_text_ref, &container_width), move |mut builder| {
            builder
                .paragraph()
                .size(FontSize::Custom { size: 14.px() })
                .bold()
                .color(palette::WHITE)
                .text(badge_text_ref.as_str())
                .render_left_center(badge_height)
        }),
    );

    let badge_text_width = badge_text
        .bounding_box()
        .map(|rect| rect.width())
        .unwrap_or_default();
    let badge_padding = 6.px();
    let badge_gap = 4.px();
    let badge_icon_width = 16.px();
    let badge_width =
        badge_padding + badge_icon_width + badge_gap + badge_text_width + badge_padding;
    let badge_x = 0.px();
    let badge_y = (container_height - badge_height) / 2.0;
    let badge_rect = Rect::Xywh {
        x: badge_x,
        y: badge_y,
        width: badge_width,
        height: badge_height,
    };
    let badge_radius = badge_height / 2.0;
    let badge_path = Path::new().add_rrect(badge_rect, badge_radius, badge_radius);

    ctx.translate(Xy::new(
        badge_x + badge_padding + badge_icon_width + badge_gap,
        badge_y,
    ))
    .add(badge_text);

    ctx.translate(Xy::new(badge_x + badge_padding, badge_y))
        .add(
            crate::icon::Icon::new(crate::icon::IconKind::Damage)
                .size(crate::icon::IconSize::Small)
                .wh(Wh::new(badge_icon_width, badge_height)),
        );

    ctx.add(namui::path(
        badge_path.clone(),
        Paint::new(palette::WHITE)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(3.px()),
    ));
    ctx.add(namui::path(
        badge_path,
        Paint::new(palette::RED).set_style(PaintStyle::Fill),
    ));

    badge_width
}
