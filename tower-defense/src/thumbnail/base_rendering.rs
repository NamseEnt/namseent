use crate::{
    asset_loader,
    game_state::tower::{AnimationKind, TowerKind},
    icon::{Icon, IconKind, IconSize},
};
use namui::*;

/// 타워 이미지를 렌더링하는 기본 함수
pub fn render_tower_image(width_height: Wh<Px>, tower_kind: TowerKind) -> RenderingTree {
    asset_loader::get_tower_asset((tower_kind, AnimationKind::Idle1))
        .map(|image| {
            namui::image(ImageParam {
                rect: width_height.to_rect(),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            })
        })
        .unwrap_or_default()
}

/// 바리케이드 타워를 렌더링하는 함수 (가장 자주 사용되는 기본 타워)
pub fn render_default_tower(width_height: Wh<Px>) -> RenderingTree {
    render_tower_image(width_height, TowerKind::Barricade)
}

/// 아이콘을 렌더링하는 기본 함수
pub fn render_icon_base(width_height: Wh<Px>, icon_kind: IconKind) -> RenderingTree {
    Icon::new(icon_kind)
        .wh(width_height)
        .size(IconSize::Custom {
            size: width_height.width,
        })
        .to_rendering_tree()
}
