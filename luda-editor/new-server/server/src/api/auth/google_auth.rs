use crate::*;
use rpc::auth::google_auth::*;

pub async fn google_auth(
    ArchivedRequest { jwt }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    todo!()
}
