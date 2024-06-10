use std::collections::VecDeque;

#[derive(Debug, Default, PartialEq)]
pub struct History<const N: usize, Item: Clone> {
    undo_queue: VecDeque<Item>,
    item: Item,
    redo_queue: Vec<Item>,
}

impl<const N: usize, Item: Clone> History<N, Item> {
    pub fn new(item: Item) -> Self {
        Self {
            undo_queue: VecDeque::with_capacity(N),
            item,
            redo_queue: Vec::with_capacity(N),
        }
    }

    pub fn get(&self) -> &Item {
        &self.item
    }

    pub fn undo(&mut self) -> bool {
        let Some(item) = self.undo_queue.pop_back() else {
            return false;
        };
        let last_item = std::mem::replace(&mut self.item, item);
        self.redo_queue.push(last_item);

        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(item) = self.redo_queue.pop() else {
            return false;
        };
        let last_item = std::mem::replace(&mut self.item, item);
        self.undo_queue.push_back(last_item);

        true
    }

    pub fn update(&mut self, func: impl FnOnce(&mut Item)) {
        self.redo_queue.clear();

        self.undo_queue.push_back({
            let cloned = self.item.clone();
            std::mem::replace(&mut self.item, cloned)
        });

        if self.undo_queue.len() + 1 > N {
            self.undo_queue.pop_front();
        }

        func(&mut self.item);
    }
}
