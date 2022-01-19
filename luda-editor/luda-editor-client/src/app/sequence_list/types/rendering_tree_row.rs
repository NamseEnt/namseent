use namui::{render, RenderingTree};

pub struct RenderingTreeRow {
    pub rendering_tree: RenderingTree,
    pub height: f32,
}

impl RenderingTreeRow {
    pub fn new(rendering_tree: RenderingTree, height: f32) -> Self {
        Self {
            rendering_tree,
            height,
        }
    }
}

pub trait RenderingTreeRows {
    fn height(&self, spacing: f32) -> f32;
    fn render(self, spacing: f32) -> RenderingTree;
}

impl RenderingTreeRows for Vec<RenderingTreeRow> {
    fn height(&self, spacing: f32) -> f32 {
        self.iter()
            .enumerate()
            .fold(0.0, |height, (index, rendering_tree_with_height)| {
                height + rendering_tree_with_height.height + if index != 0 { spacing } else { 0.0 }
            })
    }
    fn render(self, spacing: f32) -> RenderingTree {
        let mut previous_height = 0.0;
        let rows: Vec<RenderingTree> = self
            .into_iter()
            .map(|row| {
                let rendering_tree = namui::translate(0.0, previous_height, row.rendering_tree);
                previous_height += row.height + spacing;
                rendering_tree
            })
            .collect();
        render!(rows)
    }
}
