mod component;
mod rendering_tree;
pub mod rich_text;

use crate::{card::Suit, rarity::Rarity};
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum IconKind {
    Accept,
    AttackDamage,
    AttackRange,
    AttackSpeed,
    Config,
    EnemyBoss,
    EnemyNamed,
    EnemyNormal,
    Gold,
    Health,
    Invincible,
    Item,
    Level,
    Lock,
    MoveSpeed,
    Quest,
    Refresh,
    Reject,
    Shield,
    Shop,
    Speaker,
    Suit { suit: Suit },
    Up,
    Down,
    Card,
    New,
    Add,
    Multiply,
    Rarity { rarity: Rarity },
}
impl IconKind {
    pub fn image(self) -> Image {
        match self {
            IconKind::Accept => crate::asset::image::icon::ACCEPT,
            IconKind::AttackDamage => crate::asset::image::icon::ATTACK_DAMAGE,
            IconKind::AttackRange => crate::asset::image::icon::ATTACK_RANGE,
            IconKind::AttackSpeed => crate::asset::image::icon::ATTACK_SPEED,
            IconKind::Config => crate::asset::image::icon::CONFIG,
            IconKind::EnemyBoss => crate::asset::image::icon::ENEMY_BOSS,
            IconKind::EnemyNamed => crate::asset::image::icon::ENEMY_NAMED,
            IconKind::EnemyNormal => crate::asset::image::icon::ENEMY_NORMAL,
            IconKind::Gold => crate::asset::image::icon::GOLD,
            IconKind::Health => crate::asset::image::icon::HEALTH,
            IconKind::Invincible => crate::asset::image::icon::INVINCIBLE,
            IconKind::Item => crate::asset::image::icon::ITEM,
            IconKind::Level => crate::asset::image::icon::LEVEL,
            IconKind::Lock => crate::asset::image::icon::LOCK,
            IconKind::MoveSpeed => crate::asset::image::icon::MOVE_SPEED,
            IconKind::Quest => crate::asset::image::icon::QUEST,
            IconKind::Refresh => crate::asset::image::icon::REFRESH,
            IconKind::Reject => crate::asset::image::icon::REJECT,
            IconKind::Shield => crate::asset::image::icon::SHIELD,
            IconKind::Shop => crate::asset::image::icon::SHOP,
            IconKind::Speaker => crate::asset::image::icon::SPEAKER,
            IconKind::Suit { suit } => match suit {
                Suit::Spades => crate::asset::image::icon::SUIT_SPADES,
                Suit::Hearts => crate::asset::image::icon::SUIT_HEARTS,
                Suit::Diamonds => crate::asset::image::icon::SUIT_DIAMONDS,
                Suit::Clubs => crate::asset::image::icon::SUIT_CLUBS,
            },
            IconKind::Up => crate::asset::image::icon::UP,
            IconKind::Down => crate::asset::image::icon::DOWN,
            IconKind::Card => crate::asset::image::icon::CARD,
            IconKind::New => crate::asset::image::icon::NEW,
            IconKind::Add => crate::asset::image::icon::ADD,
            IconKind::Multiply => crate::asset::image::icon::MULTIPLY,
            IconKind::Rarity { rarity } => match rarity {
                Rarity::Common => crate::asset::image::icon::RARITY_COMMON,
                Rarity::Rare => crate::asset::image::icon::RARITY_RARE,
                Rarity::Epic => crate::asset::image::icon::RARITY_EPIC,
                Rarity::Legendary => crate::asset::image::icon::RARITY_LEGENDARY,
            },
        }
    }

