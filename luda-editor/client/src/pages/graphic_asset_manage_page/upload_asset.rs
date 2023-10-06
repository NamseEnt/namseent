use super::start_fetch_graphic_assets;
use crate::{
    app::notification::{push_notification, remove_notification, Notification},
    components::{cg_upload::create_cg, image_upload::create_image},
};
use namui::prelude::*;

pub fn add_new_image(project_id: Uuid, png_bytes: Vec<u8>) {
    spawn_local(async move {
        let notification_id = push_notification(
            Notification::info("Uploading image...".to_string()).set_loading(true),
        );
        match create_image(project_id, png_bytes).await {
            Ok(_image_id) => {
                start_fetch_graphic_assets(project_id);
            }
            Err(error) => {
                push_notification(Notification::error(format!(
                    "Failed to upload image: {error}"
                )));
            }
        };

        remove_notification(notification_id);
    });
}

pub fn add_new_cg(project_id: Uuid, psd_name: String, psd_bytes: Vec<u8>) {
    spawn_local(async move {
        let notification_id = push_notification(
            Notification::info(format!("Uploading CG {psd_name}...")).set_loading(true),
        );
        match create_cg(project_id, psd_name, psd_bytes).await {
            Ok(_cg_file) => {
                start_fetch_graphic_assets(project_id);
            }
            Err(error) => {
                push_notification(Notification::error(format!("Failed to upload CG: {error}")));
            }
        }

        remove_notification(notification_id);
    });
}
