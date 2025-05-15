use super::ICON_ASSET_LOADER_ATOM;
use crate::icon::{IconAttribute, IconKind};
use namui::skia::load_image_from_resource_location;
use namui::*;
use std::collections::HashMap;

pub struct IconAssetLoaderInitializer {}
impl Component for IconAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        let (_, set_icon_asset_loader) =
            ctx.init_atom(&ICON_ASSET_LOADER_ATOM, IconAssetLoader::new);

        ctx.effect("Load icon assets", || {
            let icon_asset_kinds = [
                IconKind::AttackDamage,
                IconKind::AttackRange,
                IconKind::AttackSpeed,
                IconKind::EnemyBoss,
                IconKind::EnemyNamed,
                IconKind::EnemyNormal,
                IconKind::Gold,
                IconKind::Invincible,
                IconKind::Item,
                IconKind::MoveSpeed,
                IconKind::Quest,
                IconKind::Shield,
                IconKind::Shop,
                IconKind::Health,
            ]
            .into_iter()
            .map(IconAssetKind::from)
            .chain(
                [IconAttribute::Up, IconAttribute::Down]
                    .into_iter()
                    .map(IconAssetKind::from),
            );
            for kind in icon_asset_kinds {
                ctx.spawn(async move {
                    let resource_location = kind.get_resource_location();
                    match load_image_from_resource_location(resource_location.clone()).await {
                        Ok(image) => {
                            set_icon_asset_loader.mutate(move |icon_asset_loader| {
                                icon_asset_loader.set_asset(resource_location.clone(), image);
                            });
                        }
                        Err(error) => {
                            println!("Failed to load icon image: {:?}", error);
                        }
                    }
                });
            }
        });
    }
}

pub struct IconAssetLoader {
    pub inner: HashMap<ResourceLocation, namui::Image>,
}
impl IconAssetLoader {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn set_asset(&mut self, kind: ResourceLocation, image: namui::Image) {
        self.inner.insert(kind, image);
    }

    pub fn get<T: Into<IconAssetKind>>(&self, kind: T) -> Option<namui::Image> {
        let icon_asset_kind: IconAssetKind = kind.into();
        self.inner
            .get(&icon_asset_kind.get_resource_location())
            .cloned()
    }
}

pub enum IconAssetKind {
    Icon(IconKind),
    Attribute(IconAttribute),
}
impl From<IconKind> for IconAssetKind {
    fn from(kind: IconKind) -> Self {
        Self::Icon(kind)
    }
}
impl From<IconAttribute> for IconAssetKind {
    fn from(attribute: IconAttribute) -> Self {
        Self::Attribute(attribute)
    }
}
impl IconAssetKind {
    fn get_resource_location(&self) -> ResourceLocation {
        let asset_id = match self {
            IconAssetKind::Icon(kind) => kind.asset_id(),
            IconAssetKind::Attribute(attribute) => attribute.asset_id(),
        };
        ResourceLocation::bundle(format!("asset/image/icon/{}.png", asset_id))
    }
}
