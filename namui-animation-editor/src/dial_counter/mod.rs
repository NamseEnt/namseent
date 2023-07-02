use namui::prelude::*;
use namui_prebuilt::simple_rect;
use std::{
    fmt::{Debug, Display},
    ops::*,
    sync::Arc,
};

#[allow(dead_code)]
pub struct DialCounter {
    id: namui::Uuid,
    // zoom: f32, // TODO
}

pub struct Props<TValue, TValueChanged> {
    pub rect: Rect<Px>,
    pub value: TValue,
    pub value_per_px: Per<TValue, Px>,
    pub small_gradation_value_interval: TValue,
    pub big_gradation_per_small_gradation: usize,
    pub on_value_changed: TValueChanged,
}

pub trait Abs {
    fn abs(&self) -> Self;
}

impl DialCounter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { id: uuid() }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, _event: &namui::Event) {}

    #[allow(dead_code)]
    pub fn render<TValue, TValueChanged>(
        &self,
        props: Props<TValue, TValueChanged>,
    ) -> RenderingTree
    where
        TValue: Rem<Output = TValue>
            + Copy
            + Add<Output = TValue>
            + Sub<Output = TValue>
            + Div<Output = f32>
            + Mul<f32, Output = TValue>
            + Display
            + Debug
            + Abs
            + 'static,
        TValueChanged: Fn(TValue) + 'static,
    {
        let value_line = translate(
            props.rect.width() / 2,
            0.px(),
            path(
                PathBuilder::new().line_to(0.px(), props.rect.height()),
                PaintBuilder::new()
                    .set_anti_alias(true)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(1.px())
                    .set_color(Color::from_u8(255, 0, 255, 255)),
            ),
        );

        let most_left_position_value =
            props.value - (props.value_per_px * (props.rect.width() / 2));

        let first_gradation_value = {
            let sign = most_left_position_value / most_left_position_value.abs();
            if sign >= 0.0 {
                most_left_position_value
                    + (props.small_gradation_value_interval
                        - (most_left_position_value % props.small_gradation_value_interval))
            } else {
                most_left_position_value
                    - (props.small_gradation_value_interval
                        + (most_left_position_value % props.small_gradation_value_interval))
            }
        };

        let px_of_value = |value: TValue| -> Px {
            props.value_per_px.invert() * (value - most_left_position_value)
        };

        let mut gradations = vec![];

        let small_gradation = path(
            PathBuilder::new().line_to(0.px(), props.rect.height() / 5),
            PaintBuilder::new()
                .set_anti_alias(true)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(1.px())
                .set_color(Color::grayscale_f01(0.5)),
        );

        let big_gradation = |x: Px, label: String| {
            const LABEL_FONT_SIZE: IntPx = int_px(10);
            translate(
                x,
                0.px(),
                render([
                    path(
                        PathBuilder::new().line_to(0.px(), props.rect.height() / 4),
                        PaintBuilder::new()
                            .set_anti_alias(true)
                            .set_style(PaintStyle::Stroke)
                            .set_stroke_width(2.px())
                            .set_color(Color::grayscale_f01(0.0)),
                    ),
                    text(TextParam {
                        text: label,
                        x: 0.px(),
                        y: props.rect.height() / 4,
                        align: TextAlign::Center,
                        baseline: TextBaseline::Top,
                        font_type: FontType {
                            serif: false,
                            size: LABEL_FONT_SIZE,
                            language: Language::Ko,
                            font_weight: FontWeight::BOLD,
                        },
                        style: TextStyle {
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        max_width: None,
                    }),
                ]),
            )
        };

        let value_at_px = |px: Px| -> TValue { props.value_per_px * px + most_left_position_value };

        let is_px_on_big_gradation = |px: Px| {
            if props.big_gradation_per_small_gradation == 0 {
                false
            } else {
                (value_at_px(px) / props.small_gradation_value_interval).round() as i32
                    % props.big_gradation_per_small_gradation as i32
                    == 0
            }
        };

        let mut gradation_value = first_gradation_value;

        loop {
            let gradation_px = px_of_value(gradation_value);
            if gradation_px > props.rect.width() {
                break;
            }

            if (is_px_on_big_gradation)(gradation_px) {
                let mut label_text = format!("{:.1}", gradation_value);

                if label_text.eq("-0.0") {
                    label_text = "0.0".to_string();
                }

                gradations.push((big_gradation)(gradation_px, label_text));
            } else {
                gradations.push(translate(gradation_px, 0.px(), small_gradation.clone()));
            }
            gradation_value = gradation_value + props.small_gradation_value_interval;
        }

        let on_value_changed = Arc::new(props.on_value_changed);
        let background_with_event_handler =
            simple_rect(props.rect.wh(), Color::BLACK, 1.px(), Color::WHITE).attach_event(
                move |builder| {
                    let on_value_changed = on_value_changed.clone();
                    builder.on_wheel({
                        move |event: WheelEvent| {
                            let next_value =
                                props.value + (props.value_per_px * event.delta_xy.y.px());

                            (on_value_changed)(next_value);
                        }
                    });
                },
            );

        translate(
            props.rect.x(),
            props.rect.y(),
            render([
                background_with_event_handler,
                render(gradations),
                value_line,
            ]),
        )
    }
}
