use namui::Xy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Circumscribed {
    pub center: Xy<f32>,
    pub radius: f32,
}
