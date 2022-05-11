use crate::{namui::skia::StrokeOptions, LtrbRect, Path, Xy};
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize, Clone)]
enum PathCommand {
    AddRect(LtrbRect),
    AddRrect(LtrbRect, Xy<f32>),
    Stroke(StrokeOptions),
    MoveTo(Xy<f32>),
    LineTo(Xy<f32>),
    ArcTo(LtrbRect, f32, f32),
    Scale(Xy<f32>),
    Translate(Xy<f32>),
    Transform([f32; 9]),
    AddOval(LtrbRect),
    AddArc(LtrbRect, f32, f32),
    AddPoly(Vec<Xy<f32>>, bool),
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
    pub fn add_rect(mut self, ltrb_rect: &LtrbRect) -> Self {
        self.commands.push(PathCommand::AddRect(*ltrb_rect));
        self
    }
    pub fn add_rrect(mut self, rect: &LtrbRect, rx: f32, ry: f32) -> Self {
        self.commands
            .push(PathCommand::AddRrect(*rect, Xy { x: rx, y: ry }));
        self
    }
    pub fn stroke(&mut self, options: StrokeOptions) -> Result<(), ()> {
        self.commands.push(PathCommand::Stroke(options));
        Ok(()) // TODO: This is false Ok. Make it sure with stroke execution.
    }
    pub fn move_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::MoveTo(Xy { x, y }));
        self
    }
    pub fn line_to(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::LineTo(Xy { x, y }));
        self
    }
    pub fn arc_to(mut self, oval: &LtrbRect, start_radian: f32, delta_radian: f32) -> Self {
        self.commands
            .push(PathCommand::ArcTo(*oval, start_radian, delta_radian));
        self
    }
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::Scale(Xy { x, y }));
        self
    }
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.commands.push(PathCommand::Translate(Xy { x, y }));
        self
    }
    pub fn transform(mut self, matrix_3x3: &[f32; 9]) -> Self {
        self.commands.push(PathCommand::Transform(*matrix_3x3));
        self
    }
    pub fn add_oval(mut self, ltrb_rect: &LtrbRect) -> Self {
        self.commands.push(PathCommand::AddOval(*ltrb_rect));
        self
    }
    pub fn add_arc(mut self, oval: &LtrbRect, start_radian: f32, delta_radian: f32) -> Self {
        self.commands
            .push(PathCommand::AddArc(*oval, start_radian, delta_radian));
        self
    }
    pub fn add_poly(mut self, xy_array: &[Xy<f32>], close: bool) -> Self {
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
                PathCommand::AddRect(ltrb_rect) => path.add_rect(&ltrb_rect),
                PathCommand::AddRrect(ltrb_rect, rx_ry) => {
                    path.add_rrect(&ltrb_rect, rx_ry.x, rx_ry.y)
                }
                PathCommand::Stroke(options) => {
                    path.stroke(options).unwrap();
                    path
                }
                PathCommand::MoveTo(xy) => path.move_to(xy.x, xy.y),
                PathCommand::LineTo(xy) => path.line_to(xy.x, xy.y),
                PathCommand::ArcTo(oval, start_radian, delta_radian) => {
                    path.arc_to(&oval, *start_radian, *delta_radian)
                }
                PathCommand::Scale(xy) => path.scale(xy.x, xy.y),
                PathCommand::Translate(xy) => path.translate(xy.x, xy.y),
                PathCommand::Transform(matrix_3x3) => path.transform(&matrix_3x3),
                PathCommand::AddOval(ltrb_rect) => path.add_oval(&ltrb_rect),
                PathCommand::AddArc(oval, start_radian, delta_radian) => {
                    path.add_arc(&oval, *start_radian, *delta_radian)
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
