use crate::typography::adjust_font_size;
use namui::prelude::*;

pub trait Cell {
    fn render(&self, wh: Wh<Px>) -> RenderingTree;
}
pub struct EmptyCell {
    on_edit: Option<Box<dyn Fn()>>,
}
pub struct TextCell {
    text: String,
    text_input_on_change: Option<Box<dyn Fn(&str)>>,
    on_edit: Option<Box<dyn Fn()>>,
    font_size: Option<IntPx>,
}
pub struct ImageCell {
    image_source: ImageSource,
    on_edit: Option<Box<dyn Fn()>>,
}
pub fn empty() -> EmptyCell {
    EmptyCell { on_edit: None }
}
pub fn text(text: impl AsRef<str>) -> TextCell {
    TextCell {
        text: text.as_ref().to_string(),
        text_input_on_change: None,
        on_edit: None,
        font_size: None,
    }
}
pub fn image(image_source: ImageSource) -> ImageCell {
    ImageCell {
        image_source,
        on_edit: None,
    }
}
impl TextCell {
    pub fn edit_with_text_input(self, text_input_on_change: impl Fn(&str) + 'static) -> Self {
        Self {
            text_input_on_change: Some(Box::new(text_input_on_change)),
            ..self
        }
    }
    pub fn font_size(self, font_size: IntPx) -> Self {
        Self {
            font_size: Some(font_size),
            ..self
        }
    }
}
impl ImageCell {
    pub fn on_edit(self, callback: impl Fn() + 'static) -> Self {
        Self {
            on_edit: Some(Box::new(callback)),
            ..self
        }
    }
}

impl Cell for EmptyCell {
    fn render(&self, _wh: Wh<Px>) -> RenderingTree {
        RenderingTree::Empty
    }
}
impl Cell for TextCell {
    fn render(&self, wh: Wh<Px>) -> RenderingTree {
        let font_size = self
            .font_size
            .unwrap_or_else(|| adjust_font_size(wh.height));

        namui::text(TextParam {
            text: self.text.clone(),
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: font_size,
            },
            style: TextStyle {
                color: Color::WHITE,
                ..Default::default()
            },
            max_width: Some(wh.width),
        })
    }
}
impl Cell for ImageCell {
    fn render(&self, wh: Wh<Px>) -> RenderingTree {
        namui::image(ImageParam {
            rect: Rect::from_xy_wh(Xy::zero(), wh),
            source: self.image_source.clone(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint_builder: None,
            },
        })
    }
}

impl Into<Box<dyn Cell>> for EmptyCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
impl Into<Box<dyn Cell>> for TextCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
impl Into<Box<dyn Cell>> for ImageCell {
    fn into(self) -> Box<dyn Cell> {
        Box::new(self)
    }
}
