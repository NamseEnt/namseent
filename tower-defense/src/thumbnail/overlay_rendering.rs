use super::constants::{OVERLAY_SIZE_RATIO, RANK_OVERLAY_SIZE_RATIO, TEXT_SIZE_RATIO};
use crate::{
    card::{Rank, Suit},
    icon::{Icon, IconKind, IconSize},
    theme::typography::{self, FontSize},
};
use namui::*;

/// 오버레이 위치를 계산하는 열거형
#[derive(Clone, Copy, State)]
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

    let rendered_text = typography::TypographyBuilder::new()
        .headline()
        .size(FontSize::Custom {
            size: overlay_size.height * text_size_ratio,
        })
        .color(Color::WHITE)
        .stroke(overlay_size.height * text_size_ratio * 0.05, Color::BLACK)
        .static_text(text)
        .render();

    let text_offset = Xy {
        x: (overlay_size.width - rendered_text.width) / 2.0,
        y: (overlay_size.height - rendered_text.height) / 2.0,
    };

    // RenderedRichText::into_rendering_tree() to get the RenderingTree
    namui::translate(
        overlay_position.x + text_offset.x,
        overlay_position.y + text_offset.y,
        rendered_text.into_rendering_tree(),
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
        OverlayPosition::TopLeft,
        RANK_OVERLAY_SIZE_RATIO,
        TEXT_SIZE_RATIO,
    )
}

/// 슈트 오버레이를 렌더링하는 함수
pub fn render_suit_overlay(container_size: Wh<Px>, suit: Suit) -> RenderingTree {
    render_icon_overlay(
        container_size,
        IconKind::Suit { suit },
        OverlayPosition::TopLeft,
        RANK_OVERLAY_SIZE_RATIO,
    )
}

