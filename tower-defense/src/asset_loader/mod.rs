pub mod background_asset_loader;
pub mod face_card_asset_loader;
pub mod icon_asset_loader;
pub mod monster_asset_loader;
pub mod tower_asset_loader;

use background_asset_loader::{BackgroundAssetLoader, BackgroundAssetLoaderInitializer};
use face_card_asset_loader::FaceCardAssetLoaderInitializer;
use icon_asset_loader::IconAssetLoaderInitializer;
use namui::*;
use tower_asset_loader::{TowerAssetLoader, TowerAssetLoaderInitializer};

pub static TOWER_ASSET_LOADER_ATOM: Atom<TowerAssetLoader> = namui::Atom::uninitialized();
pub static BACKGROUND_ASSET_LOADER_ATOM: Atom<BackgroundAssetLoader> = namui::Atom::uninitialized();
pub static MONSTER_ASSET_LOADER_ATOM: Atom<monster_asset_loader::MonsterAssetLoader> =
    namui::Atom::uninitialized();

pub struct AssetLoader {}
impl Component for AssetLoader {
    fn render(self, ctx: &namui::RenderCtx) {
        ctx.add(TowerAssetLoaderInitializer {});
        ctx.add(BackgroundAssetLoaderInitializer {});
        ctx.add(IconAssetLoaderInitializer {});
        ctx.add(FaceCardAssetLoaderInitializer {});
        ctx.add(monster_asset_loader::MonsterAssetLoaderInitializer {});
    }
}
