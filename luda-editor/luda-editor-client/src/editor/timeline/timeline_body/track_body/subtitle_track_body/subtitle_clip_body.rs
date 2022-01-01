use crate::editor::{
    events::*,
    types::{SubtitleClip, Time},
    TimelineRenderContext,
};
use namui::prelude::*;

pub struct SubtitleClipBody {}
pub struct SubtitleClipBodyProps<'a> {
    pub track_body_wh: &'a Wh<f32>,
    pub clip: &'a SubtitleClip,
    pub context: &'a TimelineRenderContext<'a>,
}
impl SubtitleClipBody {
    pub fn render(props: &SubtitleClipBodyProps) -> RenderingTree {
        let SubtitleClipBodyProps { clip, context, .. } = props;
        let x = ((clip.start_at - context.start_at) / context.time_per_pixel).into_f32();
        let duration = context
            .subtitle_play_duration_measurer
            .get_play_duration(&clip.subtitle, &context.language);
        let width = (duration / context.time_per_pixel).into_f32();

        let is_out_of_bounds = x + width < 0.0 || x > props.track_body_wh.width;
        if is_out_of_bounds {
            return RenderingTree::Empty;
        }

        let is_highlight = props
            .context
            .selected_clip_id
            .as_ref()
            .map_or(false, |id| id.eq(&props.clip.id));

        let border_width = if is_highlight { 3 } else { 1 } * 2;
        let component_width = (Time::from_ms(200) / context.time_per_pixel).into_f32();
        let component_height = props.track_body_wh.height / 3.0;

        let head_position = namui::Xy { x: 0.0, y: 0.0 };

        let tail_position = namui::Xy {
            x: width - component_width,
            y: props.track_body_wh.height - component_height,
        };

        let color = Color::from_string_for_random_color(clip.id.as_str(), false);
        let brighter_color = color.brighter(0.2);

        let stroke_path = namui::Path::new()
            .move_to(
                head_position.x + component_width / 2.0,
                head_position.y + component_height,
            )
            .line_to(tail_position.x + component_width / 2.0, tail_position.y);

        let head_path = namui::Path::new()
            .move_to(0.0, 0.0)
            .line_to(0.0, component_height)
            .line_to(component_width, component_height)
            .line_to(component_width, component_height / 3.0)
            .close();

        let tail_path = namui::Path::new()
            .move_to(0.0, 0.0)
            .line_to(0.0, (component_height * 2.0) / 3.0)
            .line_to(component_width, component_height)
            .line_to(component_width, 0.0)
            .close();

        let fill_paint = namui::Paint::new().set_anti_alias(true).set_color(color);

        let border_paint = namui::Paint::new()
            .set_anti_alias(true)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(border_width as f32)
            .set_color(brighter_color);

        let stroke_fill_paint = namui::Paint::new()
            .set_anti_alias(true)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(border_width as f32 / 2.0)
            .set_stroke_cap(namui::StrokeCap::Round)
            .set_color(color);

        let stroke_border_paint = namui::Paint::new()
            .set_anti_alias(true)
            .set_style(namui::PaintStyle::Stroke)
            .set_stroke_width(border_width as f32)
            .set_stroke_cap(namui::StrokeCap::Round)
            .set_color(brighter_color);

        let head_rendering_tree = translate(
            head_position.x,
            head_position.y,
            namui::clip(
                head_path.clone(),
                namui::ClipOp::Intersect,
                render![
                    namui::path(head_path.clone(), fill_paint.clone()),
                    namui::path(head_path, border_paint.clone()),
                ],
            ),
        );
        let tail_rendering_tree = translate(
            tail_position.x,
            tail_position.y,
            namui::clip(
                tail_path.clone(),
                namui::ClipOp::Intersect,
                render![
                    namui::path(tail_path.clone(), fill_paint),
                    namui::path(tail_path, border_paint),
                ],
            ),
        );
        translate(
            x,
            0.0,
            render![
                path(stroke_path.clone(), stroke_border_paint),
                head_rendering_tree,
                tail_rendering_tree,
                path(stroke_path, stroke_fill_paint),
                // TODO
                //       AfterDraw(({ translated }) => {
                //         const mouse = engine.mousePosition.mousePosition;

                //         const mouseInHead = headPath.contains(
                //           mouse.x - translated.x - headPosition.x,
                //           mouse.y - translated.y - headPosition.y,
                //         );
                //         const mouseInTail = tailPath.contains(
                //           mouse.x - translated.x - tailPosition.x,
                //           mouse.y - translated.y - tailPosition.y,
                //         );
                //         const mouseIn = mouseInHead || mouseInTail;

                //         if (mouseInHead) {
                //           engine.mousePointer.setCursor(Cursor.grab);
                //         }

                //         if (mouseInTail) {
                //           engine.mousePointer.setCursor(Cursor.leftRightResize);
                //         }

                //         if (mouseIn) {
                //           timelineState.clipIdMouseIn = clip.id;
                //         }

                //         if (timelineState.clipIdMouseIn === clip.id && !mouseIn) {
                //           timelineState.clipIdMouseIn = undefined;
                //         }

                //         engine.mouseEvent.onMouseDown((mouseEvent) => {
                //           const mouseInHead = headPath.contains(
                //             mouseEvent.x - translated.x - headPosition.x,
                //             mouseEvent.y - translated.y - headPosition.y,
                //           );
                //           const mouseInTail = tailPath.contains(
                //             mouseEvent.x - translated.x - tailPosition.x,
                //             mouseEvent.y - translated.y - tailPosition.y,
                //           );
                //           const mouseIn = mouseInHead || mouseInTail;

                //           if (mouseIn) {
                //             timelineState.selectedClip = clip;
                //           } else if (
                //             timelineState.selectedClip?.id === clip.id &&
                //             !engine.render.isGlobalVectorOutOfRenderingData(
                //               Vector.from(mouseEvent),
                //               timelineState.timelineBorderId,
                //             )
                //           ) {
                //             timelineState.selectedClip = undefined;
                //           }

                //           if (timelineState.actionState) {
                //             return;
                //           }

                //           if (mouseInHead) {
                //             const mouseAnchorMs =
                //               (mouseEvent.x - translated.x) * timelineState.layout.msPerPixel;

                //             timelineState.actionState = {
                //               type: "moveClipTime",
                //               clipId: clip.id,
                //               anchorMs: mouseAnchorMs,
                //             };
                //           }

                //           if (mouseInTail) {
                //             timelineState.actionState = {
                //               type: "resizeClip",
                //               clipId: clip.id,
                //               side: "right",
                //               sashMouseAnchorMs:
                //                 (mouseEvent.x - translated.x - width) *
                //                 timelineState.layout.msPerPixel,
                //             };
                //           }
                //         });
                //       }),
                //     ],
                //   );
            ],
        )
    }
}
