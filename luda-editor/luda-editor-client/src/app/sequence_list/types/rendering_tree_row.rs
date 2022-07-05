use namui::prelude::*;

pub struct RenderingTreeRow {
    pub rendering_tree: RenderingTree,
    pub height: Px,
}

impl RenderingTreeRow {
    pub fn new(rendering_tree: RenderingTree, height: Px) -> Self {
        Self {
            rendering_tree,
            height,
        }
    }
}

pub trait RenderingTreeRows {
    fn height(&self, spacing: Px) -> Px;
    fn render(self, spacing: Px) -> RenderingTree;
}

impl RenderingTreeRows for Vec<RenderingTreeRow> {
    fn height(&self, spacing: Px) -> Px {
        self.iter()
            .enumerate()
            .fold(px(0.0), |height, (index, rendering_tree_with_height)| {
                height
                    + rendering_tree_with_height.height
                    + if index != 0 { spacing } else { px(0.0) }
            })
    }
    fn render(self, spacing: Px) -> RenderingTree {
        let mut previous_height = px(0.0);
        let rows = self.into_iter().map(|row| {
            let rendering_tree = namui::translate(px(0.0), previous_height, row.rendering_tree);
            previous_height += row.height + spacing;
            rendering_tree
        });
        render(rows)
    }
}
