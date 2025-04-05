use crate::*;

pub struct Grid {
    wh: Wh<usize>,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(wh: Wh<usize>) -> Self {
        let cells = (0..wh.height)
            .map(|_| (0..wh.width).map(|_| Cell::new_empty()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self { wh, cells }
    }
    pub fn within_bounds(&self, xy: GameXy) -> bool {
        xy.x >= 0 && xy.y >= 0 && xy.x < self.wh.width as isize && xy.y < self.wh.height as isize
    }
    pub fn install_object(&mut self, blueprint: Blueprint) -> Result<(), InstallErr> {
        let collider_xys = rotate_shape(&blueprint.collider_shape, blueprint.rotation)
            .iter()
            .map(|xy| xy + blueprint.xy)
            .collect::<Vec<_>>();

        for xy in collider_xys.iter().copied() {
            if !self.within_bounds(xy) {
                return Err(InstallErr::OutOfBounds);
            }
            if self.cell(xy).object.is_some() {
                return Err(InstallErr::Collision);
            }
        }

        for xy in collider_xys {
            self.cell_mut(xy).install_object();
        }

        Ok(())
    }
    fn cell(&self, xy: GameXy) -> &Cell {
        &self.cells[xy.y as usize][xy.x as usize]
    }
    fn cell_mut(&mut self, xy: GameXy) -> &mut Cell {
        &mut self.cells[xy.y as usize][xy.x as usize]
    }
}
pub struct Blueprint {
    pub xy: GameXy,
    pub collider_shape: Vec<GameXy>,
    pub rotation: ObjectRotation,
}

pub enum ObjectRotation {
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}
fn rotate_shape(shape: &Vec<GameXy>, rotation: ObjectRotation) -> Vec<GameXy> {
    shape
        .iter()
        .map(|xy| match rotation {
            ObjectRotation::None => *xy,
            ObjectRotation::Clockwise90 => GameXy::new(xy.y, -xy.x),
            ObjectRotation::Clockwise180 => GameXy::new(-xy.x, -xy.y),
            ObjectRotation::Clockwise270 => GameXy::new(-xy.y, xy.x),
        })
        .collect()
}

pub enum InstallErr {
    OutOfBounds,
    Collision,
}

#[derive(Debug)]
struct Cell {
    floor_tile: Option<FloorTile>,
    object: Option<Object>,
}

impl Cell {
    pub fn new_empty() -> Self {
        Self {
            floor_tile: None,
            object: None,
        }
    }

    fn can_install_object(&self) -> bool {
        self.object.is_none()
    }

    fn install_object(&self) {
        todo!()
    }
}

#[derive(Debug)]
enum FloorTile {}

#[derive(Debug)]
struct Object {}
