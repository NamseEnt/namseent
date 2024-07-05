use std::time::SystemTime;

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
    #[with(rkyv::with::UnixTimestamp)]
    pub created_at: SystemTime,
}

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct TeamInviteCode {
    pub code: String,
    #[with(rkyv::with::UnixTimestamp)]
    pub expiration_time: SystemTime,
}
