use std::collections::BTreeMap;

use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, bincode::Encode, bincode::Decode)]
pub struct MouseCursorNode {
    pub cursor: Box<MouseCursor>,
    pub rendering_tree: Box<RenderingTree>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode)]
pub enum MouseCursor {
    Standard(StandardCursor),
    Custom(RenderingTree),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, bincode::Encode, bincode::Decode)]
pub enum StandardCursor {
    #[default]
    Default,
    Pointer,
    Wait,
    Progress,
    Help,
    Text,
    VerticalText,
    NResize,
    SResize,
    EResize,
    WResize,
    NeResize,
    NwResize,
    SeResize,
    SwResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    Move,
    AllScroll,
    Grab,
    Copy,
    Alias,
    NoDrop,
    NotAllowed,
    Crosshair,
    Cell,
    ContextMenu,
    ZoomIn,
    ZoomOut,
    ColorPicker,
    Pencil,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
}
impl StandardCursor {
    pub fn from_css_cursor_value(value: &str) -> Option<Self> {
        Some(match value {
            "default" => StandardCursor::Default,
            "pointer" => StandardCursor::Pointer,
            "wait" => StandardCursor::Wait,
            "progress" => StandardCursor::Progress,
            "help" => StandardCursor::Help,
            "text" => StandardCursor::Text,
            "vertical-text" => StandardCursor::VerticalText,
            "n-resize" => StandardCursor::NResize,
            "s-resize" => StandardCursor::SResize,
            "e-resize" => StandardCursor::EResize,
            "w-resize" => StandardCursor::WResize,
            "ne-resize" => StandardCursor::NeResize,
            "nw-resize" => StandardCursor::NwResize,
            "se-resize" => StandardCursor::SeResize,
            "sw-resize" => StandardCursor::SwResize,
            "ew-resize" => StandardCursor::EwResize,
            "ns-resize" => StandardCursor::NsResize,
            "nesw-resize" => StandardCursor::NeswResize,
            "nwse-resize" => StandardCursor::NwseResize,
            "col-resize" => StandardCursor::ColResize,
            "row-resize" => StandardCursor::RowResize,
            "move" => StandardCursor::Move,
            "all-scroll" => StandardCursor::AllScroll,
            "grab" => StandardCursor::Grab,
            "copy" => StandardCursor::Copy,
            "alias" => StandardCursor::Alias,
            "no-drop" => StandardCursor::NoDrop,
            "not-allowed" => StandardCursor::NotAllowed,
            "crosshair" => StandardCursor::Crosshair,
            "cell" => StandardCursor::Cell,
            "context-menu" => StandardCursor::ContextMenu,
            "zoom-in" => StandardCursor::ZoomIn,
            "zoom-out" => StandardCursor::ZoomOut,
            "color-picker" => StandardCursor::ColorPicker,
            "pencil" => StandardCursor::Pencil,
            "up-arrow" => StandardCursor::UpArrow,
            "down-arrow" => StandardCursor::DownArrow,
            "left-arrow" => StandardCursor::LeftArrow,
            "right-arrow" => StandardCursor::RightArrow,
            _ => return None,
        })
    }
}

/// Metadata format:
/// - first line: number of columns, number of rows, cursor width, cursor height
/// - each subsequent line: cursor name, hotspot x, hotspot y, frame count (optional), frame duration (optional)
///
/// All cursor name is for CSS cursor value.
///
/// ```text
/// {columns} {rows} {cursor width} {cursor height}
/// {cursor name} {hotspot x} {hotspot y} {frame count} {frame duration} // for animated cursors
/// {cursor name} {hotspot x} {hotspot y} // for static cursors
/// ...
/// ```
pub struct StandardCursorSpriteSet {
    pub sheet: Image,
    pub columns: usize,
    pub rows: usize,
    pub cursor_wh: Wh<Px>,
    pub sprites: BTreeMap<StandardCursor, CursorSprite>,
}

impl StandardCursorSpriteSet {
    pub fn parse(sheet: Image, metadata_text: &str) -> Result<Self> {
        let lines = &mut metadata_text.lines();

        let line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty metadata"))?;

        let parts = &mut line.split_whitespace();

        let columns = parse_parts(parts, "columns")?;
        let rows = parse_parts(parts, "rows")?;
        let cursor_wh = Wh::new(
            parse_parts::<i32>(parts, "cursor width")?.px(),
            parse_parts::<i32>(parts, "cursor height")?.px(),
        );

        let mut sprites = BTreeMap::new();

        let mut index = 0;

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let parts = &mut line.split_whitespace().peekable();
            let name = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing cursor name"))?;
            let Some(standard_cursor) = StandardCursor::from_css_cursor_value(name) else {
                println!("Unknown cursor name: {name} at line {index}, skipping.",);
                continue;
            };
            let hotspot_xy = Xy::new(
                parse_parts::<i32>(parts, "hotspot x")?.px(),
                parse_parts::<i32>(parts, "hotspot y")?.px(),
            );
            let animation_frame_count_duration = if parts.peek().is_some() {
                Some((
                    parse_parts::<usize>(parts, "frame count")?,
                    parse_parts::<i32>(parts, "frame duration")?.ms(),
                ))
            } else {
                None
            };
            sprites.insert(
                standard_cursor,
                if let Some((frame_count, frame_duration)) = animation_frame_count_duration {
                    CursorSprite::Animated {
                        start_index: index,
                        hotspot_xy,
                        frame_count,
                        frame_duration,
                    }
                } else {
                    CursorSprite::Static { index, hotspot_xy }
                },
            );

            match animation_frame_count_duration {
                Some((frame_count, _)) => {
                    index += frame_count;
                }
                None => {
                    index += 1;
                }
            }
        }

        Ok(StandardCursorSpriteSet {
            sheet,
            columns,
            rows,
            cursor_wh,
            sprites,
        })
    }
}

pub enum CursorSprite {
    Static {
        index: usize,
        hotspot_xy: Xy<Px>,
    },
    Animated {
        start_index: usize,
        hotspot_xy: Xy<Px>,
        frame_count: usize,
        frame_duration: Duration,
    },
}

fn parse_parts<'a, T: std::str::FromStr>(
    parts: &mut impl Iterator<Item = &'a str>,
    name: &str,
) -> Result<T> {
    parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing {name}"))?
        .parse::<T>()
        .map_err(|_| anyhow::anyhow!("Failed to parse {name}"))
}
