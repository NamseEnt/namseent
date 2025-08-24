use super::palette;
use crate::icon;
use namui::*;
use namui_prebuilt::rich_text::*;
use std::{cell::RefCell, collections::HashMap, sync::OnceLock};

pub const HEADLINE_FONT_NAME: &str = "NotoSansKR-Bold";
pub const PARAGRAPH_FONT_NAME: &str = "NotoSansKR-Regular";

pub const HEADLINE_FONT_SIZE_LARGE: IntPx = int_px(24);
pub const HEADLINE_FONT_SIZE_MEDIUM: IntPx = int_px(20);
pub const HEADLINE_FONT_SIZE_SMALL: IntPx = int_px(16);

pub const PARAGRAPH_FONT_SIZE_LARGE: IntPx = int_px(16);
pub const PARAGRAPH_FONT_SIZE_MEDIUM: IntPx = int_px(14);
pub const PARAGRAPH_FONT_SIZE_SMALL: IntPx = int_px(12);

pub const DEFAULT_TEXT_STYLE: TextStyle = TextStyle {
    border: None,
    drop_shadow: None,
    color: palette::ON_SURFACE,
    background: None,
    line_height_percent: percent(130.0),
    underline: None,
};

pub static TAG_MAP: OnceLock<HashMap<String, Tag>> = OnceLock::new();
// thread_local을 사용하여 각 스레드마다 독립적인 RegexHandler 인스턴스 생성
thread_local! {
    static REGEX_HANDLERS: RefCell<Option<Vec<RegexHandler>>> = const { RefCell::new(None) };
}

