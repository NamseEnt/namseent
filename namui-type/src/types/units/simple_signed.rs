use num::Signed;

/// It is similar to `num::Signed`,
/// but it implements only `is_positive` and `is_negative`.
pub trait SimpleSigned {
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
}

impl<T: Signed> SimpleSigned for T {
    fn is_sign_positive(&self) -> bool {
        num::Signed::is_positive(self)
    }

    fn is_sign_negative(&self) -> bool {
        num::Signed::is_negative(self)
    }
}
