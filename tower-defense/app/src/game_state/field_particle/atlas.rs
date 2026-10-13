use crate::atlas::source_rect as rect;
pub use crate::atlas::{centered_rotated_sprite, centered_sprite, line_sprite_from_rect};
use namui::*;

pub fn laser_line_rect() -> Rect<Px> {
    rect(384.0, 0.0, 256.0, 64.0)
}
pub fn burning_tail() -> Rect<Px> {
    rect(0.0, 0.0, 128.0, 128.0)
}
pub fn ember_spark() -> Rect<Px> {
    rect(128.0, 0.0, 128.0, 128.0)
}
pub fn blue_spark() -> Rect<Px> {
    rect(256.0, 0.0, 128.0, 128.0)
}
pub fn lightning_bolt_rect() -> Rect<Px> {
    rect(640.0, 0.0, 256.0, 64.0)
}
pub fn sparkle() -> Rect<Px> {
    rect(896.0, 0.0, 128.0, 128.0)
}
pub fn wind_curve_trail() -> Rect<Px> {
    rect(1024.0, 0.0, 256.0, 64.0)
}
pub fn black_smoke_00() -> Rect<Px> {
    rect(1280.0, 0.0, 128.0, 128.0)
}
pub fn black_smoke_01() -> Rect<Px> {
    rect(1408.0, 0.0, 128.0, 128.0)
}
pub fn red_slash() -> Rect<Px> {
    rect(1536.0, 0.0, 128.0, 128.0)
}
pub fn yellow_explosion() -> Rect<Px> {
    rect(1664.0, 0.0, 128.0, 128.0)
}
pub fn monster_soul() -> Rect<Px> {
    rect(0.0, 512.0, 128.0, 192.0)
}

