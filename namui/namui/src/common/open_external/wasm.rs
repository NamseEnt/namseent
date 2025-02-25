use anyhow::Result;

unsafe extern "C" {
    fn _open_external(url_ptr: *const u8, url_len: usize);
}
pub fn open_external(url: &str) -> Result<()> {
    let url_bytes = url.as_bytes();
    unsafe {
        _open_external(url_bytes.as_ptr(), url_bytes.len());
    }
    Ok(())
}
