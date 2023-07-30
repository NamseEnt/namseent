// use crate::system::platform_utils::web::document;
// use wasm_bindgen::JsCast;
use web_sys::Blob;

pub trait Downloadable {
    fn to_blob(&self) -> Result<Blob, Box<dyn std::error::Error>>;
}

impl Downloadable for String {
    fn to_blob(&self) -> Result<Blob, Box<dyn std::error::Error>> {
        let array_sequence = {
            let array = js_sys::Array::new();
            array.push(&self.into());
            array
        };
        let blob = web_sys::Blob::new_with_str_sequence(&array_sequence.into())
            .map_err(|e| format!("Failed to create blob from string sequence, {:?}", e))?;
        Ok(blob)
    }
}

pub async fn download(
    filename: impl AsRef<str>,
    file: impl Downloadable,
) -> Result<(), Box<dyn std::error::Error>> {
    let blob = file.to_blob()?;
    let object_url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Failed to create object URL for blob: {:?}", e))?;

    todo!();
    // let a_tag = document()
    //     .create_element("a")
    //     .map_err(|e| format!("Failed to create <a> tag: {:?}", e))?
    //     .dyn_into::<web_sys::HtmlAnchorElement>()
    //     .map_err(|e| format!("Failed to cast <a> tag to HtmlAnchorElement: {:?}", e))?;

    // a_tag.set_href(&object_url);
    // a_tag.set_download(filename.as_ref());

    // document()
    //     .body()
    //     .unwrap()
    //     .append_child(&a_tag)
    //     .map_err(|e| format!("Failed to append <a> tag to body: {:?}", e))?;

    // a_tag.click();

    // a_tag.remove();
    // web_sys::Url::revoke_object_url(&object_url)
    //     .map_err(|e| format!("Failed to revoke object URL: {:?}", e))?;

    Ok(())
}
