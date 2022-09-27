use super::*;
use crate::{pages::sequence_edit_page::timeline, storage::Circumscribed};
use std::sync::Arc;

impl PropertyEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::LayerListPlusButtonClicked { image_clip_address } => {
                    self.editor_history_system
                        .mutate_image_clip(image_clip_address, |clip| {
                            clip.images.push(rpc::data::system_tree::Image::new(
                                None,
                                Circumscribed {
                                    center: Xy::single(50.percent()),
                                    radius: 50.percent(),
                                },
                            ))
                        });
                }
                Event::ChangeImage {
                    image_clip_address,
                    image,
                    layer_index,
                } => {
                    self.editor_history_system
                        .mutate_image_clip(image_clip_address, |clip| {
                            clip.images.update(*layer_index, |layer| {
                                layer.image_path = Some(image.clone());
                            })
                        });
                }
            }
        } else if let Some(event) = event.downcast_ref::<timeline::Event>() {
            match event {
                timeline::Event::OpenContextMenu(_)
                | timeline::Event::CloseContextMenu
                | timeline::Event::NewImageClip => {}
                timeline::Event::SelectImageClip {
                    sequence_id,
                    cut_id,
                    image_clip_ids,
                } => {
                    if image_clip_ids.len() == 1 {
                        let image_clip_address = ImageClipAddress {
                            sequence_id,
                            cut_id,
                            image_clip_id: image_clip_ids.iter().next().unwrap().clone(),
                        };

                        self.content = PropertyContent::ImageClip {
                            image_clip_address: image_clip_address.clone(),
                            layer_list_view: list_view::ListView::new(),
                        };
                    } else {
                        self.content = PropertyContent::Nothing;
                    }
                }
                timeline::Event::DeselectImageClip => {
                    self.content = PropertyContent::Nothing;
                }
            }
        } else if let Some(event) = event.downcast_ref::<crate::pages::sequence_edit_page::Event>()
        {
            match event {
                crate::pages::sequence_edit_page::Event::SelectCut(_)
                | crate::pages::sequence_edit_page::Event::AddCutClicked => {}
                &crate::pages::sequence_edit_page::Event::LayerClicked { layer_index } => {
                    let content = std::mem::take(&mut self.content);

                    let (image_clip_address, mut image_browser, layer_list_view) = match content {
                        PropertyContent::Nothing => unreachable!(),
                        PropertyContent::ImageClip {
                            image_clip_address,
                            layer_list_view,
                        } => (
                            image_clip_address.clone(),
                            image_browser::ImageBrowser::new(self.storage.clone(), |_| {}),
                            layer_list_view,
                        ),
                        PropertyContent::ImageLayer {
                            image_clip_address,
                            image_browser,
                            layer_list_view,
                            layer_index: _,
                        } => (image_clip_address, image_browser, layer_list_view),
                    };

                    image_browser.on_item_click = Arc::new({
                        let image_clip_address = image_clip_address.clone();
                        move |image| {
                            namui::event::send(Event::ChangeImage {
                                image_clip_address: image_clip_address.clone(),
                                image: image.to_string(),
                                layer_index: layer_index.clone(),
                            })
                        }
                    });

                    self.content = PropertyContent::ImageLayer {
                        image_clip_address,
                        image_browser,
                        layer_list_view,
                        layer_index,
                    };
                }
            }
        }

        match &mut self.content {
            PropertyContent::Nothing => {}
            PropertyContent::ImageLayer {
                image_browser,
                layer_list_view,
                image_clip_address: _,
                layer_index: _,
            } => {
                image_browser.update(event);
                layer_list_view.update(event);
            }
            PropertyContent::ImageClip {
                image_clip_address: _,
                layer_list_view,
            } => {
                layer_list_view.update(event);
            }
        }
    }
}
