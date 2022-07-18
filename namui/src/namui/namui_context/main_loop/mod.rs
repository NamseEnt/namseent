use crate::{event::Event, *};
mod post_update_and_render;
mod pre_update_and_render;

impl NamuiContext {
    pub(crate) async fn run_main_loop<TProps>(
        mut self,
        state: &mut dyn Entity<Props = TProps>,
        props: &TProps,
    ) {
        loop {
            let event = self.event_receiver.recv().await.unwrap();
            self.event_count += 1;

            self.pre_update_and_render(&event);

            state.update(event.as_ref());
            let prev_rendering_tree = self.rendering_tree;
            self.rendering_tree = state.render(props);
            react::reconciliate(
                &prev_rendering_tree,
                &self.rendering_tree,
                Some(event.as_ref()),
            );

            self.post_update_and_render(&event);
        }
    }
}
