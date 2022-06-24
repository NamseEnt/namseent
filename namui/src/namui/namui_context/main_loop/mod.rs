use crate::{event::Event, *};
mod post_update_and_render;
mod pre_update_and_render;

impl NamuiContext {
    pub(crate) async fn run_main_loop<TProps>(
        mut self,
        state: &mut dyn Entity<Props = TProps>,
        props: &TProps,
    ) {
        let mut event_count = 0;

        loop {
            let event = self.event_receiver.recv().await.unwrap();
            event_count += 1;

            self.pre_update_and_render(&event);

            state.update(event.as_ref());
            self.rendering_tree = state.render(props);

            self.post_update_and_render(&event);
        }
    }
}
