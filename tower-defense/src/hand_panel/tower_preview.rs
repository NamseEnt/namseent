use crate::card::RenderTowerCard;
use crate::format_compact_number;
use crate::game_state::flow::GameFlow;
use crate::game_state::tower::TowerTemplate;
use crate::hand::HAND_SLOT_WH;
use crate::theme::typography::{FontSize, memoized_text};
use crate::theme::{
    palette,
    paper_container::{PaperContainerBackground, PaperTexture, PaperVariant},
};
use namui::*;
use rand::Rng;

use crate::animation::with_spring;

const EXIT_ANIMATION_DURATION: f32 = 0.5;
const CARD_MAX_ROTATION_DEG: f32 = 7.0;

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
    rotation_deg: f32,
}

struct PreviewEntryComponent {
    wh: Wh<Px>,
    visible_wh: Wh<Px>,
    template: TowerTemplate,
    active: bool,
    rotation_deg: f32,
}

impl Component for PreviewEntryComponent {
    fn render(self, ctx: &RenderCtx) {
        let game_state = crate::game_state::use_game_state(ctx);
        let container_wh = self.wh;
        let visible_wh = self.visible_wh;
        let template = self.template;
        let tower_upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

        let tracked_upgrade_revision = ctx.track_eq(&game_state.upgrade_state.revision);
        let tracked_template = ctx.track_eq(&(
            template.kind,
            template.suit,
            template.rank,
            template.rerolled_count,
            template.card_polish_pct(),
            template.effective_shoot_interval(),
        ));

        let dps_text_signal = ctx.memo(|| {
            tracked_upgrade_revision.record_as_used();
            tracked_template.record_as_used();

            let damage = template.attack_power_with_upgrade_bonuses(&tower_upgrade_bonuses);
            let shoot_interval_secs = template.effective_shoot_interval().as_secs_f32();
            let dps = if shoot_interval_secs > 0.0 {
                damage / shoot_interval_secs
            } else {
                0.0
            };
            format_compact_number(dps)
        });
        let dps_text = dps_text_signal.as_ref();

        let target = if self.active { 1.0 } else { 0.0 };
        let position: f32 = with_spring(ctx, target, 0.0f32, |v| v * v, || 0.0f32);
        let scale = position.max(0.0001);
        let preview_width = container_wh.width;
        let card_scale = (visible_wh.width / HAND_SLOT_WH.width)
            .min(visible_wh.height / HAND_SLOT_WH.height)
            .min(1.0);
        let card_wh = HAND_SLOT_WH * card_scale;
        let card_xy = Xy::new(
            (container_wh.width - card_wh.width) / 2.0,
            (visible_wh.height - card_wh.height) / 2.0,
        );
        let card_center = card_wh.to_xy() * 0.5;

        ctx.compose(|ctx| {
            let anchor = Xy::new(container_wh.width, container_wh.height / 2.0);
            let ctx = ctx
                .translate(anchor)
                .scale(Xy::single(scale))
                .translate(-anchor);

            ctx.compose(|ctx| {
                let badge_height = 28.px();
                let _ = render_attack_power_badge(&ctx, dps_text, preview_width, badge_height);

                ctx.translate(card_xy)
                    .translate(card_center)
                    .rotate(self.rotation_deg.deg())
                    .translate(-card_center)
                    .add(RenderTowerCard {
                        wh: card_wh,
                        tower_template: &template,
                    });
            });

            ctx.add(PaperContainerBackground {
                width: container_wh.width,
                height: container_wh.height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::Tape,
                color: palette::SURFACE_CONTAINER_LOW,
                outline_color: None,
                shadow: true,
                arrow: None,
            });
        });
    }
}

pub struct HandTowerPreview {
    pub wh: Wh<Px>,
    pub visible_wh: Wh<Px>,
    pub tower_template: Option<TowerTemplate>,
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
                rotation_deg: rand::thread_rng()
                    .gen_range(-CARD_MAX_ROTATION_DEG..=CARD_MAX_ROTATION_DEG),
            });
            next_id += 1;
        }

        entries.retain(|entry| {
            entry
                .exit_animation
                .is_none_or(|exit_animation| !exit_animation.is_complete(now))
        });

        let active_id = if matches!(game_state.flow, GameFlow::SelectingTower(_)) {
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
                    visible_wh: self.visible_wh,
                    template: entry.template.clone(),
                    active,
                    rotation_deg: entry.rotation_deg,
                },
            );
        }

        set_entries.set(entries);
        set_next_id.set(next_id);
    }
}

fn render_attack_power_badge(
    ctx: &ComposeCtx<'_, '_>,
    dps_text: &str,
    container_width: Px,
    badge_height: Px,
) -> Px {
    let badge_text_string = dps_text.to_string();
    let badge_text_ref: &String = &badge_text_string;

    let badge_text = ctx.ghost_add(
        "attack-power-text",
        memoized_text((badge_text_ref, &container_width), move |mut builder| {
            builder
                .paragraph()
                .size(FontSize::Custom { size: 14.px() })
                .bold()
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
    let badge_x = (container_width - badge_width) / 2.0;
    let badge_y = -badge_height / 2.0;
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
