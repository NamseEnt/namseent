mod background_asset_loader;
mod tower_asset_loader;

use background_asset_loader::{BackgroundAssetLoader, BackgroundAssetLoaderInitializer};
use namui::*;
use tower_asset_loader::{TowerAssetLoader, TowerAssetLoaderInitializer};

pub static TOWER_ASSET_LOADER_ATOM: Atom<TowerAssetLoader> = namui::Atom::uninitialized();
pub static BACKGROUND_ASSET_LOADER_ATOM: Atom<BackgroundAssetLoader> = namui::Atom::uninitialized();

pub struct AssetLoader {}
impl Component for AssetLoader {
    fn render(self, ctx: &namui::RenderCtx) {
        ctx.add(TowerAssetLoaderInitializer {});
        ctx.add(BackgroundAssetLoaderInitializer {});
    }
}
