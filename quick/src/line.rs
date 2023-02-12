use super::*;

pub struct Line {
    items: Vec<Link>,
}
impl Line {
    pub(crate) fn height(&self) -> Px {
        LINE_HEIGHT
    }

    pub(crate) fn render(&self) -> RenderingTree {
        let mut trees = vec![];
        let mut x = 0.px();
        for item in &self.items {
            let item_rendering_tree = translate(x, 0.px(), item.render());

            x += item_rendering_tree
                .get_bounding_box()
                .map_or(0.px(), |bounding_box| bounding_box.width())
                + LINE_ITEM_GAP;

            trees.push(item_rendering_tree);
        }
        render(trees)
    }
}

pub fn line(content: impl IntoIterator<Item = Link>) -> Line {
    Line {
        items: content.into_iter().collect(),
    }
}
