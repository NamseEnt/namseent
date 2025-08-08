mod component;
mod rendering_tree;
pub mod rich_text;

use crate::card::Suit;
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}
impl IconKind {
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
        }
    }

    /// Get the default color for this icon kind
    pub fn default_color(&self) -> Color {
        match self {
            IconKind::Accept => Color::from_u8(0, 255, 0, 255), // Green for accept
            IconKind::AttackDamage => Color::from_u8(255, 100, 100, 255),
            IconKind::AttackRange => Color::from_u8(100, 255, 100, 255),
            IconKind::AttackSpeed => Color::from_u8(100, 100, 255, 255),
            IconKind::Config => Color::from_u8(128, 128, 128, 255), // Gray for config
            IconKind::EnemyBoss => Color::from_u8(128, 0, 128, 255),
            IconKind::EnemyNamed => Color::from_u8(255, 165, 0, 255),
            IconKind::EnemyNormal => Color::from_u8(139, 69, 19, 255),
            IconKind::Gold => Color::from_u8(255, 215, 0, 255),
            IconKind::Health => Color::from_u8(255, 0, 0, 255),
            IconKind::Invincible => Color::from_u8(255, 255, 255, 255),
            IconKind::Item => Color::from_u8(255, 192, 203, 255),
            IconKind::Level => Color::from_u8(0, 191, 255, 255), // Deep sky blue for level
            IconKind::Lock => Color::from_u8(169, 169, 169, 255), // Dark gray for lock
            IconKind::MoveSpeed => Color::from_u8(255, 255, 0, 255),
            IconKind::Quest => Color::from_u8(0, 255, 255, 255),
            IconKind::Refresh => Color::from_u8(124, 252, 0, 255), // Lawn green for refresh
            IconKind::Reject => Color::from_u8(255, 0, 0, 255),    // Red for reject
            IconKind::Shield => Color::from_u8(0, 0, 255, 255),
            IconKind::Shop => Color::from_u8(150, 75, 0, 255),
            IconKind::Speaker => Color::from_u8(75, 0, 130, 255), // Indigo for speaker
            IconKind::Suit { .. } => Color::from_u8(128, 128, 128, 255),
            IconKind::Up => Color::from_u8(0, 255, 0, 255),
            IconKind::Down => Color::from_u8(255, 0, 0, 255),
            IconKind::Card => Color::from_u8(255, 255, 255, 255), // White for card
            IconKind::New => Color::from_u8(0, 255, 0, 255),      // Green for new
            IconKind::Add => Color::from_u8(0, 255, 0, 255),      // Green for add
            IconKind::Multiply => Color::from_u8(0, 0, 255, 255), // Blue for multiply
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
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn kind(mut self, icon_kind: IconKind) -> Self {
        self.icon_kind = icon_kind;
        self
    }
    pub fn position(mut self, position: IconAttributePosition) -> Self {
        self.position = position;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone)]
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
    pub fn kind(mut self, kind: IconKind) -> Self {
        self.kind = kind;
        self
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
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}
