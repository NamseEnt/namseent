use crate::app::{
    color::THEME,
    note::{Direction, Note},
};
use namui::prelude::*;
use namui_prebuilt::{simple_rect, typography};

#[namui::component]
pub struct NotePlotter<'a> {
    pub wh: Wh<Px>,
    pub notes: &'a Vec<Note>,
    pub px_per_time: Per<Px, Time>,
    pub timing_zero_x: Px,
    pub played_time: Time,
}

impl Component for NotePlotter<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let NotePlotter {
            wh,
            notes,
            px_per_time,
            timing_zero_x,
            played_time,
        } = self;
        const STROKE_WIDTH: Px = px(2.0);
        const TIMING_ZERO_BAR_WIDTH: Px = px(6.0);

        let (divider_y_array, baseline_y_array) = {
            let height = wh.height / 2;
            let half = height / 2;
            let divider_y_array = [0.px(), height, height * 2];
            let baseline_y_array = [half, divider_y_array[1] + half];
            (divider_y_array, baseline_y_array)
        };

        ctx.compose(|ctx| {
            ctx.translate((timing_zero_x - (TIMING_ZERO_BAR_WIDTH / 2), 0.px()))
                .add(simple_rect(
                    Wh {
                        width: TIMING_ZERO_BAR_WIDTH,
                        height: wh.height,
                    },
                    THEME.surface.contrast_text,
                    STROKE_WIDTH,
                    THEME.primary.main,
                ));
        });

        ctx.compose(|ctx| {
            for note in notes {
                let note_x = timing_zero_x + (px_per_time * (note.time - played_time));
                if note_x < -wh.width {
                    continue;
                }
                if note_x > wh.width * 2 {
                    break;
                }
                let note_y = match note.direction {
                    Direction::Up | Direction::Right => baseline_y_array[0],
                    Direction::Down | Direction::Left => baseline_y_array[1],
                };
                let note_xy = Xy {
                    x: note_x,
                    y: note_y,
                };
                let key = format!("{:?}-{:?}", note.instrument, note.time);
                ctx.add_with_key(
                    key,
                    NoteGraphic {
                        center_xy: note_xy,
                        direction: note.direction,
                    },
                );
            }
        });

        ctx.component(path(
            Path::new()
                .move_to(0.px(), divider_y_array[1])
                .line_to(wh.width, divider_y_array[1]),
            Paint::new()
                .set_style(PaintStyle::Stroke)
                .set_color(THEME.surface.contrast_text)
                .set_stroke_width(STROKE_WIDTH),
        ));

        ctx.component(simple_rect(
            wh,
            THEME.surface.contrast_text,
            STROKE_WIDTH,
            THEME.surface.main,
        ));

        ctx.done()
    }
}

#[component]
struct NoteGraphic {
    center_xy: Xy<Px>,
    direction: Direction,
}
impl Component for NoteGraphic {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            center_xy,
            direction,
        } = self;
        const NOTE_RADIUS: Px = px(64.0);
        const STROKE_WIDTH: Px = px(4.0);

        let note_text = match direction {
            Direction::Up => "↑",
            Direction::Right => "→",
            Direction::Down => "↓",
            Direction::Left => "←",
        }
        .to_string();
        let note_color = match direction {
            Direction::Up => Color::from_u8(0x8b, 0xc3, 0x4a, 255),
            Direction::Right => Color::from_u8(0x67, 0x3a, 0xb7, 255),
            Direction::Down => Color::from_u8(0xf4, 0x43, 0x36, 255),
            Direction::Left => Color::from_u8(0x21, 0x96, 0xf3, 255),
        };
        let note_path = Path::new().add_oval(Rect::Ltrb {
            left: -NOTE_RADIUS,
            top: -NOTE_RADIUS,
            right: NOTE_RADIUS,
            bottom: NOTE_RADIUS,
        });
        ctx.compose(|ctx| {
            ctx.translate(center_xy)
                .add(text(TextParam {
                    text: note_text,
                    x: 0.px(),
                    y: 0.px(),
                    align: TextAlign::Center,
                    baseline: TextBaseline::Middle,
                    font: Font {
                        size: typography::adjust_font_size(NOTE_RADIUS * 2),
                        name: "NotoSansKR-Bold".to_string(),
                    },
                    style: TextStyle {
                        border: None,
                        drop_shadow: None,
                        color: THEME.primary.contrast_text,
                        background: None,
                        line_height_percent: 100.percent(),
                        underline: None,
                    },
                    max_width: None,
                }))
                .add(path(
                    note_path.clone(),
                    Paint::new()
                        .set_style(PaintStyle::Stroke)
                        .set_color(THEME.primary.contrast_text)
                        .set_stroke_width(STROKE_WIDTH),
                ))
                .add(path(
                    note_path,
                    Paint::new()
                        .set_style(PaintStyle::Fill)
                        .set_color(note_color),
                ));
        });

        ctx.done()
    }
}
