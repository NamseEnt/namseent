use super::*;

impl TimelineWindow {
    pub fn render(&self, props: Props) -> RenderingTree {
        vertical([
            ratio(1.0, |wh| {
                crate::time_ruler::render(&crate::time_ruler::Props {
                    xywh: XywhRect {
                        x: 0.0,
                        y: 0.0,
                        width: wh.width.into(),
                        height: wh.height.into(),
                    },
                    start_at: self.start_at,
                    time_per_pixel: self.time_per_pixel,
                })
            }),
            ratio(2.0, |wh| {
                // TODO: Timeline for other layers
                simple_rect(wh, Color::BLACK, 1.0, Color::grayscale_f01(0.5))
            }),
            ratio(7.0, |wh| {
                // TODO: Timeline for selected layer
                simple_rect(wh, Color::BLACK, 1.0, Color::grayscale_f01(0.5))
            }),
        ])(props.wh)
    }
}
