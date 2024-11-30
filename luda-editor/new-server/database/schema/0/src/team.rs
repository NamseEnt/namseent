use crate::*;
use std::collections::{BTreeSet, HashMap, HashSet};

#[document]
struct TeamDoc {
    #[id]
    id: u128,
    name: String,
    member_ids: HashSet<u128>,
    project_ids: HashSet<u128>,
    invite_codes: HashSet<u128>,
    asset_ids: HashSet<u128>,
    asset_bytes_limit: u64,
    asset_bytes_used: u64,
}

#[document]
struct TeamNameAssignDoc {
    #[id]
    team_name: String,
    team_id: u128,
}

#[document]
struct ProjectDoc {
    #[id]
    id: u128,
    name: String,
    team_id: u128,
    speakers: HashMap<SpeakerId, Speaker>,
    episode_ids: BTreeSet<u128>,
}

type SpeakerId = u128;

#[doc_part]
#[derive(PartialEq)]
#[rkyv(derive(PartialEq))]
struct Speaker {
    id: u128,
    name_l10n: HashMap<LanguageCode, String>,
}
type LanguageCode = String;
