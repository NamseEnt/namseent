use super::*;

pub fn vertical(items: impl IntoIterator<Item = GroupItem>) -> GroupItem {
    GroupItem::Group(Group {
        direction: Direction::Vertical,
        items: items.into_iter().collect(),
    })
}

pub fn horizontal(items: impl IntoIterator<Item = GroupItem>) -> GroupItem {
    GroupItem::Group(Group {
        direction: Direction::Horizontal,
        items: items.into_iter().collect(),
    })
}

pub struct Group {
    direction: Direction,
    items: Vec<GroupItem>,
}

impl GroupItem {
    pub fn render(&self, wh: Wh<Px>) -> RenderingTree {
        let GroupItem::Group(group) = &self else {
            unreachable!()
        };

        let mut trees = vec![];
        match group.direction {
            Direction::Vertical => {
                let mut y = 0.px();
                for item in &group.items {
                    match item {
                        GroupItem::Block(block) => {
                            let row = RowBlock::new(block);
                            trees.push(translate(0.px(), y, row.render(wh.width)));
                            y += row.height();
                        }
                        GroupItem::Group(_) => {
                            let wh_for_item = Wh::new(wh.width, wh.height - y);
                            trees.push(translate(0.px(), y, item.render(wh_for_item)));
                            y += item.height();
                        }
                    }
                }
            }
            Direction::Horizontal => {
                let item_width = wh.width / group.items.len() as f32;

                for (index, item) in group.items.iter().enumerate() {
                    let x = item_width * index as f32;
                    match item {
                        GroupItem::Block(block) => {
                            let row = RowBlock::new(block);
                            trees.push(translate(x, 0.px(), row.render(item_width)));
                        }
                        GroupItem::Group(_) => {
                            trees.push(translate(
                                x,
                                0.px(),
                                item.render(Wh::new(item_width, wh.height)),
                            ));
                        }
                    }
                }
            }
        }
        render(trees)
    }

    fn height(&self) -> Px {
        match self {
            GroupItem::Block(block) => {
                let row = RowBlock::new(block);
                row.height()
            }
            GroupItem::Group(group) => match group.direction {
                Direction::Vertical => group.items.iter().map(|item| item.height()).sum(),
                Direction::Horizontal => group
                    .items
                    .iter()
                    .map(|item| item.height())
                    .reduce(|a, b| a.max(b))
                    .unwrap_or(0.px()),
            },
        }
    }
}

pub enum GroupItem {
    Block(Block),
    Group(Group),
}

impl Into<GroupItem> for Block {
    fn into(self) -> GroupItem {
        GroupItem::Block(self)
    }
}

impl Into<GroupItem> for Group {
    fn into(self) -> GroupItem {
        GroupItem::Group(self)
    }
}

enum Direction {
    Vertical,
    Horizontal,
}
