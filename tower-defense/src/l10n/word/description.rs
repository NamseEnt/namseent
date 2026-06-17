use crate::{
    l10n::{Language, Locale, LocalizedText, word::WordDescription},
    theme::typography::TypographyBuilder,
};

impl LocalizedText for WordDescription {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl WordDescription {
    fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self.0 {
            super::Word::Health => builder.with_style(|builder| {
                builder.static_text(
                    "적에게 공격을 당할 때 마다 감소합니다. 0이 되면 게임에서 패배합니다",
                );
            }),
            super::Word::Gold => builder.with_style(|builder| {
                builder.static_text(
                    "상점에서 상품 구매에 사용할 수 있습니다. 적을 처치할 때 획득합니다",
                );
            }),
            super::Word::Dice => builder.with_style(|builder| {
                builder.static_text(
                    "상점과 핸드에서 리롤할 때 소비됩니다. 매 스테이지 종료 시 사라지고, 매 스테이지 시작 시 리필됩니다",
                );
            }),
            super::Word::Item => builder.with_style(|builder| {
                builder.static_text("상점에서 구매할 수 있습니다. 화면 우측 인벤토리에 나열되고 클릭 시 사용됩니다. 사용시 사라집니다");
            }),
            super::Word::Treasure => builder.with_style(|builder| {
                builder.static_text("상점에서 구매하거나 보스 처치시 획득할 수 있습니다. 화면 좌측에 나열되고 효과가 자동으로 적용됩니다");
            }),
            super::Word::Shield => builder.with_style(|builder| {
                builder.static_text("체력 대신 감소합니다. 보호막은 스테이지 종료 시 사라집니다");
            }),
            super::Word::PerfectClear => builder.static_text("데미지를 입지않고 스테이지를 클리어"),
        };
    }

    fn apply_english<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self.0 {
            super::Word::Health => builder.with_style(|builder| {
                builder.static_text(
                    "Decreases whenever you are attacked by enemies. If it reaches 0, you lose the game",
                );
            }),
            super::Word::Gold => builder.with_style(|builder| {
                builder.static_text(
                    "Can be used to purchase items in the shop. Earned by defeating enemies",
                );
            }),
            super::Word::Dice => builder.with_style(|builder| {
                builder.static_text(
                    "Consumed when rerolling in the shop and hand. Disappears at the end of each stage and refills at the start of each stage",
                );
            }),
            super::Word::Item => builder.with_style(|builder| {
                builder.static_text("Can be purchased in the shop. Listed in the inventory on the right side of the screen and used by clicking on them. Disappears upon use");
            }),
            super::Word::Treasure => builder.with_style(|builder| {
                builder.static_text("Can be purchased in the shop or obtained by defeating bosses. Listed on the left side of the screen and automatically applied");
            }),
            super::Word::Shield => builder.with_style(|builder| {
                builder.static_text("Decreases instead of health. Shields disappear at the end of each stage");
            }),
            super::Word::PerfectClear => builder.static_text("Clearing a stage without taking any damage"),
        };
    }
}
