use super::constants::{
    LARGE_OVERLAY_SIZE_RATIO, LARGE_TEXT_SIZE_RATIO, OVERLAY_SIZE_RATIO, RANK_OVERLAY_SIZE_RATIO,
    SMALL_TEXT_SIZE_RATIO, TEXT_SIZE_RATIO,
};
use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconAttribute, IconAttributePosition, IconKind, IconSize},
    theme::typography::{FontSize, TextAlign, headline},
};
use namui::*;

/// 오버레이 위치를 계산하는 열거형
#[derive(Clone, Copy)]
pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl OverlayPosition {
    pub fn calculate_position(self, container_size: Wh<Px>, overlay_size: Wh<Px>) -> Xy<Px> {
        match self {
            OverlayPosition::TopLeft => Xy::new(0.px(), 0.px()),
            OverlayPosition::TopRight => Xy::new(container_size.width - overlay_size.width, 0.px()),
            OverlayPosition::BottomLeft => {
                Xy::new(0.px(), container_size.height - overlay_size.height)
            }
            OverlayPosition::BottomRight => Xy::new(
                container_size.width - overlay_size.width,
                container_size.height - overlay_size.height,
            ),
        }
    }
}

/// 텍스트 오버레이를 렌더링하는 함수
pub fn render_text_overlay(
    container_size: Wh<Px>,
    text: &str,
    position: OverlayPosition,
    size_ratio: f32,
    text_size_ratio: f32,
) -> RenderingTree {
    let overlay_size = container_size * size_ratio;
    let overlay_position = position.calculate_position(container_size, overlay_size);

    namui::translate(
        overlay_position.x,
        overlay_position.y,
        headline(text)
            .align(TextAlign::Center { wh: overlay_size })
            .size(FontSize::Custom {
                size: overlay_size.height * text_size_ratio,
            })
            .color(Color::WHITE)
            .stroke(overlay_size.height * text_size_ratio * 0.05, Color::BLACK)
            .build()
            .into_rendering_tree(),
    )
}

/// 카운트 오버레이를 렌더링하는 함수
pub fn render_count_overlay(container_size: Wh<Px>, count: usize) -> RenderingTree {
    render_text_overlay(
        container_size,
        &count.to_string(),
        OverlayPosition::BottomRight,
        OVERLAY_SIZE_RATIO * 0.75, // 카운트는 조금 작게
        TEXT_SIZE_RATIO,
    )
}

/// 플러스 아이콘 오버레이를 렌더링하는 함수
pub fn render_plus_overlay(container_size: Wh<Px>) -> RenderingTree {
    render_icon_overlay(
        container_size,
        IconKind::Add,
        OverlayPosition::BottomRight,
        OVERLAY_SIZE_RATIO,
    )
}

/// 아이콘 오버레이를 렌더링하는 함수
pub fn render_icon_overlay(
    container_size: Wh<Px>,
    icon_kind: IconKind,
    position: OverlayPosition,
    size_ratio: f32,
) -> RenderingTree {
    let overlay_size = container_size * size_ratio;
    let overlay_position = position.calculate_position(container_size, overlay_size);

    namui::translate(
        overlay_position.x,
        overlay_position.y,
        Icon::new(icon_kind)
            .wh(overlay_size)
            .size(IconSize::Custom {
                size: overlay_size.width,
            })
            .to_rendering_tree(),
    )
}

/// 랭크 오버레이를 렌더링하는 함수
pub fn render_rank_overlay(container_size: Wh<Px>, rank: Rank) -> RenderingTree {
    render_text_overlay(
        container_size,
        &rank.to_string(),
        OverlayPosition::BottomRight,
        RANK_OVERLAY_SIZE_RATIO,
        TEXT_SIZE_RATIO,
    )
}

/// 슈트 오버레이를 렌더링하는 함수
pub fn render_suit_overlay(container_size: Wh<Px>, suit: Suit) -> RenderingTree {
    render_icon_overlay(
        container_size,
        IconKind::Suit { suit },
        OverlayPosition::BottomRight,
        RANK_OVERLAY_SIZE_RATIO,
    )
}

