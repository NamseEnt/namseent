use namui::*;

pub struct WithHoverArea<K: Into<AddKey>, C: Component> {
    pub component_key: K,
    pub component: C,
    pub placement: crate::tooltip::TooltipPlacement,
    pub content: crate::tooltip::TooltipContent,
}

impl<K, C> Component for WithHoverArea<K, C>
where
    K: Into<AddKey>,
    C: Component,
{
    fn render(self, ctx: &RenderCtx) {
        let Self {
            component_key,
            component,
            placement,
            content,
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
                    let origin = event.global_xy - event.local_xy();
                    crate::tooltip::show_tooltip(
                        *tooltip_id,
                        bounding_box + origin,
                        placement,
                        content.clone(),
                    );
                }
            } else if *hovering {
                set_hovering.set(false);
                crate::tooltip::hide_tooltip(*tooltip_id);
            }
        });
    }
}
