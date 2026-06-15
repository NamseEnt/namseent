use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use namui::*;

/// Returns the overlay image corresponding to the given suit.
pub(super) fn tower_overlay_suit_image(suit: Suit) -> Image {
    suit.hand_drawn_image()
}

/// Returns the overlay image corresponding to the given rank.
pub(super) fn tower_overlay_rank_image(rank: Rank) -> Image {
    rank.hand_drawn_image()
}

/// Converts a tower kind + animation kind into the correct sprite image.
pub trait TowerImage {
    fn image(self) -> Image;
}

impl TowerImage for (TowerKind, super::AnimationKind) {
    fn image(self) -> Image {
        let (tower_kind, animation_kind) = self;
        match tower_kind {
            TowerKind::Barricade => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::barricade::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::barricade::IDLE1,
                super::AnimationKind::Attack => crate::asset::image::tower::barricade::IDLE1,
            },
            TowerKind::High => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::high::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::high::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::high::ATTACK,
            },
            TowerKind::OnePair => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::one_pair::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::one_pair::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::one_pair::ATTACK,
            },
            TowerKind::TwoPair => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::two_pair::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::two_pair::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::two_pair::ATTACK,
            },
            TowerKind::ThreeOfAKind => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::three_of_a_kind::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::three_of_a_kind::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::three_of_a_kind::ATTACK,
            },
            TowerKind::Straight => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::straight::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::straight::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::straight::ATTACK,
            },
            TowerKind::Flush => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::flush::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::flush::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::flush::ATTACK,
            },
            TowerKind::FullHouse => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::full_house::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::full_house::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::full_house::ATTACK,
            },
            TowerKind::FourOfAKind => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::four_of_a_kind::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::four_of_a_kind::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::four_of_a_kind::ATTACK,
            },
            TowerKind::StraightFlush => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::straight_flush::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::straight_flush::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::straight_flush::ATTACK,
            },
            TowerKind::RoyalFlush => match animation_kind {
                super::AnimationKind::Idle1 => crate::asset::image::tower::royal_flush::IDLE1,
                super::AnimationKind::Idle2 => crate::asset::image::tower::royal_flush::IDLE2,
                super::AnimationKind::Attack => crate::asset::image::tower::royal_flush::ATTACK,
            },
        }
    }
}
