use crate::asset_loader::ICON_ASSET_LOADER_ATOM;
use crate::asset_loader::icon_asset_loader::{IconAssetKind, IconAssetLoader};
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
}
impl IconKind {
    pub fn asset_id(&self) -> &str {
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAttribute {
    Up,
    Down,
}
impl IconAttribute {
    pub fn asset_id(&self) -> &str {
        match self {
            IconAttribute::Up => "up",
            IconAttribute::Down => "down",
        }
    }

    pub fn attribute_render_rect(&self, icon_rect: Rect<Px>) -> Rect<Px> {
        match self {
            IconAttribute::Up | IconAttribute::Down => {
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {
    // Small,
    // Medium,
    // Large,
    // Custom { size: Px },
}
impl IconSize {
    pub fn px(self) -> Px {
        match self {
            // IconSize::Small => px(16.),
            // IconSize::Medium => px(24.),
            // IconSize::Large => px(32.),
            // IconSize::Custom { size } => size,
        }
    }
}

pub struct Icon {
    pub kind: IconKind,
    pub size: IconSize,
    pub attributes: Vec<IconAttribute>,
    pub wh: Wh<Px>,
}

fn get_icon_image(ctx: &RenderCtx, kind: impl Into<IconAssetKind> + Copy) -> Option<namui::Image> {
    let (icon_asset_loader, _) = ctx.atom::<IconAssetLoader>(&ICON_ASSET_LOADER_ATOM);
    icon_asset_loader.get(kind)
}

impl Component for Icon {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            kind,
            size,
            attributes,
            wh,
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
        for attribute in attributes {
            let attribute_image = get_icon_image(ctx, attribute);
            let Some(attribute_image) = attribute_image else {
                continue;
            };
            let attribute_render_rect = attribute.attribute_render_rect(rect);
            ctx.add(namui::image(ImageParam {
                rect: attribute_render_rect,
                image: attribute_image.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
        }
        ctx.add(namui::image(ImageParam {
            rect,
            image: image.clone(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));
    }
}
