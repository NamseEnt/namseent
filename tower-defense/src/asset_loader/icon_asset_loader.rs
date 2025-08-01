use crate::card::Suit;
use crate::icon::IconKind;
use namui::skia::load_image_from_resource_location;
use namui::*;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

// Static global icon asset loader
static GLOBAL_ICON_ASSET_LOADER: OnceLock<Arc<IconAssetLoader>> = OnceLock::new();

pub struct IconAssetLoaderInitializer {}
impl Component for IconAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        ctx.effect("Load icon assets", || {
            let icon_asset_kinds = [
                IconKind::Accept,
                IconKind::AttackDamage,
                IconKind::AttackRange,
                IconKind::AttackSpeed,
                IconKind::Config,
                IconKind::EnemyBoss,
                IconKind::EnemyNamed,
                IconKind::EnemyNormal,
                IconKind::Gold,
                IconKind::Health,
                IconKind::Invincible,
                IconKind::Item,
                IconKind::Level,
                IconKind::Lock,
                IconKind::MoveSpeed,
                IconKind::Quest,
                IconKind::Refresh,
                IconKind::Reject,
                IconKind::Shield,
                IconKind::Shop,
                IconKind::Speaker,
                IconKind::Suit { suit: Suit::Spades },
                IconKind::Suit { suit: Suit::Hearts },
                IconKind::Suit {
                    suit: Suit::Diamonds,
                },
                IconKind::Suit { suit: Suit::Clubs },
                IconKind::Up,
                IconKind::Down,
            ]
            .into_iter()
            .map(IconAssetKind::from)
            .collect::<Vec<_>>();

            ctx.spawn(async move {
                let mut asset_map = HashMap::new();
                for kind in icon_asset_kinds {
                    let resource_location = kind.get_resource_location();
                    match load_image_from_resource_location(resource_location.clone()).await {
                        Ok(image) => {
                            asset_map.insert(resource_location.clone(), image.clone());
                        }
                        Err(error) => {
                            println!("Failed to load icon image: {error:?}");
                        }
                    }
                }
                let loader = IconAssetLoader { inner: asset_map };
                let arc_loader = Arc::new(loader);
                GLOBAL_ICON_ASSET_LOADER.set(arc_loader).ok();
            });
        });
    }
}

#[derive(Clone)]
pub struct IconAssetLoader {
    pub inner: HashMap<ResourceLocation, namui::Image>,
}
impl IconAssetLoader {
    pub fn get_global() -> Option<Arc<IconAssetLoader>> {
        GLOBAL_ICON_ASSET_LOADER.get().cloned()
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
}
impl From<IconKind> for IconAssetKind {
    fn from(kind: IconKind) -> Self {
        Self::Icon(kind)
    }
}
impl IconAssetKind {
    fn get_resource_location(&self) -> ResourceLocation {
        let asset_id = match self {
            IconAssetKind::Icon(kind) => kind.asset_id(),
        };
        ResourceLocation::bundle(format!("asset/image/icon/{asset_id}.png"))
    }
}
