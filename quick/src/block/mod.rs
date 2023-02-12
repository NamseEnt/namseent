mod row_block;

use super::*;
pub(crate) use row_block::*;

pub struct Block {
    pub(crate) title: String,
    pub(crate) lines: Vec<Line>,
}

impl Block {
    pub(crate) fn vertical_margins(&self) -> Px {
        BLOCK_OUTER_TOP_MARGIN + BLOCK_OUTER_BOTTOM_MARGIN + BLOCK_INNER_MARGIN * 2
    }
}

pub fn block<'a>(title: impl ToString, lines: impl IntoIterator<Item = Line>) -> GroupItem {
    GroupItem::Block(Block {
        title: title.to_string(),
        lines: lines.into_iter().collect(),
    })
}
