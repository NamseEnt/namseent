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
    pub fn in_wh(&self, xy: GameXy) -> bool {
        xy.x < self.wh.width && xy.y < self.wh.height
    }
    pub fn install_object(&mut self, blueprint: Blueprint) -> Result<(), InstallErr> {
        let collider_xys = rotate_shape(&blueprint.collider_shape, blueprint.rotation)
            .iter()
            .map(|xy| {
                let xy = xy + blueprint.xy.map(|v| v as isize);
                if xy.x < 0 || xy.y < 0 {
                    return Err(InstallErr::OutOfBounds);
                }
                let xy = xy.map(|v| v as usize);
                if !self.in_wh(xy) {
                    return Err(InstallErr::OutOfBounds);
                }
                if self.cell(xy).object.is_some() {
                    return Err(InstallErr::Collision);
                }
                Ok(xy)
            })
            .collect::<Result<Vec<_>, _>>()?;

        for xy in collider_xys {
            self.cell_mut(xy).install_object();
        }

        Ok(())
    }
    fn cell(&self, xy: GameXy) -> &Cell {
        &self.cells[xy.y][xy.x]
    }
    fn cell_mut(&mut self, xy: GameXy) -> &mut Cell {
        &mut self.cells[xy.y][xy.x]
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
fn rotate_shape(shape: &Vec<GameXy>, rotation: ObjectRotation) -> Vec<GameXyS> {
    shape
        .iter()
        .map(|xy| {
            let xy = xy.map(|v| v as isize);
            match rotation {
                ObjectRotation::None => xy,
                ObjectRotation::Clockwise90 => GameXyS::new(xy.y, -xy.x),
                ObjectRotation::Clockwise180 => GameXyS::new(-xy.x, -xy.y),
                ObjectRotation::Clockwise270 => GameXyS::new(-xy.y, xy.x),
            }
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
