use crate::icon::IconKind;
use crate::theme::palette;
use crate::theme::typography::TypographyBuilder;

/// Typography Builder extension trait for rich text helpers
pub trait RichTextHelpers<'a> {
    fn with_range<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_attack_damage_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_percentage_increase<S: Into<String>>(&mut self, value: S)
    -> &mut TypographyBuilder<'a>;
    fn with_percentage_decrease<S: Into<String>>(&mut self, value: S)
    -> &mut TypographyBuilder<'a>;
    fn with_value_increase<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_multiplier<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_attack_speed_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_attack_range_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_gold_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_card_rank<S: Into<String>>(&mut self, rank: S) -> &mut TypographyBuilder<'a>;
    fn with_heal_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_shield_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_special_item_text<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a>;
    fn with_health_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_health_loss<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_gold_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_gold_loss<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_positive_effect<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_negative_effect<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_neutral_stat<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
    fn with_suit_color<S: Into<String>>(
        &mut self,
        text: S,
        suit: crate::card::Suit,
    ) -> &mut TypographyBuilder<'a>;
    fn with_attack_damage_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_attack_speed_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_attack_range_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_movement_speed_debuff_text<S: Into<String>>(
        &mut self,
        text: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_movement_speed_debuff_value<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_reduction_percentage<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a>;
    fn with_contract_risk<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a>;
    fn with_contract_reward<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a>;
    fn with_contract_duration<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a>;
    fn with_time_duration<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a>;
}

impl<'a> RichTextHelpers<'a> for TypographyBuilder<'a> {
    fn with_range<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::BLUE);
        });
        self
    }

    fn with_attack_damage_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackDamage);
        self.with_style(|b| {
            b.text(value.into()).color(palette::RED);
        });
        self
    }

    fn with_percentage_increase<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("+{}%", value.into())).color(palette::COMMON);
        });
        self
    }

    fn with_percentage_decrease<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("-{}%", value.into())).color(palette::RED);
        });
        self
    }

    fn with_value_increase<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("+{}", value.into())).color(palette::COMMON);
        });
        self
    }

    fn with_multiplier<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("x{}", value.into())).color(palette::BLUE);
        });
        self
    }

    fn with_attack_speed_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackSpeed);
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW);
        });
        self
    }

    fn with_attack_range_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackRange);
        self.with_style(|b| {
            b.text(value.into()).color(palette::BLUE);
        });
        self
    }

    fn with_gold_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.icon::<()>(IconKind::Gold);
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW);
        });
        self
    }

    fn with_card_rank<S: Into<String>>(&mut self, rank: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(rank.into()).color(palette::EPIC);
        });
        self
    }

    fn with_heal_icon<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("❤ {}", value.into())).color(palette::COMMON);
        });
        self
    }

    fn with_shield_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::BLUE);
        });
        self
    }

    fn with_special_item_text<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(text.into()).color(palette::BLUE);
        });
        self
    }

    fn with_suit_color<S: Into<String>>(
        &mut self,
        text: S,
        suit: crate::card::Suit,
    ) -> &mut TypographyBuilder<'a> {
        use crate::card::Suit;
        let color = match suit {
            Suit::Spades | Suit::Clubs => palette::COMMON,
            Suit::Hearts | Suit::Diamonds => palette::RED,
        };
        self.with_style(|b| {
            b.text(text.into()).color(color);
        });
        self
    }

    fn with_attack_damage_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(stat_name.into()).color(palette::RED);
        });
        self
    }

    fn with_attack_speed_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(stat_name.into()).color(palette::YELLOW);
        });
        self
    }

    fn with_attack_range_stat<S: Into<String>>(
        &mut self,
        stat_name: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(stat_name.into()).color(palette::BLUE);
        });
        self
    }

    fn with_movement_speed_debuff_text<S: Into<String>>(
        &mut self,
        text: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(text.into()).color(palette::RED);
        });
        self
    }

    fn with_movement_speed_debuff_value<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::RED);
        });
        self
    }

    fn with_reduction_percentage<S: Into<String>>(
        &mut self,
        value: S,
    ) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(format!("{}%", value.into())).color(palette::COMMON);
        });
        self
    }

    fn with_contract_risk<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(text.into()).color(palette::RED);
        });
        self
    }

    fn with_contract_reward<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(text.into()).color(palette::BLUE);
        });
        self
    }

    fn with_contract_duration<S: Into<String>>(&mut self, text: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(text.into()).color(palette::YELLOW);
        });
        self
    }

    fn with_time_duration<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW);
        });
        self
    }

    fn with_health_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::RED).bold();
        });
        self
    }

    fn with_health_loss<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::RED).bold();
        });
        self
    }

    fn with_gold_value<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW).bold();
        });
        self
    }

    fn with_gold_loss<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW).bold();
        });
        self
    }

    fn with_positive_effect<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::YELLOW).bold();
        });
        self
    }

    fn with_negative_effect<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::RED).bold();
        });
        self
    }

    fn with_neutral_stat<S: Into<String>>(&mut self, value: S) -> &mut TypographyBuilder<'a> {
        self.with_style(|b| {
            b.text(value.into()).color(palette::BLUE).bold();
        });
        self
    }
}
