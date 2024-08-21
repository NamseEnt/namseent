use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::get_speaker_names::*;

pub async fn get_speaker_names(
    ArchivedRequest {
        project_id,
        speaker_ids,
        language_code,
    }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
