#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tab {
    CharacterImage,
    CharacterPosition,
    BackgroundImage,
    BackgroundPosition,
}
pub const ALL_TABS: [Tab; 4] = [
    Tab::CharacterImage,
    Tab::CharacterPosition,
    Tab::BackgroundImage,
    Tab::BackgroundPosition,
];
impl Tab {
    pub fn get_name(&self) -> &'static str {
        match self {
            Tab::CharacterImage => "캐릭터 이미지",
            Tab::CharacterPosition => "캐릭터 위치",
            Tab::BackgroundImage => "배경 이미지",
            Tab::BackgroundPosition => "배경 위치",
        }
    }
}

pub enum TabEvent {
    ClickTabButton(Tab),
}
