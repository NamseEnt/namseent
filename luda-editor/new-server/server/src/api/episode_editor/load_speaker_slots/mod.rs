use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::load_speaker_slots::*;

pub async fn load_speaker_slots(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
