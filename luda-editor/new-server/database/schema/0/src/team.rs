use crate::*;

#[document]
struct TeamDoc {
    name: String,
    member_ids: Vec<u128>,
}

#[document]
struct TeamNameDoc {
    #[id]
    team_name: String,
    team_id: u128,
}
