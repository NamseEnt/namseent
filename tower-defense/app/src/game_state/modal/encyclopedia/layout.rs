use super::catalog::{Entry, EntryKind};
use crate::rarity::Rarity;
use namui::*;

pub(super) const VERTICAL_MARGIN: Px = px(24.0);
pub(super) const PADDING: Px = px(8.0);
pub(super) const PANEL_PADDING: Px = px(16.0);
pub(super) const GROUP_GAP: Px = px(20.0);
pub(super) const RARITY_GAP: Px = px(16.0);
pub(super) const RARITY_HEADER_HEIGHT: Px = px(28.0);
pub(super) const ENTRY_SIZE: Px = px(72.0);
pub(super) const ENTRY_GAP: Px = px(8.0);
pub(super) const SCROLL_BAR_WIDTH: Px = px(8.0);
pub(super) const TAB_BAR_Y: Px = px(24.0);
pub(super) const TAB_BAR_HEIGHT: Px = px(88.0);
pub(super) const TAB_VISIBLE_HEIGHT: Px = px(80.0);
pub(super) const TAB_CONTENT_TITLE_HEIGHT: Px = px(32.0);
pub(super) const TAB_GAP: Px = px(8.0);
pub(super) const TAB_LEFT: Px = px(24.0);
pub(super) const TAB_MIN_WIDTH: Px = px(96.0);
pub(super) const TAB_MAX_WIDTH: Px = px(168.0);
pub(super) const CONTENT_PANEL_X: Px = px(12.0);
pub(super) const CONTENT_PANEL_Y: Px = px(104.0);
pub(super) const CONTENT_PANEL_BOTTOM: Px = px(16.0);
pub(super) const CONTENT_PANEL_PADDING: Px = px(12.0);
pub(super) const FAB_SIZE: Px = px(96.0);
pub(super) const FAB_PADDING: Px = px(36.0);
pub(super) const CLOSE_VISIBLE_HEIGHT: Px = px(68.0);

#[derive(Clone, Debug, PartialEq, State)]
pub(super) struct EncyclopediaLayout {
    pub(super) rarities: Vec<RarityLayout>,
    pub(super) bottom_y: Px,
}

#[derive(Clone, Copy, Debug, PartialEq, State)]
pub(super) struct TabBarLayout {
    pub(super) tab_width: Px,
}

#[derive(Clone, Debug, PartialEq, State)]
pub(super) struct RarityLayout {
    pub(super) rarity: Rarity,
    pub(super) panel_y: Px,
    pub(super) panel_width: Px,
    pub(super) panel_height: Px,
    pub(super) label_y: Px,
    pub(super) entries: Vec<EntryPlacement>,
}

#[derive(Clone, Debug, PartialEq, State)]
pub(super) struct EntryPlacement {
    pub(super) entry_index: usize,
    pub(super) x: Px,
    pub(super) y: Px,
}

pub(super) fn calculate_layout(
    entries: &[Entry],
    content_width: Px,
    selected_kind: EntryKind,
) -> EncyclopediaLayout {
    let panel_width = (content_width - PADDING * 2.0).max(ENTRY_SIZE + PANEL_PADDING * 2.0);
    let grid_width = (panel_width - PANEL_PADDING * 2.0).max(ENTRY_SIZE);
    let columns = ((grid_width + ENTRY_GAP) / (ENTRY_SIZE + ENTRY_GAP))
        .floor()
        .max(1.0) as usize;
    let occupied_grid_width = ENTRY_SIZE * columns as f32 + ENTRY_GAP * (columns - 1) as f32;
    let grid_x = PANEL_PADDING + (grid_width - occupied_grid_width).max(px(0.0)) * 0.5;
    let mut y = VERTICAL_MARGIN;
    let mut rarities = Vec::new();

    for rarity in [
        Rarity::Common,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
    ] {
        let rarity_entry_indices: Vec<usize> = entries
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| {
                (entry.kind == selected_kind && entry.rarity == rarity).then_some(index)
            })
            .collect();
        if rarity_entry_indices.is_empty() {
            continue;
        }

        let rows = rarity_entry_indices.len().div_ceil(columns);
        let panel_y = y;
        let label_y = panel_y + PANEL_PADDING;
        let entry_start_y = label_y + RARITY_HEADER_HEIGHT + RARITY_GAP;
        let panel_height = PANEL_PADDING
            + RARITY_HEADER_HEIGHT
            + RARITY_GAP
            + (ENTRY_SIZE + ENTRY_GAP) * rows as f32
            - ENTRY_GAP
            + PANEL_PADDING;
        let placements = rarity_entry_indices
            .iter()
            .enumerate()
            .map(|(index, entry_index)| {
                let row = index / columns;
                let column = index % columns;
                EntryPlacement {
                    entry_index: *entry_index,
                    x: PADDING + grid_x + (ENTRY_SIZE + ENTRY_GAP) * column as f32,
                    y: entry_start_y + (ENTRY_SIZE + ENTRY_GAP) * row as f32,
                }
            })
            .collect();

        rarities.push(RarityLayout {
            rarity,
            panel_y,
            panel_width,
            panel_height,
            label_y,
            entries: placements,
        });
        y += panel_height + GROUP_GAP;
    }

    EncyclopediaLayout {
        rarities,
        bottom_y: y + VERTICAL_MARGIN,
    }
}
