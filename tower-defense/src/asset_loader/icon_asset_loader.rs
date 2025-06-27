use super::ICON_ASSET_LOADER_ATOM;
use crate::icon::{IconAttribute, IconKind};
use namui::skia::load_image_from_resource_location;
use namui::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

// Static global icon asset loader
static GLOBAL_ICON_ASSET_LOADER: OnceLock<Arc<Mutex<IconAssetLoader>>> = OnceLock::new();

pub struct IconAssetLoaderInitializer {}
impl Component for IconAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        let (_, set_icon_asset_loader) =
            ctx.init_atom(&ICON_ASSET_LOADER_ATOM, IconAssetLoader::new);

        // Initialize global static loader
        let global_loader = IconAssetLoader::init_global();

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
                let global_loader_clone = global_loader.clone();
                ctx.spawn(async move {
                    let resource_location = kind.get_resource_location();
                    match load_image_from_resource_location(resource_location.clone()).await {
                        Ok(image) => {
                            // Update both atom and global loader
                            let resource_location_clone = resource_location.clone();
                            let image_clone = image.clone();

                            set_icon_asset_loader.mutate(move |icon_asset_loader| {
                                icon_asset_loader.set_asset(resource_location_clone, image_clone);
                            });

                            // Update global loader
                            if let Ok(mut global) = global_loader_clone.lock() {
                                global.set_asset(resource_location, image);
                            }
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

    // Initialize the global static loader
    pub fn init_global() -> Arc<Mutex<IconAssetLoader>> {
        GLOBAL_ICON_ASSET_LOADER
            .get_or_init(|| Arc::new(Mutex::new(IconAssetLoader::new())))
            .clone()
    }

    // Get global static loader
    pub fn get_global() -> Option<Arc<Mutex<IconAssetLoader>>> {
        GLOBAL_ICON_ASSET_LOADER.get().cloned()
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
