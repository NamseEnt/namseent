use namui::AudioAsset;
use rand::Rng;

const BUBBLE_POP_ASSETS: [AudioAsset; 14] = [
    crate::asset::sound::bubble_pop::BUBBLE_POP_00,
    crate::asset::sound::bubble_pop::BUBBLE_POP_01,
    crate::asset::sound::bubble_pop::BUBBLE_POP_02,
    crate::asset::sound::bubble_pop::BUBBLE_POP_03,
    crate::asset::sound::bubble_pop::BUBBLE_POP_04,
    crate::asset::sound::bubble_pop::BUBBLE_POP_05,
    crate::asset::sound::bubble_pop::BUBBLE_POP_06,
    crate::asset::sound::bubble_pop::BUBBLE_POP_07,
    crate::asset::sound::bubble_pop::BUBBLE_POP_08,
    crate::asset::sound::bubble_pop::BUBBLE_POP_09,
    crate::asset::sound::bubble_pop::BUBBLE_POP_10,
    crate::asset::sound::bubble_pop::BUBBLE_POP_11,
    crate::asset::sound::bubble_pop::BUBBLE_POP_12,
    crate::asset::sound::bubble_pop::BUBBLE_POP_13,
];

const MURCHUNGA_ASSETS: [AudioAsset; 9] = [
    crate::asset::sound::murchunga::MURCHUNGA_00,
    crate::asset::sound::murchunga::MURCHUNGA_01,
    crate::asset::sound::murchunga::MURCHUNGA_02,
    crate::asset::sound::murchunga::MURCHUNGA_03,
    crate::asset::sound::murchunga::MURCHUNGA_04,
    crate::asset::sound::murchunga::MURCHUNGA_05,
    crate::asset::sound::murchunga::MURCHUNGA_06,
    crate::asset::sound::murchunga::MURCHUNGA_07,
    crate::asset::sound::murchunga::MURCHUNGA_08,
];

const PICK_UP_CARDS_ASSETS: [AudioAsset; 6] = [
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_00,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_01,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_02,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_03,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_04,
    crate::asset::sound::pick_up_cards::PICK_UP_CARDS_05,
];

const LEVEL_UP_ASSETS: [AudioAsset; 1] = [crate::asset::sound::level_up::LEVEL_UP_00];

pub fn random_bubble_pop() -> AudioAsset {
    random_one(&BUBBLE_POP_ASSETS)
}

pub fn random_murchunga() -> AudioAsset {
    random_one(&MURCHUNGA_ASSETS)
}

pub fn random_pick_up_cards() -> AudioAsset {
    random_one(&PICK_UP_CARDS_ASSETS)
}

pub fn random_level_up() -> AudioAsset {
    random_one(&LEVEL_UP_ASSETS)
}

fn random_one(assets: &[AudioAsset]) -> AudioAsset {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..assets.len());
    assets[index]
}
