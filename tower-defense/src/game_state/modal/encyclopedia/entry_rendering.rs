use super::catalog::{Entry, EntryContent, EntryKind};
use super::layout::{
    ENTRY_SIZE, EncyclopediaLayout, PADDING, PANEL_PADDING, RARITY_GAP, RARITY_HEADER_HEIGHT,
    VERTICAL_MARGIN,
};
use crate::game_state::discovery::DiscoveryState;
use crate::l10n::rarity::RarityText;
use crate::l10n::ui::EncyclopediaProgressText;
use crate::rarity::Rarity;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::{
    palette,
    typography::{FontSize, memoized_text},
};
use crate::thumbnail::{ThumbnailRenderOptions, render_thumbnail};
use crate::tooltip::{TooltipContent, TooltipPlacement, WithHoverArea};
use namui::*;
use namui_prebuilt::simple_rect;

#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
struct RarityStats {
    kind: EntryKind,
    rarity: Rarity,
    discovered: usize,
    total: usize,
}

#[derive(Clone, Debug, PartialEq, State)]
pub(super) struct CollectionStats {
    rarities: Vec<RarityStats>,
}

impl CollectionStats {
    pub(super) fn from_entries(entries: &[Entry], discovered_entries: &[bool]) -> Self {
        let mut stats = Self {
            rarities: Vec::new(),
        };

        for (entry, &is_discovered) in entries.iter().zip(discovered_entries) {
            if let Some(rarity) = stats
                .rarities
                .iter_mut()
                .find(|stat| stat.kind == entry.kind && stat.rarity == entry.rarity)
            {
                rarity.total += 1;
                if is_discovered {
                    rarity.discovered += 1;
                }
            } else {
                stats.rarities.push(RarityStats {
                    kind: entry.kind,
                    rarity: entry.rarity,
                    discovered: usize::from(is_discovered),
                    total: 1,
                });
            }
        }

        stats
    }

    fn rarity(&self, kind: EntryKind, rarity: Rarity) -> RarityStats {
        self.rarities
            .iter()
            .find(|stat| stat.kind == kind && stat.rarity == rarity)
            .copied()
            .unwrap_or(RarityStats {
                kind,
                rarity,
                discovered: 0,
                total: 0,
            })
    }
}

pub(super) fn render_entries(
    ctx: &ComposeCtx,
    entries: &[Entry],
    layout: &EncyclopediaLayout,
    discovered_entries: &[bool],
    stats: &CollectionStats,
    locale: crate::l10n::Locale,
) {
    for rarity_layout in &layout.rarities {
        let rarity = rarity_layout.rarity;
        let rarity_stats = stats.rarity(entries[rarity_layout.entries[0].entry_index].kind, rarity);
        ctx.translate((PADDING + PANEL_PADDING, rarity_layout.label_y))
            .add(memoized_text(
                (&rarity, &rarity_stats, &locale),
                |mut builder| {
                    builder
                        .headline()
                        .bold()
                        .size(FontSize::Medium)
                        .color(rarity.color())
                        .l10n(RarityText::from(rarity), &locale)
                        .text("  ")
                        .l10n(
                            EncyclopediaProgressText {
                                discovered: rarity_stats.discovered,
                                total: rarity_stats.total,
                            },
                            &locale,
                        )
                        .render_left_top()
                },
            ));

        for placement in &rarity_layout.entries {
            render_entry(
                ctx.translate((placement.x, placement.y)),
                &entries[placement.entry_index],
                discovered_entries[placement.entry_index],
            );
        }
    }

    for rarity_layout in &layout.rarities {
        let title_bottom = rarity_layout.label_y + RARITY_HEADER_HEIGHT + RARITY_GAP * 0.5;
        let title_height = title_bottom - rarity_layout.panel_y;
        let content_height = rarity_layout.panel_height - title_height;

        ctx.translate((PADDING, rarity_layout.panel_y))
            .add(PaperContainerBackground {
                width: rarity_layout.panel_width,
                height: title_height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::PaperSingleLayer,
                color: palette::SURFACE_CONTAINER,
                outline_color: Some(palette::OUTLINE.with_alpha(180)),
                shadow: false,
                arrow: None,
            });
        ctx.translate((PADDING, title_bottom))
            .add(PaperContainerBackground {
                width: rarity_layout.panel_width,
                height: content_height,
                texture: PaperTexture::Rough,
                variant: PaperVariant::PaperSingleLayer,
                color: palette::SURFACE_CONTAINER_HIGH,
                outline_color: Some(palette::OUTLINE.with_alpha(180)),
                shadow: true,
                arrow: None,
            });
    }

    ctx.translate((0.px(), layout.bottom_y)).add(simple_rect(
        Wh::new(1.px(), VERTICAL_MARGIN),
        Color::TRANSPARENT,
        0.px(),
        Color::TRANSPARENT,
    ));
}

pub(super) fn is_discovered(entry: &Entry, discovered: &DiscoveryState) -> bool {
    match entry.kind {
        EntryKind::Item => discovered.items.iter().any(|key| key == &entry.key),
        EntryKind::CardService => discovered.card_services.iter().any(|key| key == &entry.key),
        EntryKind::Treasure => discovered.treasures.iter().any(|key| key == &entry.key),
    }
}

fn render_entry(ctx: ComposeCtx, entry: &Entry, is_discovered: bool) {
    let thumbnail_wh = Wh::new(ENTRY_SIZE - PADDING * 2.0, ENTRY_SIZE - PADDING * 2.0);
    let source = match &entry.content {
        EntryContent::Item(item) => item.thumbnail_source(),
        EntryContent::CardService(card_service) => card_service.thumbnail_source(),
        EntryContent::Treasure(treasure) => treasure.thumbnail_source(),
    };

    ctx.compose(|ctx| {
        if is_discovered {
            ctx.translate((PADDING, PADDING)).add(render_thumbnail(
                source,
                thumbnail_wh,
                ThumbnailRenderOptions::sticker(
                    crate::thumbnail::STICKER_THUMBNAIL_STROKE,
                    true,
                    1.0,
                ),
            ));
        } else {
            let inner_ctx = ctx.translate((PADDING, PADDING));
            inner_ctx
                .compose(|ctx| {
                    ctx.add(memoized_text((), |mut builder| {
                        builder
                            .headline()
                            .bold()
                            .size(FontSize::Custom { size: px(30.0) })
                            .text("?")
                            .render_center(thumbnail_wh)
                    }));
                    ctx.add(render_thumbnail(
                        source,
                        thumbnail_wh,
                        ThumbnailRenderOptions::silhouette(Color::BLACK, 0.45),
                    ));
                })
                .add(PaperContainerBackground {
                    width: thumbnail_wh.width,
                    height: thumbnail_wh.height,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::PaperSingleLayer,
                    color: palette::SURFACE_CONTAINER_LOW,
                    outline_color: Some(palette::OUTLINE),
                    shadow: true,
                    arrow: None,
                });
        }
        ctx.add(WithHoverArea {
            component_key: format!("encyclopedia:{}", entry.key),
            component: simple_rect(
                Wh::single(ENTRY_SIZE),
                Color::TRANSPARENT,
                0.px(),
                Color::TRANSPARENT,
            ),
            placement: TooltipPlacement::Above,
            on_enter: {
                let content = if is_discovered {
                    Some(entry.tooltip())
                } else {
                    Some(TooltipContent::Undiscovered)
                };
                move || content.clone()
            },
            on_exit: || {},
        });
    });
}
