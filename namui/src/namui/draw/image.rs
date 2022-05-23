use super::ImageDrawCommand;
use crate::{
    namui::{
        render::ImageFit,
        skia::{FilterMode, MipmapMode},
    },
    *,
};

pub fn draw_image(namui_context: &NamuiContext, command: &ImageDrawCommand) {
    let image = match &command.source {
        ImageSource::Url(url) => namui::managers().image_manager.try_load(&url),
        ImageSource::Image(image) => Some(image.clone()),
    };

    if image.is_none() {
        return;
    }
    let image = image.unwrap();

    let image_info = image.get_image_info();

    if command.xywh.width == 0.0
        || command.xywh.height == 0.0
        || image_info.width == 0.0
        || image_info.height == 0.0
    {
        return;
    }

    let image_size = Wh {
        width: image_info.width,
        height: image_info.height,
    };
    let (src_rect, dest_rect) = get_src_dest_rects_in_fit(command.fit, &image_size, &command.xywh);

    let paint = command
        .paint_builder
        .as_ref()
        .unwrap_or(&PaintBuilder::new())
        .build();

    namui_context.surface.canvas().draw_image_rect_options(
        &image,
        &src_rect,
        &dest_rect,
        FilterMode::Linear,
        MipmapMode::Linear,
        Some(&paint),
    );
}

fn get_src_dest_rects_in_fit(
    image_fit: ImageFit,
    image_size: &Wh<f32>,
    command_rect: &XywhRect<f32>,
) -> (XywhRect<f32>, XywhRect<f32>) {
    match image_fit {
        ImageFit::Fill => (
            XywhRect {
                x: 0.0,
                y: 0.0,
                width: image_size.width,
                height: image_size.height,
            },
            XywhRect {
                x: command_rect.x,
                y: command_rect.y,
                width: command_rect.width,
                height: command_rect.height,
            },
        ),
        ImageFit::Contain => {
            let dest_rect = calculate_contain_fit_dest_rect(image_size, command_rect);
            (
                XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: image_size.width,
                    height: image_size.height,
                },
                dest_rect,
            )
        }
        ImageFit::Cover => {
            let src_rect = calculate_cover_fit_src_rect(image_size, command_rect);
            (src_rect, *command_rect)
        }
        ImageFit::None => calculate_none_fit_rects(image_size, command_rect),
        ImageFit::ScaleDown => {
            let (contain_fit_src, contain_fit_dest) =
                get_src_dest_rects_in_fit(ImageFit::Contain, image_size, command_rect);
            let (none_fit_src, none_fit_dest) =
                get_src_dest_rects_in_fit(ImageFit::None, image_size, command_rect);
            if contain_fit_dest.width < none_fit_dest.width
                || contain_fit_dest.height < none_fit_dest.height
            {
                (contain_fit_src, contain_fit_dest)
            } else {
                (none_fit_src, none_fit_dest)
            }
        }
    }
}

fn calculate_none_fit_rects(
    image_size: &Wh<f32>,
    command_rect: &XywhRect<f32>,
) -> (XywhRect<f32>, XywhRect<f32>) {
    let src_x = if image_size.width <= command_rect.width {
        0.0
    } else {
        (image_size.width - command_rect.width) / 2.0
    };
    let src_y = if image_size.height <= command_rect.height {
        0.0
    } else {
        (image_size.height - command_rect.height) / 2.0
    };
    let src_width = if image_size.width <= command_rect.width {
        image_size.width
    } else {
        command_rect.width
    };
    let src_height = if image_size.height <= command_rect.height {
        image_size.height
    } else {
        command_rect.height
    };
    let src_rect = XywhRect {
        x: src_x,
        y: src_y,
        width: src_width,
        height: src_height,
    };

    let dest_center_x = command_rect.x + command_rect.width / 2.0;
    let dest_center_y = command_rect.y + command_rect.height / 2.0;
    let dest_x = dest_center_x - src_width / 2.0;
    let dest_y = dest_center_y - src_height / 2.0;
    let dest_rect = XywhRect {
        x: dest_x,
        y: dest_y,
        width: src_width,
        height: src_height,
    };

    (src_rect, dest_rect)
}

fn calculate_contain_fit_dest_rect(
    image_size: &Wh<f32>,
    command_rect: &XywhRect<f32>,
) -> XywhRect<f32> {
    if image_size.width / image_size.height == command_rect.width / command_rect.height {
        return *command_rect;
    }

    if image_size.width / image_size.height > command_rect.width / command_rect.height {
        let k = command_rect.width / image_size.width;
        let delta_y = (command_rect.height - k * image_size.height) / 2.0;
        return XywhRect {
            x: command_rect.x,
            y: command_rect.y + delta_y,
            width: command_rect.width,
            height: k * image_size.height,
        };
    }

    let k = command_rect.height / image_size.height;
    let delta_x = (command_rect.width - k * image_size.width) / 2.0;
    return XywhRect {
        x: command_rect.x + delta_x,
        y: command_rect.y,
        width: k * image_size.width,
        height: command_rect.height,
    };
}

fn calculate_cover_fit_src_rect(
    image_size: &Wh<f32>,
    command_rect: &XywhRect<f32>,
) -> XywhRect<f32> {
    if image_size.width / image_size.height == command_rect.width / command_rect.height {
        return XywhRect {
            x: 0.0,
            y: 0.0,
            width: image_size.width,
            height: image_size.height,
        };
    }

    if image_size.width / image_size.height > command_rect.width / command_rect.height {
        let k = command_rect.height / image_size.height;
        let delta_x = (k * image_size.width - command_rect.width) / (2.0 * k);
        return XywhRect {
            x: delta_x,
            y: 0.0,
            width: image_size.width - 2.0 * delta_x,
            height: image_size.height,
        };
    }

    let k = command_rect.width / image_size.width;
    let delta_y = (k * image_size.height - command_rect.height) / (2.0 * k);
    return XywhRect {
        x: 0.0,
        y: delta_y,
        width: image_size.width,
        height: image_size.height - 2.0 * delta_y,
    };
}
