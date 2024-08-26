use crate::*;

#[document]
struct EpisodeSpeakerSlotDoc {
    #[pk]
    user_id: String,
    #[pk]
    episode_id: String,
    speaker_ids: Vec<String>,
}
