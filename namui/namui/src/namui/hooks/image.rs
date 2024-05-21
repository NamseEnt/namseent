use crate::skia::load_image_from_url;
use anyhow::Result;
use namui_hooks::*;
use namui_skia::*;

pub type Load<T> = Option<Result<T>>;

pub trait ImageTrait {
    fn image(&self, url: impl AsRef<str>) -> Sig<Load<Image>, &Load<Image>>;
}

impl ImageTrait for RenderCtx<'_, '_> {
    fn image(&self, url: impl AsRef<str>) -> Sig<Load<Image>, &Load<Image>> {
        let url = self.track_eq(&url.as_ref().to_string());
        let (load, set_load) = self.state(|| Load::None);

        // without sig
        let loaded = load.is_some();

        self.effect(format!("Load image from {url}"), || {
            let url = (*url).clone();

            if loaded {
                set_load.set(None);
            }

            let set_load = set_load.cloned();
            let join_handle = crate::spawn(async move {
                let image = load_image_from_url(&url).await;
                set_load.set(Some(image));
            });

            EffectCleanUp::once(move || {
                join_handle.abort();
            })
        });

        load
    }
}
