use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::try_edit_episode::*;

pub async fn try_edit_episode(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
