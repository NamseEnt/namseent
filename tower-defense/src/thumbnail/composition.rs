use super::{base_rendering, overlay_rendering};
use namui::*;

/// 썸네일 구성을 위한 빌더 패턴 구조체
pub struct ThumbnailComposer {
    width_height: Wh<Px>,
    base_layer: Option<RenderingTree>,
    overlay_layers: Vec<RenderingTree>,
}

impl ThumbnailComposer {
    /// 새로운 썸네일 구성자를 생성합니다.
    pub fn new(width_height: Wh<Px>) -> Self {
        Self {
            width_height,
            base_layer: None,
            overlay_layers: Vec::new(),
        }
    }

    /// 특정 타워 이미지를 설정합니다.
    pub fn with_tower_image(mut self, tower_kind: crate::game_state::tower::TowerKind) -> Self {
        self.base_layer = Some(base_rendering::render_tower_image(
            self.width_height,
            tower_kind,
        ));
        self
    }

    /// 아이콘을 기본 레이어로 설정합니다.
    pub fn with_icon_base(mut self, icon_kind: crate::icon::IconKind) -> Self {
        self.base_layer = Some(base_rendering::render_icon_base(
            self.width_height,
            icon_kind,
        ));
        self
    }

    /// 플러스 오버레이를 추가합니다.
    pub fn add_plus_overlay(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_plus_overlay(self.width_height));
        self
    }

    /// 랭크 오버레이를 추가합니다.
    pub fn add_rank_overlay(mut self, rank: crate::card::Rank) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_rank_overlay(
                self.width_height,
                rank,
            ));
        self
    }

    /// 슈트 오버레이를 추가합니다.
    pub fn add_suit_overlay(mut self, suit: crate::card::Suit) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_suit_overlay(
                self.width_height,
                suit,
            ));
        self
    }

    /// 아이콘 오버레이를 추가합니다.
    pub fn add_icon_overlay(
        mut self,
        icon_kind: crate::icon::IconKind,
        position: overlay_rendering::OverlayPosition,
        size_ratio: f32,
    ) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_icon_overlay(
                self.width_height,
                icon_kind,
                position,
                size_ratio,
            ));
        self
    }

    /// 최종 렌더링 트리를 생성합니다.
    pub fn build(self) -> RenderingTree {
        let mut layers = Vec::new();

        layers.extend(self.overlay_layers);

        if let Some(base) = self.base_layer {
            layers.push(base);
        }

        namui::render(layers)
    }
}
