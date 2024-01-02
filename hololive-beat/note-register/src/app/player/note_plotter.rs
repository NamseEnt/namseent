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
    pub px_per_time: Per<Px, Duration>,
    pub timing_zero_y_from_bottom: Px,
    pub played_time: Duration,
}

impl Component for NotePlotter<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let NotePlotter {
            wh,
            notes,
            px_per_time,
            timing_zero_y_from_bottom,
            played_time,
        } = self;
        const STROKE_WIDTH: Px = px(2.0);
        const TIMING_ZERO_BAR_HEIGHT: Px = px(6.0);

        let (divider_x_array, baseline_x_array) = {
            let width = wh.width / 4;
            let half = width / 2;
            let divider_x_array = [width, width * 2, width * 3];
            let baseline_x_array = [
                half,
                divider_x_array[0] + half,
                divider_x_array[1] + half,
                divider_x_array[2] + half,
            ];
            (divider_x_array, baseline_x_array)
        };

        ctx.compose(|ctx| {
            ctx.translate((
                0.px(),
                wh.height - timing_zero_y_from_bottom - (TIMING_ZERO_BAR_HEIGHT / 2),
            ))
            .add(simple_rect(
                Wh {
                    width: wh.width,
                    height: TIMING_ZERO_BAR_HEIGHT,
                },
                THEME.surface.contrast_text,
                STROKE_WIDTH,
                THEME.primary.main,
            ));
        });

        ctx.compose(|ctx| {
            for note in notes {
                let note_y = (px_per_time * (Instant::new(played_time) - note.start_time))
                    - timing_zero_y_from_bottom;
                if note_y > wh.height * 2 {
                    continue;
                }
                if note_y < -wh.height {
                    break;
                }
                let note_x = baseline_x_array[note.direction.lane()];
                let note_xy = Xy {
                    x: note_x,
                    y: note_y,
                };
                let key = format!("{:?}-{:?}", note.instrument, note.start_time);
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
                .move_to(divider_x_array[0], 0.px())
                .line_to(divider_x_array[0], wh.height)
                .move_to(divider_x_array[1], 0.px())
                .line_to(divider_x_array[1], wh.height)
                .move_to(divider_x_array[2], 0.px())
                .line_to(divider_x_array[2], wh.height),
            Paint::new(THEME.surface.contrast_text)
                .set_style(PaintStyle::Stroke)
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
                    Paint::new(THEME.primary.contrast_text)
                        .set_style(PaintStyle::Stroke)
                        .set_stroke_width(STROKE_WIDTH),
                ))
                .add(path(
                    note_path,
                    Paint::new(note_color).set_style(PaintStyle::Fill),
                ));
        });

        ctx.done()
    }
}
