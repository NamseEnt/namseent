use super::palette;
use namui::*;

pub const HEADLINE_FONT_NAME: &str = "NotoSansKR-Bold";
pub const PARAGRAPH_FONT_NAME: &str = "NotoSansKR-Regular";

pub const HEADLINE_FONT_SIZE_LARGE: IntPx = int_px(20);
pub const HEADLINE_FONT_SIZE_MEDIUM: IntPx = int_px(16);
pub const HEADLINE_FONT_SIZE_SMALL: IntPx = int_px(12);

pub const PARAGRAPH_FONT_SIZE_LARGE: IntPx = int_px(16);
pub const PARAGRAPH_FONT_SIZE_MEDIUM: IntPx = int_px(12);
pub const PARAGRAPH_FONT_SIZE_SMALL: IntPx = int_px(8);

pub const DEFAULT_TEXT_STYLE: TextStyle = TextStyle {
    border: None,
    drop_shadow: None,
    color: palette::ON_SURFACE,
    background: None,
    line_height_percent: percent(130.0),
    underline: None,
};

pub enum FontSize {
    Large,
    Medium,
    Small,
}
pub enum TextAlign {
    LeftTop,
    LeftCenter { height: Px },
    Center { wh: Wh<Px> },
    RightTop { width: Px },
}

pub struct Headline {
    pub text: String,
    pub font_size: FontSize,
    pub text_align: TextAlign,
    pub max_width: Option<Px>,
}
impl Component for Headline {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            text,
            font_size,
            text_align,
            max_width,
        } = self;

        let (x, y) = match text_align {
            TextAlign::LeftTop => (0.px(), 0.px()),
            TextAlign::LeftCenter { height } => (0.px(), height * 0.5),
            TextAlign::Center { wh } => (wh.width * 0.5, wh.height * 0.5),
            TextAlign::RightTop { width } => (width, 0.px()),
        };
        let align = match text_align {
            TextAlign::LeftTop => namui::TextAlign::Left,
            TextAlign::LeftCenter { .. } => namui::TextAlign::Left,
            TextAlign::Center { .. } => namui::TextAlign::Center,
            TextAlign::RightTop { .. } => namui::TextAlign::Right,
        };
        let size = match font_size {
            FontSize::Large => HEADLINE_FONT_SIZE_LARGE,
            FontSize::Medium => HEADLINE_FONT_SIZE_MEDIUM,
            FontSize::Small => HEADLINE_FONT_SIZE_SMALL,
        };
        let baseline = match text_align {
            TextAlign::LeftTop => TextBaseline::Top,
            TextAlign::LeftCenter { .. } => TextBaseline::Middle,
            TextAlign::Center { .. } => TextBaseline::Middle,
            TextAlign::RightTop { .. } => TextBaseline::Top,
        };

        ctx.add(namui::text(TextParam {
            text,
            x,
            y,
            align,
            baseline,
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(),
                size,
            },
            style: DEFAULT_TEXT_STYLE,
            max_width,
        }));
    }
}

pub struct Paragraph {
    pub text: String,
    pub font_size: FontSize,
    pub text_align: TextAlign,
    pub max_width: Option<Px>,
}
impl Component for Paragraph {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            text,
            font_size,
            text_align,
            max_width,
        } = self;

        let (x, y) = match text_align {
            TextAlign::LeftTop => (0.px(), 0.px()),
            TextAlign::LeftCenter { height } => (0.px(), height * 0.5),
            TextAlign::Center { wh } => (wh.width * 0.5, wh.height * 0.5),
            TextAlign::RightTop { width } => (width, 0.px()),
        };
        let align = match text_align {
            TextAlign::LeftTop => namui::TextAlign::Left,
            TextAlign::LeftCenter { .. } => namui::TextAlign::Left,
            TextAlign::Center { .. } => namui::TextAlign::Center,
            TextAlign::RightTop { .. } => namui::TextAlign::Right,
        };
        let size = match font_size {
            FontSize::Large => PARAGRAPH_FONT_SIZE_LARGE,
            FontSize::Medium => PARAGRAPH_FONT_SIZE_MEDIUM,
            FontSize::Small => PARAGRAPH_FONT_SIZE_SMALL,
        };
        let baseline = match text_align {
            TextAlign::LeftTop => TextBaseline::Top,
            TextAlign::LeftCenter { .. } => TextBaseline::Middle,
            TextAlign::Center { .. } => TextBaseline::Middle,
            TextAlign::RightTop { .. } => TextBaseline::Top,
        };

        ctx.add(namui::text(TextParam {
            text,
            x,
            y,
            align,
            baseline,
            font: Font {
                name: PARAGRAPH_FONT_NAME.to_string(),
                size,
            },
            style: DEFAULT_TEXT_STYLE,
            max_width,
        }));
    }
}
