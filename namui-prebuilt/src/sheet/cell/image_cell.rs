use super::*;

pub struct ImageCell {
    image_source: ImageSource,
    borders: Borders,
}
pub fn image(image_source: ImageSource) -> ImageCell {
    ImageCell {
        image_source,
        borders: Borders::new(),
    }
}

impl ImageCell {
    pub fn borders(mut self, side: Side, line: Line) -> Self {
        self.borders.add(side, line);
        self
    }
    pub fn build(self) -> Cell {
        Cell::new(Box::new(self))
    }
}

impl CellTrait for ImageCell {
    fn render(&self, props: Props) -> RenderingTree {
        namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), props.wh),
            source: self.image_source.clone(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint_builder: None,
            },
        })
    }

    fn borders(&self) -> &Borders {
        &self.borders
    }

    fn copy(&self) -> ClipboardItem {
        ClipboardItem::Image(self.image_source.clone())
    }

    fn on_paste(&self) -> Option<Arc<dyn Fn(ClipboardItem)>> {
        None
    }
}
