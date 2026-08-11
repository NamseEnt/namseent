use crate::theme::palette;
use crate::theme::typography::{FontSize, TypographyBuilder};
use namui::*;

const OVERLAY_OVERLAP: Px = px(8.0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThumbnailOverlayAnchor {
    RightTop,
    RightBottom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ThumbnailOverlay {
    Text {
        anchor: ThumbnailOverlayAnchor,
        text: String,
        color: Color,
    },
}

impl ThumbnailOverlay {
    pub fn right_top(text: impl Into<String>, color: Color) -> Self {
        Self::Text {
            anchor: ThumbnailOverlayAnchor::RightTop,
            text: text.into(),
            color,
        }
    }

    pub fn right_bottom(text: impl Into<String>, color: Color) -> Self {
        Self::Text {
            anchor: ThumbnailOverlayAnchor::RightBottom,
            text: text.into(),
            color,
        }
    }
}

pub fn render_thumbnail_overlay(overlay: &ThumbnailOverlay, width_height: Wh<Px>) -> RenderingTree {
    match overlay {
        ThumbnailOverlay::Text {
            anchor,
            text,
            color,
        } => {
            let positioned = match anchor {
                ThumbnailOverlayAnchor::RightTop => TypographyBuilder::new()
                    .headline()
                    .size(FontSize::Medium)
                    .color(*color)
                    .stroke(2.px(), palette::DARK_CHARCOAL)
                    .static_text(text)
                    .render_right_top(width_height.width),
                ThumbnailOverlayAnchor::RightBottom => TypographyBuilder::new()
                    .headline()
                    .size(FontSize::Medium)
                    .color(*color)
                    .stroke(2.px(), palette::DARK_CHARCOAL)
                    .static_text(text)
                    .render_right_bottom(width_height),
            };

            let offset = match anchor {
                ThumbnailOverlayAnchor::RightTop => Xy::new(
                    positioned.offset.x + OVERLAY_OVERLAP,
                    positioned.offset.y - OVERLAY_OVERLAP,
                ),
                ThumbnailOverlayAnchor::RightBottom => Xy::new(
                    positioned.offset.x + OVERLAY_OVERLAP,
                    positioned.offset.y + OVERLAY_OVERLAP,
                ),
            };
            namui::translate(offset.x, offset.y, positioned.tree)
        }
    }
}

pub fn render_thumbnail_overlays(
    overlays: &[ThumbnailOverlay],
    width_height: Wh<Px>,
) -> RenderingTree {
    namui::render(
        overlays
            .iter()
            .map(|overlay| render_thumbnail_overlay(overlay, width_height)),
    )
}
