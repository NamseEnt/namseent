use namui_type::*;

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Episode {
    pub id: String,
    pub name: String,
    pub created_at: SystemTime,
}

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct TeamInviteCode {
    pub code: String,
    pub expiration_time: SystemTime,
}
