use crate::*;
use std::collections::{HashMap, HashSet};

#[document]
struct TeamDoc {
    #[id]
    id: u128,
    name: String,
    member_ids: HashSet<u128>,
    projects: HashMap<ProjectId, Project>,
    invite_codes: HashSet<String>,
    asset_ids: HashSet<u128>,
    asset_bytes_limit: u64,
    asset_bytes_used: u64,
}

type ProjectId = u128;

#[document]
struct TeamNameAssignDoc {
    #[id]
    team_name: String,
    team_id: u128,
}

#[doc_part]
struct Project {
    id: u128,
    name: String,
    team_id: u128,
    speakers: HashMap<SpeakerId, Speaker>,
}

type SpeakerId = u128;

#[doc_part]
struct Speaker {
    id: u128,
    name_l10n: HashMap<LanguageCode, String>,
}
type LanguageCode = String;
