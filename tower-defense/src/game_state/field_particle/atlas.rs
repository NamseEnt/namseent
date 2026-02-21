use namui::*;

fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect<Px> {
    Rect::Xywh { x: px(x), y: px(y), width: px(w), height: px(h) }
}

const LINE_SPRITE_W: f32 = 1024.0;
const LINE_SPRITE_H: f32 = 16.0;

pub fn centered_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sw = src_rect.width().as_f32();
    let sh = src_rect.height().as_f32();
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos: scale,
            ssin: 0.0,
            tx: cx - px(scale * sw / 2.0),
            ty: cy - px(scale * sh / 2.0),
        },
        color,
    }
}

pub fn centered_rotated_sprite(
    src_rect: Rect<Px>,
    cx: Px,
    cy: Px,
    scale: f32,
    angle_rad: f32,
    color: Option<Color>,
) -> ImageSprite {
    let sw = src_rect.width().as_f32();
    let sh = src_rect.height().as_f32();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    let scos = scale * cos_a;
    let ssin = scale * sin_a;
    ImageSprite {
        src_rect,
        xform: RSXform {
            scos,
            ssin,
            tx: cx - px(scos * sw / 2.0 - ssin * sh / 2.0),
            ty: cy - px(ssin * sw / 2.0 + scos * sh / 2.0),
        },
        color,
    }
}

pub fn line_sprite(
    start_x: Px,
    start_y: Px,
    end_x: Px,
    end_y: Px,
    thickness: f32,
    color: Option<Color>,
) -> Option<ImageSprite> {
    let dx = (end_x - start_x).as_f32();
    let dy = (end_y - start_y).as_f32();
    let length = (dx * dx + dy * dy).sqrt();
    if length < 0.001 || thickness < 0.001 {
        return None;
    }
    let angle = dy.atan2(dx);
    let scale = thickness / LINE_SPRITE_H;
    let src_w = (length / scale).min(LINE_SPRITE_W);
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let scos = scale * cos_a;
    let ssin = scale * sin_a;
    let half_h = LINE_SPRITE_H / 2.0;
    Some(ImageSprite {
        src_rect: Rect::Xywh {
            x: px(512.0),
            y: px(0.0),
            width: px(src_w),
            height: px(LINE_SPRITE_H),
        },
        xform: RSXform {
            scos,
            ssin,
            tx: start_x + px(ssin * half_h),
            ty: start_y - px(scos * half_h),
        },
        color,
    })
}

pub fn glow_circle() -> Rect<Px> { rect(0.0, 0.0, 128.0, 128.0) }
pub fn star_burst() -> Rect<Px> { rect(128.0, 0.0, 128.0, 128.0) }
pub fn cross() -> Rect<Px> { rect(256.0, 0.0, 128.0, 128.0) }
pub fn ring() -> Rect<Px> { rect(384.0, 0.0, 128.0, 128.0) }
pub fn monster_soul() -> Rect<Px> { rect(1792.0, 256.0, 128.0, 128.0) }

pub fn projectile_rect(kind: crate::game_state::projectile::ProjectileKind) -> Rect<Px> {
    use crate::game_state::projectile::ProjectileKind;
    match kind {
        ProjectileKind::Trash01 => rect(0.0, 128.0, 128.0, 128.0),
        ProjectileKind::Trash02 => rect(128.0, 128.0, 128.0, 128.0),
        ProjectileKind::Trash03 => rect(256.0, 128.0, 128.0, 128.0),
        ProjectileKind::Trash04 => rect(384.0, 128.0, 128.0, 128.0),
    }
}

