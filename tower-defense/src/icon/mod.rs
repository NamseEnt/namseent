mod component;
mod rendering_tree;

use crate::asset_loader::icon_asset_loader::{IconAssetKind, IconAssetLoader};
use crate::card::Suit;
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    AttackDamage,
    AttackRange,
    AttackSpeed,
    EnemyBoss,
    EnemyNamed,
    EnemyNormal,
    Gold,
    Invincible,
    Item,
    MoveSpeed,
    Quest,
    Shield,
    Shop,
    Health,
    Suit { suit: Suit },
    Up,
    Down,
}
impl IconKind {
    pub fn asset_id(&self) -> &'static str {
        match self {
            IconKind::AttackDamage => "attack_damage",
            IconKind::AttackRange => "attack_range",
            IconKind::AttackSpeed => "attack_speed",
            IconKind::EnemyBoss => "enemy_boss",
            IconKind::EnemyNamed => "enemy_named",
            IconKind::EnemyNormal => "enemy_normal",
            IconKind::Gold => "gold",
            IconKind::Invincible => "invincible",
            IconKind::Item => "item",
            IconKind::MoveSpeed => "move_speed",
            IconKind::Quest => "quest",
            IconKind::Shield => "shield",
            IconKind::Shop => "shop",
            IconKind::Health => "health",
            IconKind::Suit { suit } => match suit {
                Suit::Spades => "suit_spades",
                Suit::Hearts => "suit_hearts",
                Suit::Diamonds => "suit_diamonds",
                Suit::Clubs => "suit_clubs",
            },
            IconKind::Up => "up",
            IconKind::Down => "down",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IconAttribute {
    pub icon_kind: IconKind,
    pub position: IconAttributePosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAttributePosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl IconAttribute {
    pub fn attribute_render_rect(&self, icon_rect: Rect<Px>) -> Rect<Px> {
        match self.position {
            IconAttributePosition::TopLeft => {
                let left = icon_rect.left();
                let top = icon_rect.top();
                let right = icon_rect.left() + icon_rect.width() * 0.5;
                let bottom = icon_rect.top() + icon_rect.height() * 0.5;
                Rect::Ltrb {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
            IconAttributePosition::TopRight => {
                let left = icon_rect.left() + icon_rect.width() * 0.5;
                let top = icon_rect.top();
                let right = icon_rect.right();
                let bottom = icon_rect.top() + icon_rect.height() * 0.5;
                Rect::Ltrb {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
            IconAttributePosition::BottomLeft => {
                let left = icon_rect.left();
                let top = icon_rect.top() + icon_rect.height() * 0.5;
                let right = icon_rect.left() + icon_rect.width() * 0.5;
                let bottom = icon_rect.bottom();
                Rect::Ltrb {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
            IconAttributePosition::BottomRight => {
                let left = icon_rect.left() + icon_rect.width() * 0.5;
                let top = icon_rect.top() + icon_rect.height() * 0.5;
                let right = icon_rect.right();
                let bottom = icon_rect.bottom();
                Rect::Ltrb {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
            IconAttributePosition::Center => {
                let size_factor = 0.6; // 중앙에 더 작게 배치
                let width = icon_rect.width() * size_factor;
                let height = icon_rect.height() * size_factor;
                let left = icon_rect.left() + (icon_rect.width() - width) * 0.5;
                let top = icon_rect.top() + (icon_rect.height() - height) * 0.5;
                let right = left + width;
                let bottom = top + height;
                Rect::Ltrb {
                    left,
                    top,
                    right,
                    bottom,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
    // Small,
    // Medium,
    // Large,
    Custom { size: Px },
}
impl IconSize {
    pub fn px(self) -> Px {
        match self {
            // IconSize::Small => px(16.),
            // IconSize::Medium => px(24.),
            // IconSize::Large => px(32.),
            IconSize::Custom { size } => size,
        }
    }
}

#[derive(Clone)]
pub struct Icon {
    pub kind: IconKind,
    pub size: IconSize,
    pub attributes: Vec<IconAttribute>,
    pub wh: Wh<Px>,
    pub opacity: f32,
}

fn get_icon_image(_ctx: &RenderCtx, kind: impl Into<IconAssetKind> + Copy) -> Option<namui::Image> {
    IconAssetLoader::get_global().and_then(|loader| loader.get(kind))
}
