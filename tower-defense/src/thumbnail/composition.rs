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

    /// 기본 타워 이미지를 설정합니다.
    pub fn with_default_tower(mut self) -> Self {
        self.base_layer = Some(base_rendering::render_default_tower(self.width_height));
        self
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

    /// 카운트 오버레이를 추가합니다.
    pub fn add_count_overlay(mut self, count: usize) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_count_overlay(
                self.width_height,
                count,
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

    /// 새 아이템 표시기를 추가합니다.
    pub fn add_new_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_new_indicator(self.width_height));
        self
    }

    /// 확장 표시기를 추가합니다.
    pub fn add_expansion_indicator(mut self, text: &str) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_expansion_indicator(
                self.width_height,
                text,
            ));
        self
    }

    /// 낮은 카드 표시기를 추가합니다.
    pub fn add_low_card_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_low_card_indicator(
                self.width_height,
            ));
        self
    }

    /// 리롤 없음 표시기를 추가합니다.
    pub fn add_no_reroll_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_no_reroll_indicator(
                self.width_height,
            ));
        self
    }

    /// 리롤 허용 표시기를 추가합니다.
    pub fn add_reroll_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_reroll_indicator(
                self.width_height,
            ));
        self
    }

    /// 짝수/홀수 표시기를 추가합니다.
    pub fn add_even_odd_indicator(mut self, is_even: bool) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_even_odd_indicator(
                self.width_height,
                is_even,
            ));
        self
    }

    /// 페이스/숫자 표시기를 추가합니다.
    pub fn add_face_number_indicator(mut self, is_face: bool) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_face_number_indicator(
                self.width_height,
                is_face,
            ));
        self
    }

    /// 단축키 표시기를 추가합니다.
    pub fn add_shortcut_indicator(mut self, text: &str) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_shortcut_indicator(
                self.width_height,
                text,
            ));
        self
    }

    /// 건너뛰기 표시기를 추가합니다.
    pub fn add_skip_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_skip_indicator(self.width_height));
        self
    }

    /// 같은 슈트 표시기를 추가합니다.
    pub fn add_same_suits_indicator(mut self) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_same_suits_indicator(
                self.width_height,
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

    /// 텍스트 오버레이를 추가합니다.
    #[allow(dead_code)]
    pub fn add_text_overlay(
        mut self,
        text: &str,
        position: overlay_rendering::OverlayPosition,
        size_ratio: f32,
        background_color: Color,
        text_size_ratio: f32,
    ) -> Self {
        self.overlay_layers
            .push(overlay_rendering::render_text_overlay(
                self.width_height,
                text,
                position,
                size_ratio,
                background_color,
                text_size_ratio,
            ));
        self
    }

    /// 최종 렌더링 트리를 생성합니다.
    pub fn build(self) -> RenderingTree {
        let mut layers = Vec::new();

        // 기본 레이어를 먼저 추가 (배경에 렌더링됨 - 타워가 가장 아래)
        if let Some(base) = self.base_layer {
            layers.push(base);
        }

        // 오버레이 레이어들을 나중에 추가 (위에 렌더링됨)
        layers.extend(self.overlay_layers);

        namui::render(layers)
    }
}
