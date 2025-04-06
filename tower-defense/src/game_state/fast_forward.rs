use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FastForwardMultiplier {
    #[default]
    X1,
    X2,
    X4,
    X8,
}
impl FastForwardMultiplier {
    pub fn next(self) -> Self {
        match self {
            FastForwardMultiplier::X1 => FastForwardMultiplier::X2,
            FastForwardMultiplier::X2 => FastForwardMultiplier::X4,
            FastForwardMultiplier::X4 => FastForwardMultiplier::X8,
            FastForwardMultiplier::X8 => FastForwardMultiplier::X8,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            FastForwardMultiplier::X1 => FastForwardMultiplier::X1,
            FastForwardMultiplier::X2 => FastForwardMultiplier::X1,
            FastForwardMultiplier::X4 => FastForwardMultiplier::X2,
            FastForwardMultiplier::X8 => FastForwardMultiplier::X4,
        }
    }
    pub fn time_scale(self) -> NonZeroUsize {
        NonZeroUsize::new(match self {
            FastForwardMultiplier::X1 => 1,
            FastForwardMultiplier::X2 => 2,
            FastForwardMultiplier::X4 => 4,
            FastForwardMultiplier::X8 => 8,
        })
        .unwrap()
    }
}
