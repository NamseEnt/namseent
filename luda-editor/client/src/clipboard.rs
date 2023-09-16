use futures::Future;
use namui::{clipboard::ClipboardItem, prelude::*};
use rpc::data::{Cut, ScreenCg};
use serde::de::DeserializeOwned;
use std::pin::Pin;

pub trait LudaEditorClipboardItem: Sized + serde::Serialize {
    fn type_name() -> &'static str;
    fn write_to_clipboard<'a>(&'a self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'a>> {
        Box::pin(write_json(self))
    }
}

pub trait TryReadLudaEditorClipboardItem<T> {
    fn try_read_from_clipboard<'a>(&'a self) -> Pin<Box<dyn Future<Output = Option<T>> + 'a>>;
}

impl LudaEditorClipboardItem for ScreenCg {
    fn type_name() -> &'static str {
        "web application/luda-editor-cg+json"
    }
}

impl LudaEditorClipboardItem for Cut {
    fn type_name() -> &'static str {
        "web application/luda-editor-cut+json"
    }
}

impl<T, U> TryReadLudaEditorClipboardItem<T> for U
where
    T: DeserializeOwned + LudaEditorClipboardItem + 'static,
    U: ClipboardItem,
{
    fn try_read_from_clipboard<'a>(&'a self) -> Pin<Box<dyn Future<Output = Option<T>> + 'a>> {
        Box::pin(try_read(self))
    }
}

async fn try_read<T, U>(_self: &U) -> Option<T>
where
    T: DeserializeOwned + LudaEditorClipboardItem,
    U: ClipboardItem,
{
    let type_name = T::type_name();
    if _self.types().iter().any(|type_| type_ == type_name) {
        let bytes = _self.get_type(type_name).await.unwrap();

        return Some(serde_json::from_slice::<T>(&bytes).unwrap());
    }
    None
}

async fn write_json<T>(_self: &T) -> anyhow::Result<()>
where
    T: serde::Serialize + LudaEditorClipboardItem,
{
    let type_name = T::type_name();
    match clipboard::write([(T::type_name(), serde_json::to_string(_self).unwrap())]).await {
        Ok(_) => anyhow::Ok(()),
        Err(_) => Err(anyhow!("Failed to copy `{type_name}` to clipboard")),
    }
}