/// 확장 표시기를 렌더링하는 함수
pub fn render_expansion_indicator(container_size: Wh<Px>, text: &str) -> RenderingTree {
    render_text_overlay(
        container_size,
        text,
        OverlayPosition::BottomLeft,
        LARGE_OVERLAY_SIZE_RATIO,
        SMALL_TEXT_SIZE_RATIO,
    )
}

/// 낮은 카드 표시기를 렌더링하는 함수
pub fn render_low_card_indicator(container_size: Wh<Px>) -> RenderingTree {
    render_text_overlay(
        container_size,
        "≤3",
        OverlayPosition::BottomLeft,
        OVERLAY_SIZE_RATIO,
        TEXT_SIZE_RATIO,
    )
}

/// 리롤 없음 표시기를 렌더링하는 함수
pub fn render_no_reroll_indicator(container_size: Wh<Px>) -> RenderingTree {
    let indicator_size = container_size * OVERLAY_SIZE_RATIO;
    let indicator_position =
        OverlayPosition::BottomLeft.calculate_position(container_size, indicator_size);

    namui::translate(
        indicator_position.x,
        indicator_position.y,
        Icon::new(IconKind::Refresh)
            .wh(indicator_size)
            .size(IconSize::Custom {
                size: indicator_size.width,
            })
            .attributes(vec![
                IconAttribute::new(IconKind::Reject).position(IconAttributePosition::Center),
            ])
            .to_rendering_tree(),
    )
}

/// 리롤 허용 표시기를 렌더링하는 함수
pub fn render_reroll_indicator(container_size: Wh<Px>) -> RenderingTree {
    let indicator_size = container_size * OVERLAY_SIZE_RATIO;
    let indicator_position =
        OverlayPosition::BottomLeft.calculate_position(container_size, indicator_size);

    namui::translate(
        indicator_position.x,
        indicator_position.y,
        Icon::new(IconKind::Refresh)
            .wh(indicator_size)
            .size(IconSize::Custom {
                size: indicator_size.width,
            })
            .attributes(vec![
                IconAttribute::new(IconKind::Accept).position(IconAttributePosition::Center),
            ])
            .to_rendering_tree(),
    )
}

/// 짝수/홀수 표시기를 렌더링하는 함수
pub fn render_even_odd_indicator(container_size: Wh<Px>, is_even: bool) -> RenderingTree {
    let text = if is_even { "Even" } else { "Odd" };

    render_text_overlay(
        container_size,
        text,
        OverlayPosition::BottomLeft,
        OVERLAY_SIZE_RATIO,
        SMALL_TEXT_SIZE_RATIO,
    )
}

/// 페이스/숫자 표시기를 렌더링하는 함수
pub fn render_face_number_indicator(container_size: Wh<Px>, is_face: bool) -> RenderingTree {
    let text = if is_face { "Face" } else { "Num" };

    render_text_overlay(
        container_size,
        text,
        OverlayPosition::BottomLeft,
        OVERLAY_SIZE_RATIO,
        SMALL_TEXT_SIZE_RATIO,
    )
}

/// 단축키 표시기를 렌더링하는 함수
pub fn render_shortcut_indicator(container_size: Wh<Px>, text: &str) -> RenderingTree {
    render_text_overlay(
        container_size,
        text,
        OverlayPosition::TopLeft,
        LARGE_OVERLAY_SIZE_RATIO,
        TEXT_SIZE_RATIO,
    )
}

/// 건너뛰기 표시기를 렌더링하는 함수
pub fn render_skip_indicator(container_size: Wh<Px>) -> RenderingTree {
    render_text_overlay(
        container_size,
        "Skip",
        OverlayPosition::TopLeft,
        OVERLAY_SIZE_RATIO,
        SMALL_TEXT_SIZE_RATIO,
    )
}

/// 같은 슈트 표시기를 렌더링하는 함수
pub fn render_same_suits_indicator(container_size: Wh<Px>) -> RenderingTree {
    render_text_overlay(
        container_size,
        "=",
        OverlayPosition::TopLeft,
        OVERLAY_SIZE_RATIO,
        LARGE_TEXT_SIZE_RATIO,
    )
}

/// 새 아이템 표시기를 렌더링하는 함수
pub fn render_new_indicator(container_size: Wh<Px>) -> RenderingTree {
    render_icon_overlay(
        container_size,
        IconKind::New,
        OverlayPosition::TopLeft,
        OVERLAY_SIZE_RATIO,
    )
}
