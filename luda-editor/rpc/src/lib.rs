pub mod data;
mod define_rpc;
pub mod utils;

pub use define_rpc::RpcFuture;
pub use namui_type::{uuid, Uuid};
pub use revert_json_patch as json_patch;

#[macro_export]
macro_rules! simple_error_impl {
    ($error_struct: ident) => {
        impl std::fmt::Display for $error_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        impl std::error::Error for $error_struct {}
    };
}

pub mod types {
    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    pub enum ProjectAclUserPermission {
        Editor,
    }
}

define_rpc::define_rpc! {
    AuthService: {
        exchange_google_auth_code_to_access_token: {
            pub struct Request {
                pub code: String,
            }
            pub struct Response {}
            Error {
                Unknown(String)
            }
        },
        log_in_with_github_oauth_code: {
            pub struct Request {
                pub code: String,
            }
            pub struct Response {
                pub session_id: crate::Uuid,
            }
            Error {
                AlreadyLoggedIn,
                Unknown(String),
            }
        },
        validate_session: {
            pub struct Request {}
            pub struct Response {}
            Error {
                InvalidSession,
                Unknown(String),
            }
        },
        get_user_id: {
            pub struct Request {}
            pub struct Response {
                pub user_id: crate::Uuid,
            }
            Error {
                InvalidSession,
                Unknown(String)
            }
        },
    },
    SequenceService: {
        list_project_sequences: {
            pub struct Request {
                pub project_id: crate::Uuid,
            }
            pub struct Response {
                pub sequence_name_and_ids: Vec<SequenceNameAndId>,
            }
            pub struct SequenceNameAndId {
                pub name: String,
                pub id: crate::Uuid,
            }
            Error {
                Unknown(String),
            }
        },
        create_sequence: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub name: String,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        undo_update: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Forbidden,
                NotFound,
                NoMoreUndo,
                Unknown(String),
            }
        },
        redo_update: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Forbidden,
                NotFound,
                NoMoreRedo,
                Unknown(String),
            }
        },
        update_sequence: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
                pub action: crate::data::SequenceUpdateAction,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Forbidden,
                SequenceNotFound,
                CutNotFound,
                Unknown(String),
            }
        },
        update_sequence_cut: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
                pub cut_id: crate::Uuid,
                pub action: crate::data::CutUpdateAction,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Forbidden,
                SequenceNotFound,
                CutNotFound,
                Unknown(String),
            }
        },
        get_sequence_and_project_shared_data: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
            }
            pub struct Response {
                pub sequence: crate::data::Sequence,
                pub project_shared_data_json: String,
            }
            Error {
                SequenceNotFound,
                Unknown(String),
            }
        },
        delete_sequence: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                SequenceNotFound,
                Unknown(String),
            }
        },
    },
    ImageService: {
        put_image_meta_data: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
                pub labels: Vec<crate::data::Label>,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        prepare_upload_image: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
            }
            pub struct Response {
                pub upload_url: String,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_images: {
            pub struct Request {
                pub project_id: crate::Uuid,
            }
            pub struct Response {
                pub images: Vec<crate::data::ImageWithLabels>
            }
            Error {
                Unknown(String),
            }
        },
        delete_image: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    ProjectService: {
        create_project: {
            pub struct Request {
                pub name: String,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_editable_projects: {
            pub struct EditableProject {
                pub id: crate::Uuid,
                pub name: String,
            }
            pub struct Request {
                pub start_after: Option<String>,
            }
            pub struct Response {
                pub projects: Vec<EditableProject>,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        edit_user_acl: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub user_id: crate::Uuid,
                pub permission: Option<crate::types::ProjectAclUserPermission>,
            }
            pub struct Response {}
            Error {
                Unauthorized,
                CannotSetOwnerPermission,
                Unknown(String),
            }
        },
        update_server_project_shared_data: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub patch: revert_json_patch::Patch,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_client_project_shared_data: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub project_shared_data_json: serde_json::Value,
            }
            pub struct Response {
                pub patch: revert_json_patch::Patch,
            }
            Error {
                Unknown(String),
            }
        },
    },
    CgService: {
        request_put_psd_presigned_url: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub psd_file_name: String,
                pub psd_file_size: usize,
            }
            pub struct Response {
                pub presigned_url: String,
                pub psd_id: crate::Uuid,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        complete_put_psd: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub psd_file_name: String,
                pub psd_id: crate::Uuid,
            }
            pub struct Response {
                pub cg_id: crate::Uuid,
            }
            Error {
                Unauthorized,
                PsdFileNotFound,
                WrongPsdFile(String),
                WrongPsdFileName,
                Unknown(String),
            }
        },
        list_cg_files: {
            pub struct Request {
                pub project_id: crate::Uuid,
            }
            pub struct Response {
                pub cg_files: Vec<crate::data::CgFile>,
            }
            Error {
                Unknown(String),
            }
        },
        get_cg_file: {
            pub struct Request {
                pub project_id: crate::Uuid,
                pub cg_id: crate::Uuid,
            }
            pub struct Response {
                pub cg_file: crate::data::CgFile,
            }
            Error {
                NotFound,
                Unknown(String),
            }
        },
    },
    MemoService: {
        list_sequence_memos: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
            }
            pub struct Response {
                pub memos: Vec<crate::data::Memo>,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        create_memo: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
                pub cut_id: crate::Uuid,
                pub content: String,
            }
            pub struct Response {
                pub memo: crate::data::Memo,
            }
            Error {
                Unauthorized,
                Unknown(String),
            }
        },
        delete_memo: {
            pub struct Request {
                pub sequence_id: crate::Uuid,
                pub memo_id: crate::Uuid,
            }
            pub struct Response {
            }
            Error {
                Unauthorized,
                Forbidden,
                Unknown(String),
            }
        },
    },
}
