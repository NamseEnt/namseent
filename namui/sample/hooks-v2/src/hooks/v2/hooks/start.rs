use super::*;

pub fn start<T: Component + 'static>(component: &'static T) {
    ctx::init();
    let mut rx = channel::init();

    let mut root_tree: ComponentTree = mount_visit(component);

    draw(&root_tree);

    namui::spawn_local(async move {
        while let Some(item) = rx.recv().await {
            match item {
                Item::SetStateItem(set_state_item) => {
                    let sig_id = set_state_item.sig_id();

                    root_tree = set_state_visit(
                        component,
                        root_tree,
                        set_state_item,
                        Arc::new(Mutex::new(vec![sig_id].into_iter().collect())),
                    );
                }
                Item::EventCallback(event_callback) => {
                    root_tree = event_visit(component, root_tree, event_callback);
                }
            }
            draw(&root_tree);
        }
    })
}
