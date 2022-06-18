use namui::Xy;

pub enum CanvasEvent {
    Scrolled { offset: Xy<f32> },
    Zoomed { offset: Xy<f32>, scale: f32 },
}
