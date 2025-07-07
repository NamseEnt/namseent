use namui::{skia::load_image_from_resource_location, *};
use std::collections::HashMap;

use crate::{asset_loader::MONSTER_ASSET_LOADER_ATOM, game_state::MonsterKind};

pub struct MonsterAssetLoaderInitializer {}
impl Component for MonsterAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        let (_, set_monster_asset_loader) =
            ctx.init_atom(&MONSTER_ASSET_LOADER_ATOM, MonsterAssetLoader::new);

        ctx.effect("Load monster assets", || {
            [
                MonsterKind::Mob01,
                MonsterKind::Mob02,
                MonsterKind::Mob03,
                MonsterKind::Mob04,
                MonsterKind::Mob05,
                MonsterKind::Mob06,
                MonsterKind::Mob07,
                MonsterKind::Mob08,
                MonsterKind::Mob09,
                MonsterKind::Mob10,
                MonsterKind::Mob11,
                MonsterKind::Mob12,
                MonsterKind::Mob13,
                MonsterKind::Mob14,
                MonsterKind::Mob15,
                MonsterKind::Named01,
                MonsterKind::Named02,
                MonsterKind::Named03,
                MonsterKind::Named04,
                MonsterKind::Named05,
                MonsterKind::Named06,
                MonsterKind::Named07,
                MonsterKind::Named08,
                MonsterKind::Named09,
                MonsterKind::Named10,
                MonsterKind::Named11,
                MonsterKind::Named12,
                MonsterKind::Named13,
                MonsterKind::Named14,
                MonsterKind::Named15,
                MonsterKind::Named16,
                MonsterKind::Boss01,
                MonsterKind::Boss02,
                MonsterKind::Boss03,
                MonsterKind::Boss04,
                MonsterKind::Boss05,
                MonsterKind::Boss06,
                MonsterKind::Boss07,
                MonsterKind::Boss08,
                MonsterKind::Boss09,
                MonsterKind::Boss10,
                MonsterKind::Boss11,
            ]
            .into_iter()
            .for_each(|monster_kind| {
                ctx.spawn(async move {
                    let resource_location = monster_image_resource_location(monster_kind);
                    let Ok(image) = load_image_from_resource_location(resource_location).await
                    else {
                        return;
                    };
                    set_monster_asset_loader.mutate(move |monster_asset_loader| {
                        monster_asset_loader.set_asset(monster_kind, image);
                    });
                });
            })
        });
    }
}

pub struct MonsterAssetLoader {
    pub inner: HashMap<ResourceLocation, namui::Image>,
}
impl MonsterAssetLoader {
    pub(super) fn new() -> Self {
        let inner = HashMap::new();
        Self { inner }
    }

    fn set_asset(&mut self, monster_kind: MonsterKind, image: namui::Image) {
        self.inner
            .insert(monster_image_resource_location(monster_kind), image);
    }

    pub fn get(&self, monster_kind: MonsterKind) -> Option<namui::Image> {
        self.inner
            .get(&monster_image_resource_location(monster_kind))
            .cloned()
    }
}

fn monster_image_resource_location(monster_kind: MonsterKind) -> ResourceLocation {
    ResourceLocation::bundle(format!(
        "asset/image/monster/{}.png",
        monster_kind.asset_id(),
    ))
}
