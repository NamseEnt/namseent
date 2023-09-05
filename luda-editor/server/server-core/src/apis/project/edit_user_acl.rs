use super::shared::*;
use crate::documents::*;
use rpc::edit_user_acl::{Error, Request, Response};

pub async fn edit_user_acl(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::edit_user_acl::Result {
    todo!()
}
