use crate::icon::IconKind;
use crate::theme::palette;
use crate::theme::typography::TypographyBuilder;

/// Typography Builder extension trait for rich text helpers
pub trait RichTextHelpers<'a> {
    fn with_range<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_damage_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_percentage_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_percentage_decrease<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_value_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_multiplier<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_speed_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_range_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_gold_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_card_rank<S: Into<String>>(self, rank: S) -> TypographyBuilder<'a>;
    fn with_heal_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_shield_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_special_item_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_health_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_health_loss<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_gold_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_gold_loss<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_positive_effect<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_negative_effect<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_neutral_stat<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_suit_color<S: Into<String>>(
        self,
        text: S,
        suit: crate::card::Suit,
    ) -> TypographyBuilder<'a>;
    fn with_attack_damage_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_attack_speed_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_attack_range_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_movement_speed_debuff_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_movement_speed_debuff_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_reduction_percentage<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_contract_risk<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_contract_reward<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_contract_duration<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_time_duration<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
}

impl<'a> RichTextHelpers<'a> for TypographyBuilder<'a> {
    fn with_range<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::BLUE)
    }

    fn with_attack_damage_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackDamage)
            .text(value.into())
            .color(palette::RED)
    }

    fn with_percentage_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("+{}%", value.into()))
            .color(palette::COMMON)
    }

    fn with_percentage_decrease<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("-{}%", value.into())).color(palette::RED)
    }

    fn with_value_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("+{}", value.into()))
            .color(palette::COMMON)
    }

    fn with_multiplier<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("x{}", value.into())).color(palette::BLUE)
    }

    fn with_attack_speed_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackSpeed)
            .text(value.into())
            .color(palette::YELLOW)
    }

    fn with_attack_range_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackRange)
            .text(value.into())
            .color(palette::BLUE)
    }

    fn with_gold_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::Gold)
            .text(value.into())
            .color(palette::YELLOW)
    }

    fn with_card_rank<S: Into<String>>(self, rank: S) -> TypographyBuilder<'a> {
        self.text(rank.into()).color(palette::EPIC)
    }

    fn with_heal_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("❤ {}", value.into()))
            .color(palette::COMMON)
    }

    fn with_shield_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::BLUE)
    }

    fn with_special_item_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::BLUE)
    }

    fn with_suit_color<S: Into<String>>(
        self,
        text: S,
        suit: crate::card::Suit,
    ) -> TypographyBuilder<'a> {
        use crate::card::Suit;
        let color = match suit {
            Suit::Spades | Suit::Clubs => palette::COMMON,
            Suit::Hearts | Suit::Diamonds => palette::RED,
        };
        self.text(text.into()).color(color)
    }

    fn with_attack_damage_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::RED)
    }

    fn with_attack_speed_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::YELLOW)
    }

    fn with_attack_range_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::BLUE)
    }

    fn with_movement_speed_debuff_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::RED)
    }

    fn with_movement_speed_debuff_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::RED)
    }

    fn with_reduction_percentage<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("{}%", value.into()))
            .color(palette::COMMON)
    }

    fn with_contract_risk<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::RED)
    }

    fn with_contract_reward<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::BLUE)
    }

    fn with_contract_duration<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::YELLOW)
    }

    fn with_time_duration<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::YELLOW)
    }

    fn with_health_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::RED).bold()
    }

    fn with_health_loss<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::RED).bold()
    }

    fn with_gold_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::YELLOW).bold()
    }

    fn with_gold_loss<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::YELLOW).bold()
    }

    fn with_positive_effect<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::YELLOW).bold()
    }

    fn with_negative_effect<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::RED).bold()
    }

    fn with_neutral_stat<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::BLUE).bold()
    }
}
