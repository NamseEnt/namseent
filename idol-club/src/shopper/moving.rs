use super::*;

#[derive(Debug)]
pub struct Moving {
    path: Vec<GameXy>,
    /// 0.0 ~ path.len()
    path_progress: f32,
}
impl Moving {
    pub fn new(path: Vec<GameXy>) -> Self {
        Self {
            path,
            path_progress: 0.0,
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.path_progress += dt.as_secs_f32();
        if self.path_progress >= self.path.len() as f32 {
            self.path_progress = self.path.len() as f32;
        }
    }

    pub fn done(&self) -> bool {
        self.path_progress >= self.path.len() as f32
    }

    pub fn now(&self) -> GameXy {
        self.path[self.path_progress as usize]
    }

    pub fn nowf(&self) -> GameXyF {
        self.path[self.path_progress as usize].map(|v| v as f32)
            + (self.path[self.path_progress as usize + 1] - self.path[self.path_progress as usize])
                .map(|v| v as f32)
                * (self.path_progress - self.path_progress.floor())
    }

    pub fn heading_unit_vector(&self) -> Xy<f32> {
        let next_index = (self.path_progress as usize + 1).min(self.path.len() - 1);
        let next_xy = self.path[next_index];
        let now_xy = self.path[self.path_progress as usize];
        let heading_vector = (next_xy - now_xy).map(|v| v as f32);

        heading_vector / heading_vector.length()
    }
}
