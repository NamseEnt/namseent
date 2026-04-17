use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::upgrade::UpgradeKindText;
use crate::theme::typography::TypographyBuilder;

impl UpgradeKindText<'_> {
    pub fn apply_korean<'a>(self, builder: &mut TypographyBuilder<'a>) {
        match self {
            UpgradeKindText::Name(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Cat { .. } => {
                    builder.static_text("고양이");
                }
                crate::game_state::upgrade::UpgradeKind::Staff { .. } => {
                    builder.static_text("지팡이");
                }
                crate::game_state::upgrade::UpgradeKind::LongSword { .. } => {
                    builder.static_text("롱소드");
                }
                crate::game_state::upgrade::UpgradeKind::Mace { .. } => {
                    builder.static_text("메이스");
                }
                crate::game_state::upgrade::UpgradeKind::ClubSword { .. } => {
                    builder.static_text("몽둥이");
                }
                crate::game_state::upgrade::UpgradeKind::Backpack { .. } => {
                    builder.static_text("배낭");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle { .. } => {
                    builder.static_text("주사위 꾸러미");
                }
                crate::game_state::upgrade::UpgradeKind::Tricycle { .. } => {
                    builder.static_text("세발자전거");
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink { .. } => {
                    builder.static_text("에너지드링크");
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. } => {
                    builder.static_text("완벽한 도자기");
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { .. } => {
                    builder.static_text("젓가락");
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { .. } => {
                    builder.static_text("젓가락 세트");
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
                crate::game_state::upgrade::UpgradeKind::Eraser { .. } => {
                    builder.static_text("지우개");
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => {
                    builder.static_text("깨진 도자기");
                }
            },
            UpgradeKindText::Description(upgrade_kind) => match upgrade_kind {
                crate::game_state::upgrade::UpgradeKind::Cat { add } => {
                    builder
                        .static_text("몬스터 처치 시 ")
                        .with_icon_bold(IconKind::Gold, format!("{add}"));
                }
                crate::game_state::upgrade::UpgradeKind::Staff { damage_multiplier } => {
                    builder
                        .static_text("다이아몬드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::LongSword { damage_multiplier } => {
                    builder
                        .static_text("스페이드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Mace { damage_multiplier } => {
                    builder
                        .static_text("하트 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::ClubSword { damage_multiplier } => {
                    builder
                        .static_text("클럽 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Backpack { .. } => {
                    builder.with_icon_bold(IconKind::Shop, "상점 슬롯 +1");
                }
                crate::game_state::upgrade::UpgradeKind::DiceBundle { .. } => {
                    builder.with_icon_bold(IconKind::Refresh, "+1");
                }
                crate::game_state::upgrade::UpgradeKind::Tricycle { damage_multiplier } => {
                    builder
                        .static_text("3장 이하 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::PerfectPottery { damage_multiplier } => {
                    builder
                        .static_text("리롤 안한 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::SingleChopstick { damage_multiplier } => {
                    builder
                        .static_text("홀수 카드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::PairChopsticks { damage_multiplier } => {
                    builder
                        .static_text("짝수 카드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::FountainPen { damage_multiplier } => {
                    builder
                        .static_text("숫자 카드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::Brush { damage_multiplier } => {
                    builder
                        .static_text("그림 카드 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::BrokenPottery { damage_multiplier } => {
                    builder
                        .static_text("리롤한 타워 ")
                        .with_icon_bold(IconKind::Damage, format!("X{damage_multiplier:.1}"));
                }
                crate::game_state::upgrade::UpgradeKind::EnergyDrink { add } => {
                    builder
                        .static_text("상점 가격 ")
                        .with_icon_bold(IconKind::Gold, format!("{add}"))
                        .static_text(" 할인");
                }
                crate::game_state::upgrade::UpgradeKind::Eraser { .. } => {
                    builder.static_text("덱에서 2부터 하나씩 숫자카드를 제거합니다");
                }
                crate::game_state::upgrade::UpgradeKind::FourLeafClover => {
                    builder.static_text("스트레이트와 플러시를 4장으로 만들 수 있습니다");
                }
                crate::game_state::upgrade::UpgradeKind::Rabbit => {
                    builder.static_text("스트레이트를 만들 때 하나를 건너뛸 수 있습니다");
                }
                crate::game_state::upgrade::UpgradeKind::BlackWhite => {
                    builder
                        .static_text("하트와 다이아를, 클럽과 스페이드를 같은 문양으로 간주합니다");
                }
            },
        }
    }
}
