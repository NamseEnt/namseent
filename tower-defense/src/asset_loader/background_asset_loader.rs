use super::BACKGROUND_ASSET_LOADER_ATOM;
use crate::game_state::background::BackgroundKind;
use namui::{skia::load_image_from_resource_location, *};
use std::collections::HashMap;

pub struct BackgroundAssetLoaderInitializer {}
impl Component for BackgroundAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        let (_, set_background_asset_loader) =
            ctx.init_atom(&BACKGROUND_ASSET_LOADER_ATOM, BackgroundAssetLoader::new);

        ctx.effect("Load background assets", || {
            [
                BackgroundKind::Tile0,
                BackgroundKind::Tile1,
                BackgroundKind::Tile2,
                BackgroundKind::Tile3,
            ]
            .into_iter()
            .for_each(|background_kind| {
                ctx.spawn(async move {
                    let resource_location = background_image_resource_location(background_kind);

                    match load_image_from_resource_location(resource_location.clone()).await {
                        Ok(image) => {
                            println!("Loaded image from resource location: {resource_location:?}");
                            set_background_asset_loader.mutate(move |background_asset_loader| {
                                background_asset_loader.set_asset(background_kind, image);
                            });
                        }
                        Err(error) => {
                            println!("Failed to load image from resource location: {error:?}");
                        }
                    }
                });
            });
        });
    }
}

pub struct BackgroundAssetLoader {
    pub inner: HashMap<ResourceLocation, namui::Image>,
}
impl BackgroundAssetLoader {
    pub(super) fn new() -> Self {
        let inner = HashMap::new();
        Self { inner }
    }

    fn set_asset(&mut self, background_kind: BackgroundKind, image: namui::Image) {
        self.inner
            .insert(background_image_resource_location(background_kind), image);
    }

    pub fn get(&self, background_kind: BackgroundKind) -> Option<namui::Image> {
        self.inner
            .get(&background_image_resource_location(background_kind))
            .cloned()
    }
}

fn background_image_resource_location(background_kind: BackgroundKind) -> ResourceLocation {
    ResourceLocation::bundle(format!(
        "asset/image/background/{}.jpg",
        background_kind.asset_id()
    ))
}