fn with_regex_handlers<F, R>(f: F) -> R
where
    F: FnOnce(&Vec<RegexHandler>) -> R,
{
    REGEX_HANDLERS.with(|handlers| {
        let mut handlers = handlers.borrow_mut();
        if handlers.is_none() {
            *handlers = Some(init_regex_handlers());
        }
        f(handlers.as_ref().unwrap())
    })
}
pub fn init_tag_map() -> HashMap<String, Tag> {
    let mut map = HashMap::new();

    // 색상 태그 추가
    map.insert(
        "red".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::RED,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "blue".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::BLUE,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "purple".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(128, 0, 128, 255), // 보라색
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "green".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(0, 255, 0, 255), // 초록색
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "yellow".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(255, 255, 0, 255), // 노란색
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "epic".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::EPIC,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    map.insert(
        "rare".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::RARE,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 공격력 컬러 태그
    map.insert(
        "attack_damage_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(255, 100, 100, 255), // 빨간색 계열
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 공격속도 컬러 태그
    map.insert(
        "attack_speed_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(100, 100, 255, 255), // 파란색 계열
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 사거리 컬러 태그
    map.insert(
        "attack_range_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(100, 255, 100, 255), // 초록색 계열
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 골드 컬러 태그
    map.insert(
        "gold_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::from_u8(255, 215, 0, 255), // 골드 색상
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 스페이드/클럽 색상 태그 (검은색)
    map.insert(
        "black_suit_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: Color::BLACK,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 하트/다이아몬드 색상 태그 (빨간색)
    map.insert(
        "red_suit_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::RED,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 흰색 suit 색상 태그 (타워 미리보기용)
    map.insert(
        "white_suit_color".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 적용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: TextStyle {
                color: palette::WHITE,
                ..DEFAULT_TEXT_STYLE
            },
        },
    );

    // 볼드체 태그
    map.insert(
        "B".to_string(),
        Tag::StyledText {
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(), // 볼드 폰트 사용
                size: PARAGRAPH_FONT_SIZE_MEDIUM,
            },
            style: DEFAULT_TEXT_STYLE,
        },
    );

    map
}
pub fn init_regex_handlers() -> Vec<RegexHandler> {
    icon::Icon::create_icon_regex_handlers()
}

pub enum FontSize {
    Large,
    Medium,
    Small,
    Custom { size: Px },
}

pub enum TextAlign {
    LeftTop,
    LeftCenter { height: Px },
    Center { wh: Wh<Px> },
    RightTop { width: Px },
}

pub struct HeadlineBuilder {
    text_content: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
    text_color: Option<Color>,
    text_border: Option<TextStyleBorder>,
    vertical_align: namui_prebuilt::rich_text::VerticalAlign,
}

impl HeadlineBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text_content: text.into(),
            font_size: FontSize::Medium,
            text_align: TextAlign::LeftTop,
            max_width: None,
            text_color: None,
            text_border: None,
            vertical_align: namui_prebuilt::rich_text::VerticalAlign::Center,
        }
    }

    pub fn size(mut self, size: FontSize) -> Self {
        self.font_size = size;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    pub fn max_width(mut self, width: Px) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn stroke(mut self, width: Px, color: Color) -> Self {
        self.text_border = Some(TextStyleBorder { width, color });
        self
    }

    #[allow(dead_code)]
    pub fn vertical_align(mut self, align: namui_prebuilt::rich_text::VerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }

    pub fn build(self) -> HeadlineComponent {
        HeadlineComponent {
            text: self.text_content,
            font_size: self.font_size,
            text_align: self.text_align,
            max_width: self.max_width,
            text_color: self.text_color,
            text_border: self.text_border,
        }
    }

    pub fn build_rich(self) -> RichHeadlineComponent {
        RichHeadlineComponent {
            text: self.text_content,
            font_size: self.font_size,
            text_align: self.text_align,
            max_width: self.max_width,
            text_color: self.text_color,
            vertical_align: self.vertical_align,
        }
    }
}

pub struct ParagraphBuilder {
    text_content: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
    text_color: Option<Color>,
    vertical_align: namui_prebuilt::rich_text::VerticalAlign,
}

impl ParagraphBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text_content: text.into(),
            font_size: FontSize::Medium,
            text_align: TextAlign::LeftTop,
            max_width: None,
            text_color: None,
            vertical_align: namui_prebuilt::rich_text::VerticalAlign::Center,
        }
    }

    pub fn size(mut self, size: FontSize) -> Self {
        self.font_size = size;
        self
    }

    pub fn align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    pub fn max_width(mut self, width: Px) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    #[allow(dead_code)]
    pub fn vertical_align(mut self, align: namui_prebuilt::rich_text::VerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }

    pub fn build(self) -> ParagraphComponent {
        ParagraphComponent {
            text: self.text_content,
            font_size: self.font_size,
            text_align: self.text_align,
            max_width: self.max_width,
        }
    }

    pub fn build_rich(self) -> RichParagraphComponent {
        RichParagraphComponent {
            text: self.text_content,
            font_size: self.font_size,
            text_align: self.text_align,
            max_width: self.max_width,
            vertical_align: self.vertical_align,
        }
    }
}

pub struct RichHeadlineComponent {
    text: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
    text_color: Option<Color>,
    vertical_align: namui_prebuilt::rich_text::VerticalAlign,
}

impl Component for RichHeadlineComponent {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            text,
            font_size,
            text_align,
            max_width,
            text_color,
            vertical_align,
        } = self;

        let (x, y) = match text_align {
            TextAlign::LeftTop => (0.px(), 0.px()),
            TextAlign::LeftCenter { height } => (0.px(), height * 0.5),
            TextAlign::Center { wh } => (wh.width * 0.5, wh.height * 0.5),
            TextAlign::RightTop { width } => (width, 0.px()),
        };

        let size = match font_size {
            FontSize::Large => HEADLINE_FONT_SIZE_LARGE,
            FontSize::Medium => HEADLINE_FONT_SIZE_MEDIUM,
            FontSize::Small => HEADLINE_FONT_SIZE_SMALL,
            FontSize::Custom { size } => size.into(),
        };

        ctx.translate(Xy { x, y });

        let text_style = if let Some(custom_color) = text_color {
            TextStyle {
                color: custom_color,
                ..DEFAULT_TEXT_STYLE
            }
        } else {
            DEFAULT_TEXT_STYLE
        };

        let rich_text_align = match text_align {
            TextAlign::LeftTop => namui::TextAlign::Left,
            TextAlign::LeftCenter { .. } => namui::TextAlign::Left,
            TextAlign::Center { .. } => namui::TextAlign::Center,
            TextAlign::RightTop { .. } => namui::TextAlign::Right,
        };

        let effective_max_width = match (&text_align, max_width) {
            (TextAlign::Center { wh }, None) => Some(wh.width),
            (TextAlign::RightTop { width }, None) => Some(*width),
            _ => max_width,
        };

        with_regex_handlers(|regex_handlers| {
            ctx.add(namui_prebuilt::rich_text::RichText {
                text,
                max_width: effective_max_width,
                default_font: Font {
                    name: HEADLINE_FONT_NAME.to_string(),
                    size,
                },
                default_text_style: text_style,
                default_text_align: rich_text_align,
                default_vertical_align: vertical_align,
                tag_map: TAG_MAP.get_or_init(init_tag_map),
                regex_handlers,
                on_parse_error: None,
            });
        });
    }
}

pub struct RichParagraphComponent {
    text: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
    vertical_align: namui_prebuilt::rich_text::VerticalAlign,
}