pub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {
    use crate::game_state::MonsterKind;
    match kind {
        MonsterKind::Mob01 => rect(0.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob02 => rect(128.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob03 => rect(256.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob04 => rect(384.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob05 => rect(512.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob06 => rect(640.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob07 => rect(768.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob08 => rect(896.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob09 => rect(1024.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob10 => rect(1152.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob11 => rect(1280.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob12 => rect(1408.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob13 => rect(1536.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob14 => rect(1664.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob15 => rect(1792.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob16 => rect(1920.0, 0.0, 128.0, 128.0),
        MonsterKind::Mob17 => rect(0.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob18 => rect(128.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob19 => rect(256.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob20 => rect(384.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob21 => rect(512.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob22 => rect(640.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob23 => rect(768.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob24 => rect(896.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob25 => rect(1024.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob26 => rect(1152.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob27 => rect(1280.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob28 => rect(1408.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob29 => rect(1536.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob30 => rect(1664.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob31 => rect(1792.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob32 => rect(1920.0, 128.0, 128.0, 128.0),
        MonsterKind::Mob33 => rect(0.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob34 => rect(128.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob35 => rect(256.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob36 => rect(384.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob37 => rect(512.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob38 => rect(640.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob39 => rect(768.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob40 => rect(896.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob41 => rect(1024.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob42 => rect(1152.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob43 => rect(1280.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob44 => rect(1408.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob45 => rect(1536.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob46 => rect(1664.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob47 => rect(1792.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob48 => rect(1920.0, 256.0, 128.0, 128.0),
        MonsterKind::Mob49 => rect(0.0, 384.0, 128.0, 128.0),
        MonsterKind::Mob50 => rect(128.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss01 => rect(256.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss02 => rect(384.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss03 => rect(512.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss04 => rect(640.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss05 => rect(768.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss06 => rect(896.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss07 => rect(1024.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss08 => rect(1152.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss09 => rect(1280.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss10 => rect(1408.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss11 => rect(1536.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss12 => rect(1664.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss13 => rect(1792.0, 384.0, 128.0, 128.0),
        MonsterKind::Boss14 => rect(1920.0, 384.0, 128.0, 128.0),
    }
}

pub fn projectile_rect(kind: crate::game_state::projectile::ProjectileKind) -> Rect<Px> {
    use crate::game_state::projectile::ProjectileKind;
    match kind {
        ProjectileKind::Trash01 => rect(0.0, 0.0, 128.0, 128.0),
        ProjectileKind::Trash02 => rect(128.0, 0.0, 128.0, 128.0),
        ProjectileKind::Trash03 => rect(256.0, 0.0, 128.0, 128.0),
        ProjectileKind::Trash04 => rect(384.0, 0.0, 128.0, 128.0),
        ProjectileKind::Girl00 => rect(512.0, 0.0, 128.0, 128.0),
        ProjectileKind::Girl01 => rect(640.0, 0.0, 128.0, 128.0),
        ProjectileKind::Girl02 => rect(768.0, 0.0, 128.0, 128.0),
        ProjectileKind::Girl03 => rect(896.0, 0.0, 128.0, 128.0),
        ProjectileKind::Girl04 => rect(1024.0, 0.0, 128.0, 128.0),
        ProjectileKind::Cards00 => rect(1152.0, 0.0, 128.0, 128.0),
        ProjectileKind::Heart00 => rect(1280.0, 0.0, 128.0, 128.0),
    }
}

pub fn card_particle_rect(kind: crate::game_state::field_particle::particle::CardKind) -> Rect<Px> {
    use crate::game_state::field_particle::particle::CardKind;
    match kind {
        CardKind::Card00 => rect(1408.0, 0.0, 128.0, 128.0),
        CardKind::Card01 => rect(1536.0, 0.0, 128.0, 128.0),
        CardKind::Card02 => rect(1664.0, 0.0, 128.0, 128.0),
        CardKind::Card03 => rect(1792.0, 0.0, 128.0, 128.0),
    }
}

pub fn heart_particle_rect(
    kind: crate::game_state::field_particle::particle::HeartParticleKind,
) -> Rect<Px> {
    use crate::game_state::field_particle::particle::HeartParticleKind;
    match kind {
        HeartParticleKind::Heart00 => rect(1920.0, 0.0, 128.0, 128.0),
        HeartParticleKind::Heart01 => rect(0.0, 128.0, 128.0, 128.0),
        HeartParticleKind::Heart02 => rect(128.0, 128.0, 128.0, 128.0),
        HeartParticleKind::RisingHeart { .. } => rect(1280.0, 0.0, 128.0, 128.0),
        _ => rect(384.0, 128.0, 128.0, 128.0),
    }
}

pub fn projectile_pink_smoke() -> Rect<Px> {
    rect(256.0, 128.0, 128.0, 128.0)
}

pub fn icon_rect(kind: &crate::icon::IconKind) -> Rect<Px> {
    use crate::icon::IconKind;
    match kind {
        IconKind::Accept => rect(0.0, 0.0, 128.0, 128.0),
        IconKind::Damage => rect(256.0, 0.0, 128.0, 128.0),
        IconKind::Warning => rect(384.0, 256.0, 128.0, 128.0),
        IconKind::Config => rect(512.0, 0.0, 128.0, 128.0),
        IconKind::EnemyBoss => rect(768.0, 0.0, 128.0, 128.0),
        IconKind::EnemyNamed => rect(896.0, 0.0, 128.0, 128.0),
        IconKind::EnemyNormal => rect(1024.0, 0.0, 128.0, 128.0),
        IconKind::Gold => rect(1152.0, 0.0, 128.0, 128.0),
        IconKind::Health => rect(1280.0, 0.0, 128.0, 128.0),
        IconKind::Invincible => rect(1408.0, 0.0, 128.0, 128.0),
        IconKind::Item => rect(1536.0, 0.0, 128.0, 128.0),
        IconKind::Lock => rect(1664.0, 0.0, 128.0, 128.0),
        IconKind::MoveSpeed => rect(1792.0, 0.0, 128.0, 128.0),
        IconKind::Play => rect(256.0, 256.0, 128.0, 128.0),
        IconKind::Refresh => rect(896.0, 128.0, 128.0, 128.0),
        IconKind::Reject => rect(1024.0, 128.0, 128.0, 128.0),
        IconKind::Shield => rect(1152.0, 128.0, 128.0, 128.0),
        IconKind::Shop => rect(1280.0, 128.0, 128.0, 128.0),
        IconKind::Speaker => rect(1408.0, 128.0, 128.0, 128.0),
        IconKind::Treasure => rect(1536.0, 128.0, 128.0, 128.0),
        IconKind::Up => rect(128.0, 256.0, 128.0, 128.0),
        IconKind::Down => rect(640.0, 0.0, 128.0, 128.0),
        IconKind::Card => rect(384.0, 0.0, 128.0, 128.0),
        IconKind::New => rect(0.0, 128.0, 128.0, 128.0),
        IconKind::Add => rect(128.0, 0.0, 128.0, 128.0),
        IconKind::Multiply => rect(1920.0, 0.0, 128.0, 128.0),
        IconKind::Rating => rect(768.0, 128.0, 128.0, 128.0),
        IconKind::Deck => rect(512.0, 256.0, 128.0, 128.0),
        IconKind::Encyclopedia => rect(512.0, 256.0, 128.0, 128.0),
        IconKind::Suit { suit } => match suit {
            crate::Suit::Spades => rect(0.0, 256.0, 128.0, 128.0),
            crate::Suit::Hearts => rect(1920.0, 128.0, 128.0, 128.0),
            crate::Suit::Diamonds => rect(1792.0, 128.0, 128.0, 128.0),
            crate::Suit::Clubs => rect(1664.0, 128.0, 128.0, 128.0),
        },
        IconKind::Rarity { rarity } => match rarity {
            crate::Rarity::Common => rect(256.0, 128.0, 128.0, 128.0),
            crate::Rarity::Rare => rect(640.0, 128.0, 128.0, 128.0),
            crate::Rarity::Epic => rect(384.0, 128.0, 128.0, 128.0),
            crate::Rarity::Legendary => rect(512.0, 128.0, 128.0, 128.0),
        },
    }
}

pub fn digit_rect(ch: u8) -> Rect<Px> {
    let idx = match ch {
        b'0'..=b'9' => (ch - b'0') as f32,
        b'.' => 10.0,
        b'k' => 11.0,
        b'm' => 12.0,
        b'b' => 13.0,
        _ => 0.0,
    };
    rect(idx * 64.0, 0.0, 64.0, 64.0)
}
