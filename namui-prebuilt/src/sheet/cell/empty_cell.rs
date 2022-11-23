use super::*;

pub struct EmptyCell {
    on_edit: Option<Box<dyn Fn()>>,
    borders: Borders,
}
pub fn empty() -> EmptyCell {
    EmptyCell {
        on_edit: None,
        borders: Borders::new(),
    }
}
impl Cell for EmptyCell {
    fn render(&self, _props: Props) -> RenderingTree {
        RenderingTree::Empty
    }

    fn borders(&self) -> &Borders {
        &self.borders
    }

    fn copy(&self) -> ClipboardItem {
        ClipboardItem::Empty
    }

    fn on_paste(&self) -> Option<Arc<dyn Fn(ClipboardItem)>> {
        None
    }
}
impl EmptyCell {
    pub fn borders(mut self, side: Side, line: Line) -> Self {
        self.borders.add(side, line);
        self
    }
}
impl Into<Box<dyn Cell>> for EmptyCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
