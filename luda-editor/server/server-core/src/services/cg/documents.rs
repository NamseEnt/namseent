use rpc::data::CgFile;

#[document_macro::document]
pub struct CgDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub cg_file: CgFile,
}

#[document_macro::document]
pub struct CgInProject {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub cg_id: rpc::Uuid,
}
