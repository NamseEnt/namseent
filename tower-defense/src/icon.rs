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

impl Component for Icon {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            kind,
            size,
            attributes,
            wh,
            opacity,
        } = self;
        let icon_size = size.px();
        let icon_wh = Wh {
            width: icon_size,
            height: icon_size,
        };
        let icon_xy = Xy::new(
            (wh.width - icon_wh.width) / 2.0,
            (wh.height - icon_wh.height) / 2.0,
        );
        let rect = Rect::from_xy_wh(icon_xy, icon_wh);
        let image = get_icon_image(ctx, kind);
        let Some(image) = image else {
            return;
        };

        // Create paint with opacity
        let paint = if opacity < 1.0 {
            Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, opacity)))
        } else {
            None
        };

        for attribute in attributes {
            let attribute_image = get_icon_image(ctx, attribute.icon_kind);
            let Some(attribute_image) = attribute_image else {
                continue;
            };
            let attribute_render_rect = attribute.attribute_render_rect(rect);
            ctx.add(namui::image(ImageParam {
                rect: attribute_render_rect,
                image: attribute_image.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: paint.clone(),
                },
            }));
        }
        ctx.add(namui::image(ImageParam {
            rect,
            image: image.clone(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint,
            },
        }));
    }
}

impl Icon {
    pub fn to_rendering_tree(&self) -> RenderingTree {
        let Self {
            kind,
            size,
            attributes,
            wh,
            opacity,
        } = self;
        let icon_size = size.px();
        let icon_wh = Wh {
            width: icon_size,
            height: icon_size,
        };
        let icon_xy = Xy::new(
            (wh.width - icon_wh.width) / 2.0,
            (wh.height - icon_wh.height) / 2.0,
        );
        let rect = Rect::from_xy_wh(icon_xy, icon_wh);

        let mut rendering_trees = Vec::new();

        // Try to get images from global asset loader
        if let Some(global_loader) = IconAssetLoader::get_global() {
            // Add attribute images
            for attribute in attributes {
                if let Some(attribute_image) = global_loader.get(attribute.icon_kind) {
                    let attribute_render_rect = attribute.attribute_render_rect(rect);
                    let paint = if *opacity < 1.0 {
                        Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, *opacity)))
                    } else {
                        None
                    };
                    rendering_trees.push(namui::image(ImageParam {
                        rect: attribute_render_rect,
                        image: attribute_image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: paint.clone(),
                        },
                    }));
                }
            }

            // Add main icon image
            if let Some(image) = global_loader.get(*kind) {
                let paint = if *opacity < 1.0 {
                    Some(Paint::new(Color::from_f01(1.0, 1.0, 1.0, *opacity)))
                } else {
                    None
                };
                rendering_trees.push(namui::image(ImageParam {
                    rect,
                    image,
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint,
                    },
                }));
            }

            return namui::render(rendering_trees);
        }

        // Fallback to empty rendering tree if global loader is not available
        namui::render(rendering_trees)
    }
}
