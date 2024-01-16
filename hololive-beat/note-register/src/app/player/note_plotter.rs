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
    pub timing_zero_x: Px,
    pub played_time: Duration,
    pub note_width: Px,
}

impl Component for NotePlotter<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let NotePlotter {
            wh,
            notes,
            px_per_time,
            timing_zero_x,
            played_time,
            note_width,
        } = self;
        const STROKE_WIDTH: Px = px(8.0);
        const PAD_WIDTH: Px = px(128.0);

        let note_wh = Wh {
            width: note_width,
            height: (wh.height - (STROKE_WIDTH * 5)) / 4,
        };
        let (divider_y_array, baseline_y_array) = {
            let height = wh.height / 4;
            let divider_y_array = [height, height * 2, height * 3];
            let half_stroke = STROKE_WIDTH / 2;
            let baseline_y_array = [
                STROKE_WIDTH,
                half_stroke + divider_y_array[0],
                half_stroke + divider_y_array[1],
                half_stroke + divider_y_array[2],
            ];
            (divider_y_array, baseline_y_array)
        };

        for direction in [
            Direction::Up,
            Direction::Right,
            Direction::Left,
            Direction::Down,
        ] {
            let rect = Rect::from_xy_wh(
                Xy {
                    x: STROKE_WIDTH,
                    y: baseline_y_array[direction.lane()],
                },
                Wh::new(PAD_WIDTH, note_wh.height),
            );
            ctx.component(Pad { rect, direction });
        }

        ctx.compose(|ctx| {
            ctx.translate((timing_zero_x, 0.px())).add(simple_rect(
                Wh {
                    width: note_width,
                    height: wh.height,
                },
                Color::TRANSPARENT,
                0.px(),
                Color::from_u8(255, 255, 255, 128),
            ));
        });

        ctx.compose(|ctx| {
            for note in notes {
                let note_x = (px_per_time * (note.start_time - played_time)) + timing_zero_x;
                if note_x < -wh.width {
                    continue;
                }
                if note_x > wh.width * 2 {
                    break;
                }
                let note_y = baseline_y_array[note.direction.lane()];
                let note_xy = Xy {
                    x: note_x,
                    y: note_y,
                };
                let note_rect = Rect::from_xy_wh(note_xy, note_wh);
                let key = format!("{:?}-{:?}", note.instrument, note.start_time);
                ctx.add_with_key(
                    key,
                    NoteGraphic {
                        rect: note_rect,
                        direction: note.direction,
                    },
                );
            }
        });

        ctx.component(path(
            Path::new()
                .move_to(0.px(), divider_y_array[0])
                .line_to(wh.width, divider_y_array[0])
                .move_to(0.px(), divider_y_array[1])
                .line_to(wh.width, divider_y_array[1])
                .move_to(0.px(), divider_y_array[2])
                .line_to(wh.width, divider_y_array[2]),
            Paint::new(Color::from_u8(255, 255, 255, 128))
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(STROKE_WIDTH),
        ));

        ctx.component(simple_rect(
            wh,
            Color::from_u8(255, 255, 255, 128),
            STROKE_WIDTH,
            THEME.surface.main,
        ));

        ctx.done()
    }
}

#[component]
struct NoteGraphic {
    rect: Rect<Px>,
    direction: Direction,
}
impl Component for NoteGraphic {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { rect, direction } = self;

        let note_path = Path::new().add_rect(rect);

        ctx.component(path(
            note_path,
            Paint::new(direction.as_color()).set_style(PaintStyle::Fill),
        ));

        ctx.done()
    }
}

#[component]
struct Pad {
    rect: Rect<Px>,
    direction: Direction,
}
impl Component for Pad {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { rect, direction } = self;

        let text = match direction {
            Direction::Left => "←",
            Direction::Up => "↑",
            Direction::Down => "↓",
            Direction::Right => "→",
        }
        .to_string();

        ctx.compose(|ctx| {
            ctx.translate(rect.xy())
                .add(typography::center_text_full_height(
                    rect.wh(),
                    text,
                    Color::WHITE,
                ))
                .add(simple_rect(
                    rect.wh(),
                    Color::TRANSPARENT,
                    0.px(),
                    direction.as_color(),
                ));
        });

        ctx.done()
    }
}
