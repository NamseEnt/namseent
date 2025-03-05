use crate::game_state::tower::{AnimationKind, TowerKind};
use namui::{skia::load_image_from_resource_location, *};
use std::collections::HashMap;

pub static TOWER_ASSET_LOADER_ATOM: Atom<TowerAssetLoader> = namui::Atom::uninitialized();

pub struct AssetLoader {}
impl Component for AssetLoader {
    fn render(self, ctx: &namui::RenderCtx) {
        let (_, set_tower_asset_loader) =
            ctx.init_atom(&TOWER_ASSET_LOADER_ATOM, TowerAssetLoader::new);

        ctx.effect("Load tower assets", || {
            namui::spawn(async move {
                let _futures = [
                    TowerKind::Barricade,
                    TowerKind::High,
                    TowerKind::OnePair,
                    TowerKind::TwoPair,
                    TowerKind::ThreeOfAKind,
                    TowerKind::Straight,
                    TowerKind::Flush,
                    TowerKind::FullHouse,
                    TowerKind::FourOfAKind,
                    TowerKind::StraightFlush,
                    TowerKind::RoyalFlush,
                ]
                .into_iter()
                .flat_map(|tower_kind| {
                    [
                        AnimationKind::Idle1,
                        AnimationKind::Idle2,
                        AnimationKind::Attack,
                    ]
                    .into_iter()
                    .map({
                        let set_tower_asset_loader = set_tower_asset_loader.clone();
                        move |animation_kind| async move {
                            let resource_location =
                                tower_image_resource_location(tower_kind, animation_kind);

                            let Ok(image) =
                                load_image_from_resource_location(resource_location).await
                            else {
                                return;
                            };
                            set_tower_asset_loader.mutate(move |tower_asset_loader| {
                                tower_asset_loader.set_asset(tower_kind, animation_kind, image);
                            });
                        }
                    })
                });
            });
        });
    }
}

pub struct TowerAssetLoader {
    pub inner: HashMap<ResourceLocation, namui::Image>,
}
impl TowerAssetLoader {
    fn new() -> Self {
        let inner = HashMap::new();
        Self { inner }
    }

    fn set_asset(
        &mut self,
        tower_kind: TowerKind,
        animation_kind: AnimationKind,
        image: namui::Image,
    ) {
        self.inner.insert(
            tower_image_resource_location(tower_kind, animation_kind),
            image,
        );
    }

    pub fn get(
        &self,
        tower_kind: TowerKind,
        animation_kind: AnimationKind,
    ) -> Option<namui::Image> {
        self.inner
            .get(&tower_image_resource_location(tower_kind, animation_kind))
            .cloned()
    }
}

fn tower_image_resource_location(
    tower_kind: TowerKind,
    animation_kind: AnimationKind,
) -> ResourceLocation {
    ResourceLocation::bundle(format!(
        "asset/image/tower/{}/{}.png",
        tower_kind.asset_id(),
        animation_kind.asset_id()
    ))
}
