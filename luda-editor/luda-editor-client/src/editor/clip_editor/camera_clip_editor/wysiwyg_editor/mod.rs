use crate::editor::{events::EditorEvent, types::*};
use namui::prelude::*;
use std::sync::Arc;

pub struct WysiwygEditor {}

pub struct WysiwygEditorProps<'a> {
    pub xywh: XywhRect<f32>,
    pub camera_angle: &'a CameraAngle,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
}

impl WysiwygEditor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {

        //     engine.mouseEvent.onMouseUp(() => {
        //         state.wysiwygEditor.dragging = undefined;
        //     });
    }
    pub fn render(&self, props: &WysiwygEditorProps) -> RenderingTree {
        let container_size = Wh {
            width: props.xywh.width,
            height: props.xywh.height,
        };

        let image_url = props
            .camera_angle
            .character_pose_emotion
            .get_url(props.image_filename_objects);
        if image_url.is_none() {
            return RenderingTree::Empty;
        }
        let image_url = image_url.unwrap();

        let image = namui::managers().image_manager.clone().try_load(&image_url);
        if image.is_none() {
            return RenderingTree::Empty;
        }
        let image = image.unwrap();

        translate(
            props.xywh.x,
            props.xywh.y,
            render![
                rect(RectParam {
                    x: 0.0,
                    y: 0.0,
                    width: props.xywh.width,
                    height: props.xywh.height,
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: 2.0,
                            border_position: BorderPosition::Inside,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                render_outer_image(image.clone(), props.camera_angle, &container_size),
                render_inner_image(image.clone(), props.camera_angle, &container_size),
                // Resizer(state, { containerSize, imageSource }),
                // Croper(state),
            ],
        )
    }
}

// function getImageSource(
//     state: CameraAngleEditorState,
// ): ImageSource | undefined {
//     if (state.wysiwygEditor.resizer.source) {
//         return state.wysiwygEditor.resizer.source;
//     }

//     const image = engine.imageLoad.tryLoad(state.cameraAngle.imageSourceUrl);
//     if (image) {
//         const widthHeightRatio = image.width() / image.height();
//         state.wysiwygEditor.resizer.source = {
//             widthHeightRatio,
//         };

//         return state.wysiwygEditor.resizer.source;
//     }

//     return;
// }

// function keepWidthHeightRatio(
//     state: CameraAngleEditorState,
//     imageSource: ImageSource,
// ) {
//     const { widthHeightRatio } = imageSource;
//     const screenWhRatio = 16 / 9;

//     if (widthHeightRatio > 1) {
//         state.cameraAngle.source01Rect.height =
//             (state.cameraAngle.source01Rect.width * screenWhRatio) / widthHeightRatio;
//     } else {
//         state.cameraAngle.source01Rect.width =
//             (state.cameraAngle.source01Rect.height / screenWhRatio) *
//             widthHeightRatio;
//     }
// }

fn get_rect_in_container(
    point_rect_length_ratio: &PointRectLengthRatio,
    image_size: &Wh<f32>,
    container_size: &Wh<f32>,
) -> XywhRect<f32> {
    let length_of_container_rect =
        (container_size.width.powf(2.0) + container_size.height.powf(2.0)).sqrt();
    let length_of_result_rect = point_rect_length_ratio.rect_length * length_of_container_rect;

    let image_size_length = (image_size.width.powf(2.0) + image_size.height.powf(2.0)).sqrt();
    let image_width_length_ratio = image_size.width / image_size_length;
    let image_height_length_ratio = image_size.height / image_size_length;

    let image_width_length = image_width_length_ratio * length_of_result_rect;
    let image_height_length = image_height_length_ratio * length_of_result_rect;

    XywhRect {
        x: container_size.width * point_rect_length_ratio.x,
        y: container_size.height * point_rect_length_ratio.y,
        width: image_width_length,
        height: image_height_length,
    }
}
pub fn render_source_image(
    image: Arc<Image>,
    paint: Option<Paint>,
    container_size: &Wh<f32>,
    camera_angle: &CameraAngle,
) -> RenderingTree {
    let image_info = image.get_image_info();
    namui::image(ImageParam {
        xywh: get_rect_in_container(
            &camera_angle.source_point_rect_length_ratio,
            &Wh {
                width: image_info.width,
                height: image_info.height,
            },
            &container_size,
        ),
        style: ImageStyle {
            fit: ImageFit::Fill,
            paint,
        },
        source: ImageSource::Image(image),
    })
}

// const OuterImage: Render<CameraAngleEditorState, {}> = (state, props) => {
fn render_outer_image(
    image: Arc<Image>,
    camera_angle: &CameraAngle,
    container_size: &Wh<f32>,
) -> RenderingTree {
    let outside_image_paint = namui::Paint::new()
        .set_style(namui::PaintStyle::Fill)
        .set_color_filter(&namui::ColorFilter::blend(
            &Color::gary_scale_f01(0.5),
            &namui::BlendMode::Multiply,
        ));
    let image_size = image.size();

    namui::clip(
        namui::Path::new().add_rect(
            get_rect_in_container(
                &camera_angle.dest_point_rect_length_ratio,
                &image_size,
                &container_size,
            )
            .into_ltrb(),
        ),
        namui::ClipOp::Difference,
        namui::render![render_source_image(
            image,
            Some(outside_image_paint),
            container_size,
            camera_angle
        )],
    )
}

fn render_inner_image(
    image: Arc<Image>,
    camera_angle: &CameraAngle,
    container_size: &Wh<f32>,
) -> RenderingTree {
    // TODO
    //     engine.mouseEvent.onMouseMove((event) => {
    //         const { dragging } = state.wysiwygEditor;
    //         if (!dragging || dragging.targetId !== "move") {
    //             return;
    //         }
    //         const mouseVector = Vector.from(event);
    //         const diff = mouseVector.sub(dragging.lastMousePosition);
    //         state.cameraAngle.source01Rect.x += diff.x / props.containerSize.width;
    //         state.cameraAngle.source01Rect.y += diff.y / props.containerSize.height;
    //         dragging.lastMousePosition = mouseVector;
    //     });

    //     const destRect = getDestRect(state);
    //     return Clip(

    let image_size = image.size();

    namui::clip(
        namui::Path::new().add_rect(
            get_rect_in_container(
                &camera_angle.dest_point_rect_length_ratio,
                &image_size,
                &container_size,
            )
            .into_ltrb(),
        ),
        namui::ClipOp::Intersect,
        namui::render![render_source_image(
            image,
            None,
            container_size,
            camera_angle
        )],
    )
    // TODO
    //                 onMouseDown(event) {
    //                     state.wysiwygEditor.dragging = {
    //                         targetId: "move",
    //                         lastMousePosition: Vector.from(event),
    //                     };
    //                 },
    //                 onMouseIn() {
    //                     engine.mousePointer.setCursor(Cursor.move);
    //                 },
    //             },
    //         ],
    //     );
    // };
}
