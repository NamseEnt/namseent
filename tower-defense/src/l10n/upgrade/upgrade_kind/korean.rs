use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Magnet => {
                    builder.static_text("자석");
                }
                crate::game_state::upgrade::UpgradeKind::CainSword { .. } => {
                    builder.static_text("케인소드");
                }
                crate::game_state::upgrade::UpgradeKind::LongSword { .. } => {
                    builder.static_text("롱소드");
                }
                crate::game_state::upgrade::UpgradeKind::Mace { .. } => {
                    builder.static_text("메이스");
                }
                crate::game_state::upgrade::UpgradeKind::ClubSword { .. } => {
                    builder.static_text("클럽");
                }
                crate::game_state::upgrade::UpgradeKind::Backpack => {
                    builder.static_text("배낭");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle => {
                    builder.static_text("주사위 꾸러미");
                }
                crate::game_state::upgrade::UpgradeKind::Spoon { .. } => {
                    builder.static_text("숟가락");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink => {
                    builder.static_text("에너지드링크");
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. } => {
                    builder.static_text("완벽한 도자기");
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { .. } => {
                    builder.static_text("젓가락 한개");
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { .. } => {
                    builder.static_text("젓가락 두개");
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { .. } => {
                    builder.static_text("만년필");
                }
                crate::game_state::upgrade::UpgradeKind::Brush { .. } => {
                    builder.static_text("붓");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("네잎클로버");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("토끼");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder.static_text("흑백");
                }
                crate::game_state::upgrade::UpgradeKind::Eraser => {
                    builder.static_text("지우개");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => {
                    builder.static_text("깨진 도자기");
                }
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Magnet => {
                    builder
                        .static_text("몬스터를 처치할 때 얻는 ")
                        .with_gold_icon("골드")
                        .static_text("가 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::CainSword { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::LongSword { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::Mace { damage_multiplier }
                | crate::game_state::upgrade::UpgradeKind::ClubSword { damage_multiplier } => {
                    builder
                        .static_text("특정 무늬 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::Backpack => {
                    builder
                        .static_text("상점에서 구매할 수 있는 슬롯이 ")
                        .with_positive_effect("1개")
                        .static_text(" 추가됩니다.");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle => {
                    builder
                        .static_text("매 라운드마다 사용할 수 있는 주사위 개수가 ")
                        .with_positive_effect("1개")
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::Spoon { damage_multiplier } => {
                    builder
                        .static_text("3장 이하로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { damage_multiplier } => {
                    builder
                        .static_text("리롤하지 않고 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { damage_multiplier } => {
                    builder
                        .static_text("홀수 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { damage_multiplier } => {
                    builder
                        .static_text("짝수 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { damage_multiplier } => {
                    builder
                        .static_text("숫자 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::Brush { damage_multiplier } => {
                    builder
                        .static_text("그림 카드로 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { damage_multiplier } => {
                    builder
                        .static_text("리롤하고 만든 타워의 ")
                        .with_attack_damage_stat("공격력")
                        .static_text("이 ")
                        .with_multiplier(format!("{damage_multiplier:.1}"))
                        .static_text(" 증가합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink => {
                    builder.static_text("상점 아이템의 가격이 할인됩니다.");
                }
                crate::game_state::upgrade::UpgradeKind::Eraser => {
                    builder.static_text("덱에서 숫자 카드 랭크를 순차적으로 제거합니다.");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("스트레이트 플러시를 4장으로 만들 수 있게 됩니다.");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("스트레이트에서 한 랭크를 건너뛸 수 있게 됩니다.");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder.static_text("모든 무늬를 같은 것으로 취급합니다.");
                }
            },
        }
    }
}
