mod skill;

use super::*;
use namui::*;
pub use skill::*;
use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct Tower {
    id: usize,
    pub left_top: MapCoord,
    cooldown: Duration,
    template: TowerTemplate,
    pub status_effects: Vec<TowerStatusEffect>,
    pub skills: Vec<TowerSkill>,
    animation: Animation,
}
impl Tower {
    pub fn new(template: &TowerTemplate, left_top: MapCoord) -> Self {
        const ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            left_top,
            cooldown: Duration::from_secs(0),
            template: template.clone(),
            status_effects: vec![],
            skills: vec![],
            animation: Animation::new(),
        }
    }
    pub fn in_cooltime(&self) -> bool {
        self.cooldown > Duration::from_secs(0)
    }

    pub fn shoot(&mut self, target_indicator: ProjectileTargetIndicator) -> Projectile {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack);

        Projectile {
            kind: self.projectile_kind,
            xy: self.left_top.map(|t| t as f32 + 0.5),
            velocity: self.projectile_speed,
            target_indicator,
            damage: self.calculate_projectile_damage(),
        }
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_xy().map(|t| t as f32)
    }

    fn calculate_projectile_damage(&self) -> f32 {
        let mut damage = self.default_damage as f32;

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageAdd { add } = status_effect.kind {
                damage += add;
            }
        });

        if damage < 0.0 {
            return 0.0;
        }

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageMul { mul } = status_effect.kind {
                damage *= mul;
            }
        });

        damage
    }

    pub(crate) fn attack_range_radius(&self) -> f32 {
        self.status_effects.iter().fold(
            self.default_attack_range_radius,
            |attack_range_radius, status_effect| {
                if let TowerStatusEffectKind::AttackRangeAdd { add } = status_effect.kind {
                    attack_range_radius + add
                } else {
                    attack_range_radius
                }
            },
        )
    }
}
impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {
        let animation_name = match self.animation.kind {
            AnimationKind::Idle1 => "idle1",
            AnimationKind::Idle2 => "idle2",
            AnimationKind::Attack => "attack",
        };
        let image = ctx.image(ResourceLocation::bundle(format!(
            "tower/{}/{animation_name}.jpg",
            self.kind.asset_id(),
        )));

        if let Some(Ok(image)) = image.as_ref() {
            ctx.add(namui::image(ImageParam {
                rect: Rect::from_xy_wh(Xy::zero(), image.info.wh()),
                image: image.clone(),
                style: ImageStyle {
                    fit: ImageFit::None,
                    paint: None,
                },
            }));
        }
    }
}
impl Deref for Tower {
    type Target = TowerTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Clone)]
pub struct TowerTemplate {
    pub kind: TowerKind,
    pub shoot_interval: Duration,
    pub default_attack_range_radius: f32,
    pub projectile_kind: ProjectileKind,
    pub projectile_speed: Velocity,
    pub default_damage: usize,
}

#[derive(Clone, Copy)]
pub enum TowerKind {
    // TODO: Add tower kinds
}

impl TowerKind {
    fn asset_id(&self) -> &'static str {
        todo!()
    }
}

pub fn tower_cooldown_tick(game_state: &mut GameState, dt: Duration) {
    game_state.towers.iter_mut().for_each(|tower| {
        if tower.cooldown == Duration::from_secs(0) {
            return;
        }

        let mut time_multiple = 1.0;

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedAdd { add } = status_effect.kind {
                time_multiple += add;
            }
        });
        if time_multiple == 0.0 {
            return;
        }

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedMul { mul } = status_effect.kind {
                time_multiple *= mul;
            }
        });

        let cooldown_sub = dt * time_multiple;

        if tower.cooldown < cooldown_sub {
            tower.cooldown = Duration::from_secs(0);
        } else {
            tower.cooldown -= cooldown_sub;
        }
    });
}

pub fn tower_animation_tick(game_state: &mut GameState, now: Instant) {
    game_state.towers.iter_mut().for_each(|tower| {
        let animation = &mut tower.animation;

        if now - animation.start_at < animation.duration() {
            return;
        }

        animation.transition(match animation.kind {
            AnimationKind::Idle1 => AnimationKind::Idle2,
            AnimationKind::Idle2 => AnimationKind::Idle1,
            AnimationKind::Attack => AnimationKind::Idle1,
        });
    });
}

struct Animation {
    kind: AnimationKind,
    start_at: Instant,
}

impl Animation {
    fn new() -> Self {
        Self {
            kind: AnimationKind::Idle1,
            start_at: Instant::now(),
        }
    }

    fn transition(&mut self, kind: AnimationKind) {
        self.kind = kind;
        self.start_at = Instant::now();
    }

    fn duration(&self) -> Duration {
        self.kind.duration()
    }
}

enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}

impl AnimationKind {
    fn duration(&self) -> Duration {
        match self {
            Self::Idle1 => Duration::from_secs(1),
            Self::Idle2 => Duration::from_secs(1),
            Self::Attack => Duration::from_secs(1),
        }
    }
}
