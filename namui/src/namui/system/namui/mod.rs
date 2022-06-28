pub trait NamuiSystem {
    fn init(self) -> NamuiContext;
    fn request_animation_frame(&self, callback: impl FnOnce() + 'static);
    fn log(&self, format: String);
    fn now(&self) -> Duration;
}
