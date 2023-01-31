#[document_macro::document]
pub struct ProjectDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub owner_id: rpc::Uuid,
    pub name: String,
    pub shared_data_json: String,
}

#[document_macro::document]
pub struct OwnerProjectDocument {
    #[pk]
    pub owner_id: rpc::Uuid,
    #[sk]
    pub project_id: rpc::Uuid,
}

#[document_macro::document]
pub struct UserInProjectAclDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub user_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

#[document_macro::document]
pub struct ProjectAclUserInDocument {
    #[pk]
    pub user_id: rpc::Uuid,
    #[sk]
    pub project_id: rpc::Uuid,
    pub permission: rpc::types::ProjectAclUserPermission,
}

// pub struct UserInProjectAclDocumentUpdate<Update, TUpdateFuture>
// where
//     Update: FnOnce(UserInProjectAclDocument) -> TUpdateFuture + 'static + Send,
//     TUpdateFuture: std::future::Future<Output = Result<UserInProjectAclDocument, ()>> + Send,
// {
//     pub pk_project_id: rpc::Uuid,
//     pub sk_user_id: rpc::Uuid,
//     pub update: Update,
// }

// impl<Update, TUpdateFuture>
//     Into<
//         crate::storage::dynamo_db::TransactUpdateCommand<
//             UserInProjectAclDocument,
//             Update,
//             TUpdateFuture,
//         >,
//     > for UserInProjectAclDocumentUpdate<Update, TUpdateFuture>
// where
//     Update: FnOnce(UserInProjectAclDocument) -> TUpdateFuture + 'static + Send,
//     TUpdateFuture: std::future::Future<Output = Result<UserInProjectAclDocument, ()>> + Send,
// {
//     fn into(
//         self,
//     ) -> crate::storage::dynamo_db::TransactUpdateCommand<
//         UserInProjectAclDocument,
//         Update,
//         TUpdateFuture,
//     > {
//         crate::storage::dynamo_db::TransactUpdateCommand {
//             partition_prefix: "project_acl_user_in".to_string(),
//             partition_key_without_prefix: self.pk_project_id.to_string(),
//             sort_key: Some(self.sk_user_id.to_string()),
//             update: self.update,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }

// pub struct ProjectAclUserInDocumentUpdate<Update, TUpdateFuture>
// where
//     Update: FnOnce(ProjectAclUserInDocument) -> TUpdateFuture + 'static + Send,
//     TUpdateFuture: std::future::Future<Output = Result<ProjectAclUserInDocument, ()>> + Send,
// {
//     pub pk_user_id: rpc::Uuid,
//     pub sk_project_id: rpc::Uuid,
//     pub update: Update,
// }

// impl<Update, TUpdateFuture>
//     Into<
//         crate::storage::dynamo_db::TransactUpdateCommand<
//             ProjectAclUserInDocument,
//             Update,
//             TUpdateFuture,
//         >,
//     > for ProjectAclUserInDocumentUpdate<Update, TUpdateFuture>
// where
//     Update: FnOnce(ProjectAclUserInDocument) -> TUpdateFuture + 'static + Send,
//     TUpdateFuture: std::future::Future<Output = Result<ProjectAclUserInDocument, ()>> + Send,
// {
//     fn into(
//         self,
//     ) -> crate::storage::dynamo_db::TransactUpdateCommand<
//         ProjectAclUserInDocument,
//         Update,
//         TUpdateFuture,
//     > {
//         crate::storage::dynamo_db::TransactUpdateCommand {
//             partition_prefix: "project_acl_user_in".to_string(),
//             partition_key_without_prefix: self.pk_user_id.to_string(),
//             sort_key: Some(self.sk_project_id.to_string()),
//             update: self.update,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }
