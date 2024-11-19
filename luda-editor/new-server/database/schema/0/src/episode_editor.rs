use crate::*;

#[document]
struct EpisodeSpeakerSlotDoc {
    #[id]
    user_id: u128,
    #[id]
    episode_id: u128,
    speaker_ids: Vec<String>,
}
