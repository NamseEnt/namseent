mod description;
mod name;

#[derive(Clone, Copy)]
pub enum Word {
    Health,
    Gold,
    Dice,
    Item,
    Treasure,
    Shield,
}

#[derive(Clone, Copy)]
pub struct WordName(Word);

#[derive(Clone, Copy)]
pub struct WordDescription(Word);

impl Word {
    pub fn name(&self) -> WordName {
        WordName(*self)
    }

    pub fn description(&self) -> WordDescription {
        WordDescription(*self)
    }
}
