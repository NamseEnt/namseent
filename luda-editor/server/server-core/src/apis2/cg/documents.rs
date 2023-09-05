use rpc::data::CgFile;

#[document_macro::document]
pub struct CgDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub cg_id: rpc::Uuid,
    pub cg_file: CgFile,
}
