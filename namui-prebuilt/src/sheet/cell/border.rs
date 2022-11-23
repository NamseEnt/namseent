pub trait Border {
    fn border(self, side: Side, line: Line) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Top,
    Right,
    Bottom,
    LeftRight,
    TopBottom,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    None,
    Single,
    Double,
    BoldSingle,
}

///
/// border
/// border starts center pixel between cell and cell.
/// So, there iis 1px extra pixel for border.
///
/// ```text
/// ┌──────┐│┌──────┐
/// │ cell │││ cell │
/// └──────┘│└──────┘
///         ^This is the 1px for start of border.
///         Every cell's border starts from here and goes to inside of cell
/// ```
///
pub struct Borders {
    pub(crate) left: Line,
    pub(crate) top: Line,
    pub(crate) right: Line,
    pub(crate) bottom: Line,
}

impl Borders {
    pub fn new() -> Self {
        Self {
            left: Line::None,
            top: Line::None,
            right: Line::None,
            bottom: Line::None,
        }
    }

    pub(crate) fn add(&mut self, side: Side, line: Line) {
        match side {
            Side::Left => self.left = line,
            Side::Top => self.top = line,
            Side::Right => self.right = line,
            Side::Bottom => self.bottom = line,
            Side::LeftRight => {
                self.left = line;
                self.right = line;
            }
            Side::TopBottom => {
                self.top = line;
                self.bottom = line;
            }
            Side::All => {
                self.left = line;
                self.top = line;
                self.right = line;
                self.bottom = line;
            }
        }
    }
}
