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

/// TODO: Add double line
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    Single,
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
    pub(crate) left: Option<Line>,
    pub(crate) top: Option<Line>,
    pub(crate) right: Option<Line>,
    pub(crate) bottom: Option<Line>,
}

impl Borders {
    pub fn new() -> Self {
        Self {
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }

    pub(crate) fn add(&mut self, side: Side, line: Line) {
        match side {
            Side::Left => self.left = Some(line),
            Side::Top => self.top = Some(line),
            Side::Right => self.right = Some(line),
            Side::Bottom => self.bottom = Some(line),
            Side::LeftRight => {
                self.left = Some(line);
                self.right = Some(line);
            }
            Side::TopBottom => {
                self.top = Some(line);
                self.bottom = Some(line);
            }
            Side::All => {
                self.left = Some(line);
                self.top = Some(line);
                self.right = Some(line);
                self.bottom = Some(line);
            }
        }
    }
}
