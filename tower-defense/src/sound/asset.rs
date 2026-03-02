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
const PICKAXE_ASSETS: [AudioAsset; 3] = [
    crate::asset::sound::pickaxe::PICKAXE_00,
    crate::asset::sound::pickaxe::PICKAXE_01,
    crate::asset::sound::pickaxe::PICKAXE_02,
];

const LEVEL_UP_ASSETS: [AudioAsset; 1] = [crate::asset::sound::level_up::LEVEL_UP_00];
const ORCH_HIT_ASSETS: [AudioAsset; 1] = [crate::asset::sound::orch_hit::ORCH_HIT_00];
const FAIL_ASSETS: [AudioAsset; 1] = [crate::asset::sound::fail::FAIL_00];
const TRUMPET_FANFARES_CLEAN_ASSETS: [AudioAsset; 4] = [
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_00,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_01,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_02,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_03,
];
const TRUMPET_FANFARES_SILLY_ASSETS: [AudioAsset; 6] = [
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_04,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_05,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_06,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_07,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_08,
    crate::asset::sound::trumpet_fanfares::TRUMPET_FANFARES_09,
];
const TRUMPET_FANFARES_SILLY_PROBABILITY: f64 = 0.1;
const COIN_SOUNDS_ASSETS: [AudioAsset; 40] = [
    crate::asset::sound::coin_sounds::COIN_SOUNDS_00,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_01,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_02,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_03,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_04,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_05,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_06,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_07,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_08,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_09,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_10,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_11,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_12,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_13,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_14,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_15,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_16,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_17,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_18,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_19,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_20,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_21,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_22,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_23,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_24,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_25,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_26,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_27,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_28,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_29,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_30,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_31,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_32,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_33,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_34,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_35,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_36,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_37,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_38,
    crate::asset::sound::coin_sounds::COIN_SOUNDS_39,
];

pub fn random_bubble_pop() -> AudioAsset {
    random_one(&BUBBLE_POP_ASSETS)
}

pub fn random_murchunga() -> AudioAsset {
    random_one(&MURCHUNGA_ASSETS)
}

pub fn random_pick_up_cards() -> AudioAsset {
    random_one(&PICK_UP_CARDS_ASSETS)
}

pub fn random_pickaxe() -> AudioAsset {
    random_one(&PICKAXE_ASSETS)
}

pub fn random_level_up() -> AudioAsset {
    random_one(&LEVEL_UP_ASSETS)
}

pub fn random_orch_hit() -> AudioAsset {
    random_one(&ORCH_HIT_ASSETS)
}

pub fn random_fail() -> AudioAsset {
    random_one(&FAIL_ASSETS)
}

pub fn random_trumpet_fanfares() -> AudioAsset {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(TRUMPET_FANFARES_SILLY_PROBABILITY) {
        random_one(&TRUMPET_FANFARES_SILLY_ASSETS)
    } else {
        random_one(&TRUMPET_FANFARES_CLEAN_ASSETS)
    }
}

pub fn random_coin_sounds() -> AudioAsset {
    random_one(&COIN_SOUNDS_ASSETS)
}

fn random_one(assets: &[AudioAsset]) -> AudioAsset {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..assets.len());
    assets[index]
}
