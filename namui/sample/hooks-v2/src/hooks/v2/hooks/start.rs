use super::*;

// struct TreeVisitCtx {
//     event_rx: channel::Receiver,
// }

// impl TreeVisitCtx {
//     fn mount_visit<T: Component>(&mut self, component: &T) {
//         let component_instance = Arc::new(ComponentInstance::new(component));

//         hooks::ctx::set_up_before_render(ContextFor::Mount, component_instance);

//         let render_ctx = RenderCtx::new(ContextFor::Mount, component_instance);
//         component.render(&render_ctx);
//     }
// }

pub fn start<T: Component>(component: &'static T) {
    // ctx::init();
    channel::init();

    // let mut tree_visit_ctx = TreeVisitCtx { event_rx: rx };

    // tree_visit_ctx.mount_visit(component);

    let tree_ctx = TreeContext::new();
    // let mut root_tree: ComponentTree =
    mount_visit(component, tree_ctx);

    // draw(&root_tree);

    // namui::spawn_local(async move {
    //     while let Some(item) = rx.recv().await {
    //         match item {
    //             Item::SetStateItem(set_state_item) => {
    //                 let sig_id = set_state_item.sig_id();

    //                 root_tree = set_state_visit(
    //                     component,
    //                     root_tree,
    //                     set_state_item,
    //                     Arc::new(Mutex::new(vec![sig_id].into_iter().collect())),
    //                 );
    //             }
    //             Item::EventCallback(event_callback) => {
    //                 root_tree = event_visit(component, root_tree, event_callback);
    //             }
    //         }
    //         draw(&root_tree);
    //     }
    // })
}
