use crate::*;

impl Draw for ImageDrawCommand {
    fn draw(self, ctx: &DrawContext) {
        let Some(image) = ctx.skia.image(&self.source) else {
            start_load_image(&self.source);
            return;
        };

        let rect_wh = self.rect.wh();
        if rect_wh.width == 0.px()
            || rect_wh.height == 0.px()
            || image.wh.width == 0.px()
            || image.wh.height == 0.px()
        {
            return;
        }

        let image_size = Wh {
            width: image.wh.width,
            height: image.wh.height,
        };
        let (src_rect, dest_rect) = get_src_dest_rects_in_fit(self.fit, image_size, self.rect);

        ctx.canvas().save();

        ctx.canvas()
            .draw_image(&image.src, src_rect, dest_rect, &self.paint);

        ctx.canvas().restore();
    }
}

fn get_src_dest_rects_in_fit(
    image_fit: ImageFit,
    image_size: Wh<Px>,
    command_rect: Rect<Px>,
) -> (Rect<Px>, Rect<Px>) {
    match image_fit {
        ImageFit::Fill => (
            Rect::Xywh {
                x: 0.px(),
                y: 0.px(),
                width: image_size.width,
                height: image_size.height,
            },
            command_rect,
        ),
        ImageFit::Contain => {
            let dest_rect = calculate_contain_fit_dest_rect(image_size, command_rect);
            (
                Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: image_size.width,
                    height: image_size.height,
                },
                dest_rect,
            )
        }
        ImageFit::Cover => {
            let src_rect = calculate_cover_fit_src_rect(image_size, command_rect);
            (src_rect, command_rect)
        }
        ImageFit::None => calculate_none_fit_rects(image_size, command_rect),
        ImageFit::ScaleDown => {
            let (contain_fit_src, contain_fit_dest) =
                get_src_dest_rects_in_fit(ImageFit::Contain, image_size, command_rect);
            let (none_fit_src, none_fit_dest) =
                get_src_dest_rects_in_fit(ImageFit::None, image_size, command_rect);

            let contain_fit_dest_wh = contain_fit_dest.wh();
            let none_fit_dest_wh = none_fit_dest.wh();

            if contain_fit_dest_wh.width < none_fit_dest_wh.width
                || contain_fit_dest_wh.height < none_fit_dest_wh.height
            {
                (contain_fit_src, contain_fit_dest)
            } else {
                (none_fit_src, none_fit_dest)
            }
        }
    }
}

fn calculate_none_fit_rects(image_size: Wh<Px>, command_rect: Rect<Px>) -> (Rect<Px>, Rect<Px>) {
    let src_x = if image_size.width <= command_rect.width() {
        0.px()
    } else {
        (image_size.width - command_rect.width()) / 2.0
    };
    let src_y = if image_size.height <= command_rect.height() {
        0.px()
    } else {
        (image_size.height - command_rect.height()) / 2.0
    };
    let src_width = if image_size.width <= command_rect.width() {
        image_size.width
    } else {
        command_rect.width()
    };
    let src_height = if image_size.height <= command_rect.height() {
        image_size.height
    } else {
        command_rect.height()
    };
    let src_rect = Rect::Xywh {
        x: src_x,
        y: src_y,
        width: src_width,
        height: src_height,
    };

    let dest_center_x = command_rect.x() + command_rect.width() / 2.0;
    let dest_center_y = command_rect.y() + command_rect.height() / 2.0;
    let dest_x = dest_center_x - src_width / 2.0;
    let dest_y = dest_center_y - src_height / 2.0;
    let dest_rect = Rect::Xywh {
        x: dest_x,
        y: dest_y,
        width: src_width,
        height: src_height,
    };

    (src_rect, dest_rect)
}

fn calculate_contain_fit_dest_rect(image_size: Wh<Px>, command_rect: Rect<Px>) -> Rect<Px> {
    if image_size.width / image_size.height == command_rect.width() / command_rect.height() {
        return command_rect;
    }

    if image_size.width / image_size.height > command_rect.width() / command_rect.height() {
        let k = command_rect.width() / image_size.width;
        let delta_y = (command_rect.height() - image_size.height * k) / 2.0;
        return Rect::Xywh {
            x: command_rect.x(),
            y: command_rect.y() + delta_y,
            width: command_rect.width(),
            height: image_size.height * k,
        };
    }

    let k = command_rect.height() / image_size.height;
    let delta_x = (command_rect.width() - image_size.width * k) / 2.0;
    Rect::Xywh {
        x: command_rect.x() + delta_x,
        y: command_rect.y(),
        width: image_size.width * k,
        height: command_rect.height(),
    }
}

fn calculate_cover_fit_src_rect(image_size: Wh<Px>, command_rect: Rect<Px>) -> Rect<Px> {
    if image_size.width / image_size.height == command_rect.width() / command_rect.height() {
        return Rect::Xywh {
            x: 0.px(),
            y: 0.px(),
            width: image_size.width,
            height: image_size.height,
        };
    }

    if image_size.width / image_size.height > command_rect.width() / command_rect.height() {
        let k = command_rect.height() / image_size.height;
        let delta_x = (image_size.width * k - command_rect.width()) / (2.0 * k);
        return Rect::Xywh {
            x: delta_x,
            y: 0.px(),
            width: image_size.width - delta_x * 2.0,
            height: image_size.height,
        };
    }

    let k = command_rect.width() / image_size.width;
    let delta_y = (image_size.height * k - command_rect.height()) / (2.0 * k);
    Rect::Xywh {
        x: 0.px(),
        y: delta_y,
        width: image_size.width,
        height: image_size.height - delta_y * 2.0,
    }
}
