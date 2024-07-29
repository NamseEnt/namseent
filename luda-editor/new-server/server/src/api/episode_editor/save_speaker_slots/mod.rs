use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::save_speaker_slots::*;

pub async fn save_speaker_slots(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
