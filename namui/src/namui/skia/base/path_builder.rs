use crate::{
    namui::{render::Matrix3x3, skia::StrokeOptions},
    *,
};
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize, Clone)]
enum PathCommand {
    AddRect(Rect<Px>),
    AddRrect(Rect<Px>, Xy<Px>),
    Stroke(StrokeOptions),
    MoveTo(Xy<Px>),
    LineTo(Xy<Px>),
    ArcTo(Rect<Px>, Angle, Angle),
    Scale(Xy<f32>),
    Translate(Xy<Px>),
    Transform(Matrix3x3),
    AddOval(Rect<Px>),
    AddArc(Rect<Px>, Angle, Angle),
    AddPoly(Vec<Xy<Px>>, bool),
    Close,
}

#[derive(Debug, Serialize, Clone)]
pub struct PathBuilder {
    commands: Vec<PathCommand>,
}

static PATH_CACHE: OnceCell<Mutex<lru::LruCache<PathBuilder, Arc<Path>>>> = OnceCell::new();

impl PathBuilder {
    pub(crate) fn build(&self) -> Arc<Path> {
        self.get_or_create_path()
    }
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    pub fn add_rect(mut self, rect: Rect<Px>) -> Self {
        self.commands.push(PathCommand::AddRect(rect));
        self
    }
    pub fn add_rrect(mut self, rect: Rect<Px>, rx: Px, ry: Px) -> Self {
        self.commands
            .push(PathCommand::AddRrect(rect, Xy { x: rx, y: ry }));
        self
    }
    pub fn stroke(&mut self, options: StrokeOptions) -> Result<(), ()> {
        self.commands.push(PathCommand::Stroke(options));
        Ok(()) // TODO: This is false Ok. Make it sure with stroke execution.
    }
    pub fn move_to(mut self, x: Px, y: Px) -> Self {
        self.commands.push(PathCommand::MoveTo(Xy { x, y }));
        self
    }
    pub fn line_to(mut self, x: Px, y: Px) -> Self {
        self.commands.push(PathCommand::LineTo(Xy { x, y }));
        self
    }
    pub fn arc_to(mut self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        self.commands
            .push(PathCommand::ArcTo(oval, start_angle, delta_angle));
        self
    }
    pub fn scale(mut self, sx: f32, sy: f32) -> Self {
        self.commands.push(PathCommand::Scale(Xy { x: sx, y: sy }));
        self
    }
    pub fn translate(mut self, x: Px, y: Px) -> Self {
        self.commands.push(PathCommand::Translate(Xy { x, y }));
        self
    }
    pub fn transform(mut self, matrix: Matrix3x3) -> Self {
        self.commands.push(PathCommand::Transform(matrix));
        self
    }
    pub fn add_oval(mut self, rect: Rect<Px>) -> Self {
        self.commands.push(PathCommand::AddOval(rect));
        self
    }
    pub fn add_arc(mut self, oval: Rect<Px>, start_angle: Angle, delta_angle: Angle) -> Self {
        self.commands
            .push(PathCommand::AddArc(oval, start_angle, delta_angle));
        self
    }
    pub fn add_poly(mut self, xy_array: &[Xy<Px>], close: bool) -> Self {
        self.commands
            .push(PathCommand::AddPoly(xy_array.to_vec(), close));
        self
    }
    pub fn close(mut self) -> Self {
        self.commands.push(PathCommand::Close);
        self
    }

    fn get_or_create_path(&self) -> Arc<Path> {
        let mut cache = PATH_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap();
        match cache.get(self) {
            Some(path) => path.clone(),
            None => {
                let path = self.create_path();
                cache.put(self.clone(), path.clone());
                path
            }
        }
    }

    fn create_path(&self) -> Arc<Path> {
        let mut path = Path::new();
        for command in &self.commands {
            path = match command {
                &PathCommand::AddRect(rect) => path.add_rect(rect),
                &PathCommand::AddRrect(rect, rx_ry) => path.add_rrect(rect, rx_ry.x, rx_ry.y),
                &PathCommand::Stroke(options) => {
                    path.stroke(options).unwrap();
                    path
                }
                &PathCommand::MoveTo(xy) => path.move_to(xy.x, xy.y),
                &PathCommand::LineTo(xy) => path.line_to(xy.x, xy.y),
                &PathCommand::ArcTo(oval, start_angle, delta_angle) => {
                    path.arc_to(oval, start_angle, delta_angle)
                }
                &PathCommand::Scale(xy) => path.scale(xy.x, xy.y),
                &PathCommand::Translate(xy) => path.translate(xy.x, xy.y),
                &PathCommand::Transform(matrix) => path.transform(matrix),
                &PathCommand::AddOval(rect) => path.add_oval(rect),
                &PathCommand::AddArc(oval, start_angle, delta_angle) => {
                    path.add_arc(oval, start_angle, delta_angle)
                }
                PathCommand::AddPoly(xy_array, close) => path.add_poly(&xy_array, *close),
                PathCommand::Close => path.close(),
            }
        }
        Arc::new(path)
    }
}

impl Hash for PathBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        bincode::serialize(self).unwrap().hash(state);
    }
}

impl PartialEq for PathBuilder {
    fn eq(&self, other: &Self) -> bool {
        bincode::serialize(self).unwrap() == bincode::serialize(other).unwrap()
    }
}
impl std::cmp::Eq for PathBuilder {}