pub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {
    use crate::game_state::MonsterKind;
    match kind {
        MonsterKind::Mob01 => rect(512.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob02 => rect(640.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob03 => rect(768.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob04 => rect(896.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob05 => rect(1024.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob06 => rect(1152.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob07 => rect(1280.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob08 => rect(1408.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob09 => rect(1536.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob10 => rect(1664.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob11 => rect(1792.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob12 => rect(1920.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob13 => rect(0.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob14 => rect(128.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob15 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob16 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob17 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob18 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob19 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob20 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob21 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob22 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob23 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob24 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob25 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob26 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob27 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob28 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob29 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob30 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob31 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob32 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob33 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob34 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob35 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob36 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob37 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob38 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob39 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob40 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob41 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob42 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob43 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob44 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob45 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob46 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob47 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob48 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob49 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob50 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss01 => rect(384.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss02 => rect(512.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss03 => rect(640.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss04 => rect(768.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss05 => rect(896.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss06 => rect(1024.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss07 => rect(1152.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss08 => rect(1280.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss09 => rect(1408.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss10 => rect(1536.0, 256.0, 128.0, 128.0),
        MonsterKind::Boss11 => rect(1664.0, 256.0, 128.0, 128.0),
    }
}

pub fn icon_rect(kind: &crate::icon::IconKind) -> Rect<Px> {
    use crate::icon::IconKind;
    match kind {
        IconKind::Accept => rect(1920.0, 256.0, 128.0, 128.0),
        IconKind::AttackDamage => rect(128.0, 384.0, 128.0, 128.0),
        IconKind::AttackRange => rect(256.0, 384.0, 128.0, 128.0),
        IconKind::AttackSpeed => rect(384.0, 384.0, 128.0, 128.0),
        IconKind::Config => rect(640.0, 384.0, 128.0, 128.0),
        IconKind::EnemyBoss => rect(896.0, 384.0, 128.0, 128.0),
        IconKind::EnemyNamed => rect(1024.0, 384.0, 128.0, 128.0),
        IconKind::EnemyNormal => rect(1152.0, 384.0, 128.0, 128.0),
        IconKind::Gold => rect(1280.0, 384.0, 128.0, 128.0),
        IconKind::Health => rect(1408.0, 384.0, 128.0, 128.0),
        IconKind::Invincible => rect(1536.0, 384.0, 128.0, 128.0),
        IconKind::Item => rect(1664.0, 384.0, 128.0, 128.0),
        IconKind::Level => rect(1792.0, 384.0, 128.0, 128.0),
        IconKind::Lock => rect(1920.0, 384.0, 128.0, 128.0),
        IconKind::MoveSpeed => rect(0.0, 512.0, 128.0, 128.0),
        IconKind::Contract => rect(384.0, 512.0, 128.0, 128.0),
        IconKind::Refresh => rect(1152.0, 512.0, 128.0, 128.0),
        IconKind::Reject => rect(1280.0, 512.0, 128.0, 128.0),
        IconKind::Shield => rect(1408.0, 512.0, 128.0, 128.0),
        IconKind::Shop => rect(1536.0, 512.0, 128.0, 128.0),
        IconKind::Speaker => rect(1664.0, 512.0, 128.0, 128.0),
        IconKind::Up => rect(256.0, 640.0, 128.0, 128.0),
        IconKind::Down => rect(768.0, 384.0, 128.0, 128.0),
        IconKind::Card => rect(512.0, 384.0, 128.0, 128.0),
        IconKind::New => rect(256.0, 512.0, 128.0, 128.0),
        IconKind::Add => rect(0.0, 384.0, 128.0, 128.0),
        IconKind::Multiply => rect(128.0, 512.0, 128.0, 128.0),
        IconKind::Rating => rect(1024.0, 512.0, 128.0, 128.0),
        IconKind::Suit { suit } => match suit {
            crate::Suit::Spades => rect(128.0, 640.0, 128.0, 128.0),
            crate::Suit::Hearts => rect(0.0, 640.0, 128.0, 128.0),
            crate::Suit::Diamonds => rect(1920.0, 512.0, 128.0, 128.0),
            crate::Suit::Clubs => rect(1792.0, 512.0, 128.0, 128.0),
        },
        IconKind::Rarity { rarity } => match rarity {
            crate::Rarity::Common => rect(512.0, 512.0, 128.0, 128.0),
            crate::Rarity::Rare => rect(896.0, 512.0, 128.0, 128.0),
            crate::Rarity::Epic => rect(640.0, 512.0, 128.0, 128.0),
            crate::Rarity::Legendary => rect(768.0, 512.0, 128.0, 128.0),
        },
    }
}
