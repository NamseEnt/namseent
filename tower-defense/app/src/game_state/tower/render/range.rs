use crate::PresentationInstant;
use crate::WorldDistance;
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::tower::TowerTemplate;
use crate::palette;
use namui::*;

pub(crate) struct TowerAttackRange {
    pub(crate) range_radius: WorldDistance,
    pub(crate) splash_radii: Vec<WorldDistance>,
}

impl TowerAttackRange {
    pub(crate) fn from_template(tower_template: &TowerTemplate) -> Self {
        let engraving_modifier = tower_template.engraving_modifier();
        Self {
            range_radius: tower_template.attack_range_radius(),
            splash_radii: engraving_modifier
                .on_attack_splashes
                .iter()
                .map(|splash| splash.radius)
                .collect(),
        }
    }
}

impl Component for TowerAttackRange {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            range_radius,
            splash_radii,
        } = self;

        let range_radius_px = TILE_PX_SIZE.width
            * (range_radius.raw() as f32 / crate::world::WORLD_UNITS_PER_TILE as f32);

        const ROTATION_SPEED_PX_PER_SEC: f32 = 120.0;
        const DASH_ON_PX: f32 = 40.0;
        const DASH_OFF_PX: f32 = 24.0;

        let elapsed_secs =
            (PresentationInstant::capture() - PresentationInstant::zero()).as_secs_f32();
        let phase_px = (elapsed_secs * ROTATION_SPEED_PX_PER_SEC) % (DASH_ON_PX + DASH_OFF_PX);

        let ctx = ctx.translate(TILE_PX_SIZE.to_xy());

        let oval = Rect::Ltrb {
            left: -range_radius_px,
            top: -range_radius_px,
            right: range_radius_px,
            bottom: range_radius_px,
        };
        let path = Path::new().add_oval(oval);
        let paint = Paint::new(palette::PRIMARY)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(4.px())
            .set_stroke_cap(StrokeCap::Round)
            .set_path_effect(PathEffect::Dash {
                on: DASH_ON_PX,
                off: DASH_OFF_PX,
                phase: phase_px,
            });
        ctx.add(namui::path(path, paint));
        for splash_radius in splash_radii {
            let splash_radius_px = TILE_PX_SIZE.width
                * (splash_radius.raw() as f32 / crate::world::WORLD_UNITS_PER_TILE as f32);
            let splash_oval = Rect::Ltrb {
                left: -splash_radius_px,
                top: -splash_radius_px,
                right: splash_radius_px,
                bottom: splash_radius_px,
            };
            let splash_path = Path::new().add_oval(splash_oval);
            let splash_paint = Paint::new(palette::GREEN)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(3.px())
                .set_stroke_cap(StrokeCap::Round)
                .set_path_effect(PathEffect::Dash {
                    on: DASH_ON_PX,
                    off: DASH_OFF_PX,
                    phase: phase_px,
                });
            ctx.add(namui::path(splash_path, splash_paint));
        }
    }
}
