mod stat;
mod tower_preview_component;
mod tower_skill;
mod upgrade_helpers;

pub use tower_preview_component::{TowerPreview, TowerPreviewContent};

/// Format a number with suffixes like k, m, b keeping one decimal place.
/// Examples: 950.0 -> "950.0", 1500.0 -> "1.5k", 2_300_000.0 -> "2.3m"
pub(super) fn format_compact_number(value: f32) -> String {
    if value >= 1_000_000_000.0 {
        format!("{:.1}b", value / 1_000_000_000.0)
    } else if value >= 1_000_000.0 {
        format!("{:.1}m", value / 1_000_000.0)
    } else if value >= 1_000.0 {
        format!("{:.1}k", value / 1_000.0)
    } else {
        format!("{:.1}", value)
    }
}
