use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::get_speaker_names::*;

pub async fn get_speaker_names(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
