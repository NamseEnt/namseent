use super::InitResult;

pub fn log(content: impl AsRef<str>) {
    println!("{}", content.as_ref());
}
pub(crate) async fn init() -> InitResult {
    Ok(())
}
