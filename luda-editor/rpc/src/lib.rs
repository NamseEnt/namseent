pub mod data;
mod define_rpc;
pub mod utils;

pub use define_rpc::RpcFuture;
pub use namui_type::{uuid, Uuid};
pub use revert_json_patch as json_patch;
pub use rpc_macro::define_rpc;

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

define_rpc! {
    AuthService: {
        exchange_google_auth_code_to_access_token: {
            struct Request {
                pub code: String,
            }
            struct Response {}
            enum Error {
                Unknown(String)
            }
        },
        log_in_with_github_oauth_code: {
            struct Request {
                pub code: String,
            }
            struct Response {
                pub session_id: crate::Uuid,
            }
            enum Error {
                AlreadyLoggedIn,
                Unknown(String),
            }
        },
        validate_session: {
            struct Request {}
            struct Response {}
            enum Error {
                InvalidSession,
                Unknown(String),
            }
        },
        get_user_id: {
            struct Request {}
            struct Response {
                pub user_id: crate::Uuid,
            }
            enum Error {
                InvalidSession,
                Unknown(String)
            }
        },
    },
    SequenceService: {
        list_project_sequences: {
            struct Request {
                pub project_id: crate::Uuid,
            }
            struct Response {
                pub sequence_name_and_ids: Vec<SequenceNameAndId>,
            }
            pub struct SequenceNameAndId {
                pub name: String,
                pub id: crate::Uuid,
            }
            enum Error {
                Unknown(String),
            }
        },
        create_sequence: {
            struct Request {
                pub project_id: crate::Uuid,
                pub name: String,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_server_sequence: {
            struct Request {
                pub sequence_id: crate::Uuid,
                pub patch: revert_json_patch::Patch,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_client_sequence: {
            struct Request {
                pub sequence_id: crate::Uuid,
                pub sequence_json: serde_json::Value,
            }
            struct Response {
                pub patch: revert_json_patch::Patch,
            }
            enum Error {
                Unknown(String),
            }
        },
        get_sequence_and_project_shared_data: {
            struct Request {
                pub sequence_id: crate::Uuid,
            }
            struct Response {
                pub sequence_json: String,
                pub project_shared_data_json: String,
            }
            enum Error {
                Unknown(String),
            }
        },
        delete_sequence: {
            struct Request {
                pub sequence_id: crate::Uuid,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        rename_sequence: {
            struct Request {
                pub sequence_id: crate::Uuid,
                pub new_name: String,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    ImageService: {
        put_image_meta_data: {
            struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
                pub labels: Vec<crate::data::Label>,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        prepare_upload_image: {
            struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
            }
            struct Response {
                pub upload_url: String,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_images: {
            struct Request {
                pub project_id: crate::Uuid,
            }
            struct Response {
                pub images: Vec<crate::data::ImageWithLabels>
            }
            enum Error {
                Unknown(String),
            }
        },
        delete_image: {
            struct Request {
                pub project_id: crate::Uuid,
                pub image_id: crate::Uuid,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    ProjectService: {
        create_project: {
            struct Request {
                pub name: String,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        list_editable_projects: {
            pub struct EditableProject {
                pub id: crate::Uuid,
                pub name: String,
            }
            struct Request {
                pub start_after: Option<String>,
            }
            struct Response {
                pub projects: Vec<EditableProject>,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        edit_user_acl: {
            struct Request {
                pub project_id: crate::Uuid,
                pub user_id: crate::Uuid,
                pub permission: Option<crate::types::ProjectAclUserPermission>,
            }
            struct Response {}
            enum Error {
                Unauthorized,
                CannotSetOwnerPermission,
                Unknown(String),
            }
        },
        update_server_project_shared_data: {
            struct Request {
                pub project_id: crate::Uuid,
                pub patch: revert_json_patch::Patch,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        update_client_project_shared_data: {
            struct Request {
                pub project_id: crate::Uuid,
                pub project_shared_data_json: serde_json::Value,
            }
            struct Response {
                pub patch: revert_json_patch::Patch,
            }
            enum Error {
                Unknown(String),
            }
        },
    },
    CgService: {
        request_put_psd_presigned_url: {
            struct Request {
                pub project_id: crate::Uuid,
                pub psd_file_name: String,
                pub psd_file_size: usize,
            }
            struct Response {
                pub presigned_url: String,
                pub psd_id: crate::Uuid,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        complete_put_psd: {
            struct Request {
                pub project_id: crate::Uuid,
                pub psd_file_name: String,
                pub psd_id: crate::Uuid,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                PsdFileNotFound,
                WrongPsdFile(String),
                WrongPsdFileName,
                Unknown(String),
            }
        },
        list_cg_files: {
            struct Request {
                pub project_id: crate::Uuid,
            }
            struct Response {
                pub cg_files: Vec<crate::data::CgFile>,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
    },
    MemoService: {
        list_sequence_memos: {
            struct Request {
                pub sequence_id: crate::Uuid,
            }
            struct Response {
                pub memos: Vec<crate::data::Memo>,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        create_memo: {
            struct Request {
                pub sequence_id: crate::Uuid,
                pub cut_id: crate::Uuid,
                pub content: String,
            }
            struct Response {
                pub memo: crate::data::Memo,
            }
            enum Error {
                Unauthorized,
                Unknown(String),
            }
        },
        delete_memo: {
            struct Request {
                pub sequence_id: crate::Uuid,
                pub memo_id: crate::Uuid,
            }
            struct Response {
            }
            enum Error {
                Unauthorized,
                Forbidden,
                Unknown(String),
            }
        },
    },
}
