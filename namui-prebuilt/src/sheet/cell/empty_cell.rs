use super::*;

pub struct EmptyCell {
    borders: Borders,
}
pub fn empty() -> EmptyCell {
    EmptyCell {
        borders: Borders::new(),
    }
}
impl CellTrait for EmptyCell {
    fn render(&self, _props: Props) -> RenderingTree {
        RenderingTree::Empty
    }

    fn borders(&self) -> &Borders {
        &self.borders
    }

    fn copy(&self) -> ClipboardItem {
        ClipboardItem::Empty
    }

    fn on_paste(&self) -> Option<ClosurePtr<ClipboardItem, ()>> {
        None
    }
}
impl EmptyCell {
    pub fn borders(mut self, side: Side, line: Line) -> Self {
        self.borders.add(side, line);
        self
    }
    pub fn build(self) -> Cell {
        Cell::new(Box::new(self))
    }
}
