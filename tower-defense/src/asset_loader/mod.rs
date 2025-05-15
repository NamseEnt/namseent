pub mod background_asset_loader;
pub mod icon_asset_loader;
pub mod tower_asset_loader;

pub use self::icon_asset_loader::IconAssetLoader;
use background_asset_loader::{BackgroundAssetLoader, BackgroundAssetLoaderInitializer};
use icon_asset_loader::IconAssetLoaderInitializer;
use namui::*;
use tower_asset_loader::{TowerAssetLoader, TowerAssetLoaderInitializer};

pub static TOWER_ASSET_LOADER_ATOM: Atom<TowerAssetLoader> = namui::Atom::uninitialized();
pub static BACKGROUND_ASSET_LOADER_ATOM: Atom<BackgroundAssetLoader> = namui::Atom::uninitialized();
pub static ICON_ASSET_LOADER_ATOM: namui::Atom<IconAssetLoader> = namui::Atom::uninitialized();

pub struct AssetLoader {}
impl Component for AssetLoader {
    fn render(self, ctx: &namui::RenderCtx) {
        ctx.add(TowerAssetLoaderInitializer {});
        ctx.add(BackgroundAssetLoaderInitializer {});
        ctx.add(IconAssetLoaderInitializer {});
    }
}
