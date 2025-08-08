use crate::{
    asset_loader,
    game_state::tower::{AnimationKind, TowerKind},
};
use namui::*;

pub fn render_barricade_tower(wh: Wh<Px>) -> RenderingTree {
    asset_loader::get_tower_asset((TowerKind::Barricade, AnimationKind::Idle1))
        .map(|image| {
            namui::image(ImageParam {
                rect: wh.to_rect(),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
        })
        .unwrap_or_default()
}

pub fn render_tower_kind_base(wh: Wh<Px>, tower_kind: TowerKind) -> RenderingTree {
    asset_loader::get_tower_asset((tower_kind, AnimationKind::Idle1))
        .map(|image| {
            namui::image(ImageParam {
                rect: wh.to_rect(),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
        })
        .unwrap_or_default()
}
