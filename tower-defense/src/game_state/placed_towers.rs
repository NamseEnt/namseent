use super::*;

/// Assume that the tower's size is 2x2.
/// All iteration in this struct will be in the order of left-top, right-top, left-bottom, right-bottom.
#[derive(Default, Clone, PartialEq, State)]
pub struct PlacedTowers {
    /// key is the left-top coord of the tower.
    inner: Vec<Tower>,
}

impl PlacedTowers {
    pub fn iter(&self) -> impl Iterator<Item = &Tower> {
        self.inner.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tower> {
        self.inner.iter_mut()
    }

    pub fn coords(&self) -> Vec<MapCoord> {
        self.iter()
            .flat_map(|tower| {
                let left_top = tower.left_top;
                let right_top = left_top + MapCoord::new(1, 0);
                let left_bottom = left_top + MapCoord::new(0, 1);
                let right_bottom = left_top + MapCoord::new(1, 1);
                [left_top, right_top, left_bottom, right_bottom]
            })
            .collect()
    }

    pub fn place_tower(&mut self, tower: Tower) {
        // let's find the right place of tower and insert it

        let Some(index) = self.inner.iter().position(|placed_tower| {
            tower.left_top.y < placed_tower.left_top.y || tower.left_top.x < placed_tower.left_top.x
        }) else {
            self.inner.push(tower);
            return;
        };

        self.inner.insert(index, tower);
    }

    pub fn remove_tower(&mut self, tower_id: usize) {
        self.inner.retain(|tower| tower.id() != tower_id);
    }

    pub fn find_by_id(&self, tower_id: usize) -> Option<&Tower> {
        self.inner.iter().find(|tower| tower.id() == tower_id)
    }

    pub fn find_by_xy(&self, xy: MapCoord) -> Option<&Tower> {
        self.inner.iter().find(|tower| {
            tower.left_top.x <= xy.x
                && xy.x < tower.left_top.x + 2
                && tower.left_top.y <= xy.y
                && xy.y < tower.left_top.y + 2
        })
    }
}
