use crate::tooltip::TooltipContent;
use namui::*;

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
        let (hovering, set_hovering) = ctx.state(|| false);

        let rendering_tree = ctx.ghost_add(component_key, component);
        let Some(bounding_box) = rendering_tree.bounding_box() else {
            return;
        };

        ctx.add(rendering_tree).attach_event(move |event| {
            let Event::MouseMove { event } = event else {
                return;
            };
            if event.is_local_xy_in() {
                if !*hovering {
                    set_hovering.set(true);
                    let Some(content) = on_enter() else {
                        return;
                    };

                    let origin = event.global_xy - event.local_xy();
                    crate::tooltip::show_tooltip(
                        *tooltip_id,
                        bounding_box + origin,
                        placement,
                        content,
                    );
                }
            } else if *hovering {
                set_hovering.set(false);
                crate::tooltip::hide_tooltip(*tooltip_id);
                on_exit();
            }
        });
    }
}
