// `ShopItemLayout` is a simple wrapper around layout parameters.  The heavy
// rendering logic lives in a submodule so that this file stays small.

pub(crate) struct ShopItemLayout<'a> {
    pub params: super::ShopItemLayoutParams<'a>,
}
