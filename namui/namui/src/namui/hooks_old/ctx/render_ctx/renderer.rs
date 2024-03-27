// use super::*;

// pub(super) struct Renderer {
//     pub(super) instance: &'static mut ComponentInstance,
// }

// impl Renderer {
//     pub(crate) fn from_instance_id(instance_id: usize) -> Self {
//         let instance = get_instance(instance_id);
//         Self { instance }
//     }
//     pub(super) fn render(&mut self, key: Option<Key>, component: impl Component) -> RenderingTree {
//         let child_instance = self
//             .instance
//             .get_or_create_child_instance(key, component.static_type_name());
//         global_state::tree_ctx().render_component(component, child_instance)
//     }
// }