    pub fn asset_id(&self) -> &'static str {
        match self {
            IconKind::Accept => "accept",
            IconKind::AttackDamage => "attack_damage",
            IconKind::AttackRange => "attack_range",
            IconKind::AttackSpeed => "attack_speed",
            IconKind::Config => "config",
            IconKind::EnemyBoss => "enemy_boss",
            IconKind::EnemyNamed => "enemy_named",
            IconKind::EnemyNormal => "enemy_normal",
            IconKind::Gold => "gold",
            IconKind::Health => "health",
            IconKind::Invincible => "invincible",
            IconKind::Item => "item",
            IconKind::Level => "level",
            IconKind::Lock => "lock",
            IconKind::MoveSpeed => "move_speed",
            IconKind::Quest => "quest",
            IconKind::Refresh => "refresh",
            IconKind::Reject => "reject",
            IconKind::Shield => "shield",
            IconKind::Shop => "shop",
            IconKind::Speaker => "speaker",
            IconKind::Suit { suit } => match suit {
                Suit::Spades => "suit_spades",
                Suit::Hearts => "suit_hearts",
                Suit::Diamonds => "suit_diamonds",
                Suit::Clubs => "suit_clubs",
            },
            IconKind::Up => "up",
            IconKind::Down => "down",
            IconKind::Card => "card",
            IconKind::New => "new",
            IconKind::Add => "add",
            IconKind::Multiply => "multiply",
            IconKind::Rarity { rarity } => match rarity {
                Rarity::Common => "rarity_common",
                Rarity::Rare => "rarity_rare",
                Rarity::Epic => "rarity_epic",
                Rarity::Legendary => "rarity_legendary",
            },
        }
    }

    pub fn from_asset_id(asset_id: &str) -> Option<Self> {
        match asset_id {
            "accept" => Some(IconKind::Accept),
            "attack_damage" => Some(IconKind::AttackDamage),
            "attack_range" => Some(IconKind::AttackRange),
            "attack_speed" => Some(IconKind::AttackSpeed),
            "config" => Some(IconKind::Config),
            "enemy_boss" => Some(IconKind::EnemyBoss),
            "enemy_named" => Some(IconKind::EnemyNamed),
            "enemy_normal" => Some(IconKind::EnemyNormal),
            "gold" => Some(IconKind::Gold),
            "health" => Some(IconKind::Health),
            "invincible" => Some(IconKind::Invincible),
            "item" => Some(IconKind::Item),
            "level" => Some(IconKind::Level),
            "lock" => Some(IconKind::Lock),
            "move_speed" => Some(IconKind::MoveSpeed),
            "quest" => Some(IconKind::Quest),
            "refresh" => Some(IconKind::Refresh),
            "reject" => Some(IconKind::Reject),
            "shield" => Some(IconKind::Shield),
            "shop" => Some(IconKind::Shop),
            "speaker" => Some(IconKind::Speaker),
            "suit_spades" => Some(IconKind::Suit { suit: Suit::Spades }),
            "suit_hearts" => Some(IconKind::Suit { suit: Suit::Hearts }),
            "suit_diamonds" => Some(IconKind::Suit {
                suit: Suit::Diamonds,
            }),
            "suit_clubs" => Some(IconKind::Suit { suit: Suit::Clubs }),
            "up" => Some(IconKind::Up),
            "down" => Some(IconKind::Down),
            "card" => Some(IconKind::Card),
            "new" => Some(IconKind::New),
            "add" => Some(IconKind::Add),
            "multiply" => Some(IconKind::Multiply),
            "rarity_common" => Some(IconKind::Rarity {
                rarity: Rarity::Common,
            }),
            "rarity_rare" => Some(IconKind::Rarity {
                rarity: Rarity::Rare,
            }),
            "rarity_epic" => Some(IconKind::Rarity {
                rarity: Rarity::Epic,
            }),
            "rarity_legendary" => Some(IconKind::Rarity {
                rarity: Rarity::Legendary,
            }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct IconAttribute {
    pub icon_kind: IconKind,
    pub position: IconAttributePosition,
}

impl IconAttribute {
    pub fn new(icon_kind: IconKind) -> Self {
        IconAttribute {
            icon_kind,
            position: IconAttributePosition::TopLeft,
        }
    }
    pub fn position(mut self, position: IconAttributePosition) -> Self {
        self.position = position;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum IconAttributePosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl IconAttributePosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            IconAttributePosition::TopLeft => "top_left",
            IconAttributePosition::TopRight => "top_right",
            IconAttributePosition::BottomLeft => "bottom_left",
            IconAttributePosition::BottomRight => "bottom_right",
            IconAttributePosition::Center => "center",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "top_left" => Some(IconAttributePosition::TopLeft),
            "top_right" => Some(IconAttributePosition::TopRight),
            "bottom_left" => Some(IconAttributePosition::BottomLeft),
            "bottom_right" => Some(IconAttributePosition::BottomRight),
            "center" => Some(IconAttributePosition::Center),
            _ => None,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum IconSize {
    Small,
    Medium,
    Large,
    Custom { size: Px },
}
impl IconSize {
    pub fn px(self) -> Px {
        match self {
            IconSize::Small => px(16.),
            IconSize::Medium => px(24.),
            IconSize::Large => px(36.),
            IconSize::Custom { size } => size,
        }
    }
}

#[derive(Clone, State)]
pub struct Icon {
    pub kind: IconKind,
    pub size: IconSize,
    pub attributes: Vec<IconAttribute>,
    pub wh: Wh<Px>,
    pub opacity: f32,
}

impl Default for Icon {
    fn default() -> Self {
        Icon {
            kind: IconKind::Gold,
            size: IconSize::Custom { size: px(24.0) },
            attributes: Vec::new(),
            wh: Wh {
                width: px(24.0),
                height: px(24.0),
            },
            opacity: 1.0,
        }
    }
}

impl Icon {
    pub fn new(kind: IconKind) -> Self {
        Icon {
            kind,
            ..Default::default()
        }
    }
    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }
    pub fn attributes(mut self, attributes: Vec<IconAttribute>) -> Self {
        self.attributes = attributes;
        self
    }
    pub fn wh(mut self, wh: Wh<Px>) -> Self {
        self.wh = wh;
        self
    }
}
