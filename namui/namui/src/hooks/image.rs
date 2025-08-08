use crate::{ResourceLocation, skia::load_image_from_resource_location};
use anyhow::Result;
use namui_hooks::*;
use namui_skia::*;
use std::ops::Deref;

pub type Load<T> = Option<Result<T>>;

pub trait ImageTrait {
    fn image(&self, resource_location: impl AsRef<ResourceLocation>) -> Sig<'_, Load<Image>>;
}

impl ImageTrait for RenderCtx<'_, '_> {
    fn image(&self, resource_location: impl AsRef<ResourceLocation>) -> Sig<'_, Load<Image>> {
        let resource_location = self.track_eq(resource_location.as_ref());
        let (load, set_load) = self.state(|| Load::None);

        // without sig
        let loaded = load.is_some();

        self.effect(format!("Load image from {resource_location}"), || {
            let resource_location = resource_location.deref().clone();

            if loaded {
                set_load.set(None);
            }

            let join_handle = crate::spawn(async move {
                let image = load_image_from_resource_location(resource_location).await;
                set_load.set(Some(image));
            });

            EffectCleanUp::once(move || {
                join_handle.abort();
            })
        });

        load
    }
}