impl Component for RichParagraphComponent {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            text,
            font_size,
            text_align,
            max_width,
            vertical_align,
        } = self;

        let (x, y) = match text_align {
            TextAlign::LeftTop => (0.px(), 0.px()),
            TextAlign::LeftCenter { height } => (0.px(), height * 0.5),
            TextAlign::Center { wh } => (wh.width * 0.5, wh.height * 0.5),
            TextAlign::RightTop { width } => (width, 0.px()),
        };

        let size = match font_size {
            FontSize::Large => PARAGRAPH_FONT_SIZE_LARGE,
            FontSize::Medium => PARAGRAPH_FONT_SIZE_MEDIUM,
            FontSize::Small => PARAGRAPH_FONT_SIZE_SMALL,
            FontSize::Custom { size } => size.into(),
        };

        let rich_text_align = match text_align {
            TextAlign::LeftTop => namui::TextAlign::Left,
            TextAlign::LeftCenter { .. } => namui::TextAlign::Left,
            TextAlign::Center { .. } => namui::TextAlign::Center,
            TextAlign::RightTop { .. } => namui::TextAlign::Right,
        };

        let effective_max_width = match (&text_align, max_width) {
            (TextAlign::Center { wh }, None) => Some(wh.width),
            (TextAlign::RightTop { width }, None) => Some(*width),
            _ => max_width,
        };

        ctx.translate(Xy { x, y });
        with_regex_handlers(|regex_handlers| {
            ctx.add(namui_prebuilt::rich_text::RichText {
                text,
                max_width: effective_max_width,
                default_font: Font {
                    name: PARAGRAPH_FONT_NAME.to_string(),
                    size,
                },
                default_text_style: DEFAULT_TEXT_STYLE,
                default_text_align: rich_text_align,
                default_vertical_align: vertical_align,
                tag_map: TAG_MAP.get_or_init(init_tag_map),
                regex_handlers,
                on_parse_error: None,
            });
        });
    }
}

pub struct HeadlineComponent {
    text: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
    text_color: Option<Color>,
    text_border: Option<TextStyleBorder>,
}
impl HeadlineComponent {
    pub fn into_rendering_tree(self) -> RenderingTree {
        let Self {
            text,
            font_size,
            text_align,
            max_width,
            text_color,
            text_border,
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
            FontSize::Custom { size } => size.into(),
        };
        let baseline = match text_align {
            TextAlign::LeftTop => TextBaseline::Top,
            TextAlign::LeftCenter { .. } => TextBaseline::Middle,
            TextAlign::Center { .. } => TextBaseline::Middle,
            TextAlign::RightTop { .. } => TextBaseline::Top,
        };

        let text_style = TextStyle {
            color: text_color.unwrap_or(DEFAULT_TEXT_STYLE.color),
            border: text_border,
            ..DEFAULT_TEXT_STYLE
        };

        let effective_max_width = match (&text_align, max_width) {
            (TextAlign::Center { wh }, None) => Some(wh.width),
            (TextAlign::RightTop { width }, None) => Some(*width),
            _ => max_width,
        };

        namui::text(TextParam {
            text,
            x,
            y,
            align,
            baseline,
            font: Font {
                name: HEADLINE_FONT_NAME.to_string(),
                size,
            },
            style: text_style,
            max_width: effective_max_width,
        })
    }
}
impl Component for HeadlineComponent {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(self.into_rendering_tree());
    }
}

pub struct ParagraphComponent {
    text: String,
    font_size: FontSize,
    text_align: TextAlign,
    max_width: Option<Px>,
}
impl ParagraphComponent {
    pub fn into_rendering_tree(self) -> RenderingTree {
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
            FontSize::Custom { size } => size.into(),
        };
        let baseline = match text_align {
            TextAlign::LeftTop => TextBaseline::Top,
            TextAlign::LeftCenter { .. } => TextBaseline::Middle,
            TextAlign::Center { .. } => TextBaseline::Middle,
            TextAlign::RightTop { .. } => TextBaseline::Top,
        };

        let effective_max_width = match (&text_align, max_width) {
            (TextAlign::Center { wh }, None) => Some(wh.width),
            (TextAlign::RightTop { width }, None) => Some(*width),
            _ => max_width,
        };
        namui::text(TextParam {
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
            max_width: effective_max_width,
        })
    }
}
impl Component for ParagraphComponent {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(self.into_rendering_tree());
    }
}

pub fn headline(text: impl Into<String>) -> HeadlineBuilder {
    HeadlineBuilder::new(text)
}

pub fn paragraph(text: impl Into<String>) -> ParagraphBuilder {
    ParagraphBuilder::new(text)
}
