use namui_type::to_hashed_id;

pub fn id_to_u128<T: Id128>(id: &T) -> u128 {
    id.as_u128()
}

pub trait Id128 {
    fn as_u128(&self) -> u128;
}

impl Id128 for u128 {
    fn as_u128(&self) -> u128 {
        *self
    }
}

impl Id128 for (u128, u128) {
    fn as_u128(&self) -> u128 {
        to_hashed_id([self.0.to_le_bytes(), self.1.to_le_bytes()].concat())
    }
}

impl Id128 for String {
    fn as_u128(&self) -> u128 {
        to_hashed_id(self)
    }
}

impl Id128 for &str {
    fn as_u128(&self) -> u128 {
        to_hashed_id(self)
    }
}
