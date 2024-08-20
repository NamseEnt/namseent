use crate::*;
use database::schema::*;
use luda_rpc::episode_editor::get_episode_texts::*;

pub async fn get_episode_texts(
    ArchivedRequest { }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    todo!()
}
