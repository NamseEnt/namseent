use ::namui::*;

pub struct TrackHeader {}
pub struct TrackHeaderProps {
    pub width: f32,
    pub height: f32,
}

impl namui::Entity for TrackHeader {
    type Props = TrackHeaderProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        render![namui::rect(namui::RectParam {
            x: 0.0,
            y: 0.0,
            width: props.width,
            height: props.height,
            style: namui::RectStyle {
                fill: Some(namui::RectFill {
                    color: namui::Color::from_f01(0.4, 0.4, 0.4, 1.0),
                }),
                stroke: Some(namui::RectStroke {
                    color: namui::Color::BLACK,
                    width: 1.0,
                    border_position: namui::BorderPosition::Inside,
                }),
                ..Default::default()
            },
            ..Default::default()
        })]
    }
}
