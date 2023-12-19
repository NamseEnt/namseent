use super::*;
use crate::typography::adjust_font_size;

pub struct TextCell {
    text: String,
    on_change: Option<Closure<dyn Fn(&String)>>,
    font_size: Option<IntPx>,
    borders: Borders,
}
pub fn text(text: impl AsRef<str>) -> TextCell {
    TextCell {
        text: text.as_ref().to_string(),
        on_change: None,
        font_size: None,
        borders: Borders::new(),
    }
}
impl TextCell {
    pub fn on_change(self, on_change: impl IntoClosure<dyn Fn(&String)>) -> Self {
        Self {
            on_change: Some(on_change.into_arc()),
            ..self
        }
    }
    pub fn font_size(self, font_size: IntPx) -> Self {
        Self {
            font_size: Some(font_size),
            ..self
        }
    }
    pub fn borders(mut self, side: Side, line: Line) -> Self {
        self.borders.add(side, line);
        self
    }
    pub fn build(self) -> Cell {
        Cell::new(Box::new(self))
    }
}
impl CellTrait for TextCell {
    fn render(&self, props: Props) -> RenderingTree {
        let text_color = if props.is_selected {
            props.color_palette.selected_text_color
        } else {
            props.color_palette.text_color
        };

        let font_size = self
            .font_size
            .unwrap_or_else(|| adjust_font_size(props.wh.height));

        match self.on_change.as_ref() {
            Some(text_input_on_change) if props.is_editing => {
                let text_input_on_change = text_input_on_change.clone();
                props.text_input.render(text_input::Props {
                    rect: Rect::from_xy_wh(Xy::zero(), props.wh),
                    text: self.text.clone(),
                    text_align: TextAlign::Center,
                    text_baseline: TextBaseline::Middle,
                    font: Font {
                        serif: false,
                        size: font_size,
                        language: Language::Ko,
                        font_weight: FontWeight::REGULAR,
                    },
                    style: text_input::Style {
                        text: TextStyle {
                            color: text_color,
                            ..Default::default()
                        },
                        rect: RectStyle {
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    event_handler: Some(text_input::EventHandler::new().on_text_updated(
                        move |text| {
                            text_input_on_change(text);
                        },
                    )),
                })
            }
            _ => namui::text(TextParam {
                text: self.text.clone(),
                x: props.wh.width / 2.0,
                y: props.wh.height / 2.0,
                align: TextAlign::Center,
                baseline: TextBaseline::Middle,
                font: Font {
                    font_weight: FontWeight::REGULAR,
                    language: Language::Ko,
                    serif: false,
                    size: font_size,
                },
                style: TextStyle {
                    color: text_color,
                    ..Default::default()
                },
                max_width: Some(props.wh.width),
            }),
        }
    }

    fn borders(&self) -> &Borders {
        &self.borders
    }

    fn copy(&self) -> ClipboardItem {
        ClipboardItem::Text(self.text.clone())
    }

    fn on_paste(&self) -> Option<Closure<dyn Fn(ClipboardItem)>> {
        if let Some(on_change) = self.on_change.clone() {
            Some(closure(move |clipboard_item: ClipboardItem| {
                if let ClipboardItem::Text(text) = clipboard_item {
                    on_change(&text);
                }
            }))
        } else {
            None
        }
    }
}
