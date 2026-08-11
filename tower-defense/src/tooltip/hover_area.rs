use crate::tooltip::TooltipContent;
use namui::*;

fn leave<ExitFn: Fn() + ?Sized>(
    is_hovering: bool,
    set_hovering: SetState<bool>,
    tooltip_id: crate::tooltip::TooltipId,
    on_exit: &ExitFn,
) {
    if is_hovering {
        set_hovering.set(false);
        crate::tooltip::hide_tooltip(tooltip_id);
        on_exit();
    }
}

pub struct WithHoverArea<
    K: Into<AddKey>,
    C: Component,
    EnterFn: Fn() -> Option<TooltipContent>,
    ExitFn: Fn(),
> {
    pub component_key: K,
    pub component: C,
    pub placement: crate::tooltip::TooltipPlacement,
    pub on_enter: EnterFn,
    pub on_exit: ExitFn,
}

impl<K, C, EnterFn, ExitFn> Component for WithHoverArea<K, C, EnterFn, ExitFn>
where
    K: Into<AddKey>,
    C: Component,
    EnterFn: Fn() -> Option<TooltipContent>,
    ExitFn: Fn(),
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            component_key,
            component,
            placement,
            on_enter,
            on_exit,
        } = self;
        let (tooltip_id, _) = ctx.state(crate::tooltip::TooltipId::new);
        let tooltip_id = *tooltip_id;
        let (hovering, set_hovering) = ctx.state(|| false);
        let dismiss_revision = crate::tooltip::dismiss_revision(ctx);

        ctx.effect("hide tooltip on unmount", || {
            move || crate::tooltip::hide_tooltip(tooltip_id)
        });
        ctx.effect("reset hover on tooltip dismissal", || {
            dismiss_revision.record_as_used();
            leave(hovering.clone_inner(), set_hovering, tooltip_id, &on_exit);
        });
        let rendering_tree = ctx.ghost_add(component_key, component);
        let Some(bounding_box) = rendering_tree.bounding_box() else {
            return;
        };

        ctx.add(rendering_tree)
            .attach_event(move |event| match event {
                Event::MouseMove { event } if event.is_local_xy_in() => {
                    if !*hovering {
                        set_hovering.set(true);
                        let Some(content) = on_enter() else {
                            return;
                        };

                        let origin = event.global_xy - event.local_xy();
                        crate::tooltip::show_tooltip(
                            tooltip_id,
                            bounding_box + origin,
                            placement,
                            content,
                        );
                    }
                }
                Event::MouseMove { .. } => {
                    leave(*hovering, set_hovering, tooltip_id, &on_exit);
                }
                Event::MouseUp { event } if event.is_local_xy_in() && *hovering => {
                    leave(true, set_hovering, tooltip_id, &on_exit);
                }
                _ => {}
            });
    }
}
