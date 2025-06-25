use super::*;
use ::tokio::sync::mpsc;
use std::sync::OnceLock;

pub fn fire_and_forget<E, P>(system: System<E, P>)
where
    E: Emitter<P> + 'static + Send + Sync,
    P: Particle<E> + 'static + Send,
{
    let tx = FIRE_AND_FORGET_TX
        .get()
        .expect("fire_and_forget is not initialized");
    tx.send(Box::new(system)).unwrap();
}

trait FireAndForgetSystem: Send + 'static {
    fn render(&self, ctx: &ComposeCtx, now: Instant);
    fn is_done(&self, now: Instant) -> bool;
}

impl<E, P> FireAndForgetSystem for System<E, P>
where
    E: Emitter<P> + 'static + Send + Sync,
    P: Particle<E> + 'static + Send,
{
    fn render(&self, ctx: &ComposeCtx, now: Instant) {
        System::render(self, ctx, now);
    }

    fn is_done(&self, now: Instant) -> bool {
        System::is_done(self, now)
    }
}

// Static channel for fire-and-forget systems
static FIRE_AND_FORGET_TX: OnceLock<mpsc::UnboundedSender<Box<dyn FireAndForgetSystem>>> =
    OnceLock::new();

pub struct FireAndForgetSystems;

impl Component for &FireAndForgetSystems {
    fn render(self, ctx: &RenderCtx) {
        let (systems, set_systems) = ctx.state::<Vec<Box<dyn FireAndForgetSystem>>>(Vec::new);

        ctx.interval("retain done systems", Duration::from_millis(1000), |_| {
            set_systems.mutate(|systems| {
                let now = Instant::now();
                systems.retain(|system| !system.is_done(now));
            });
        });

        ctx.async_effect("pull systems from channel", (), {
            |()| {
                let (req_tx, mut req_rx) = mpsc::unbounded_channel();
                FIRE_AND_FORGET_TX.set(req_tx).unwrap();

                async move {
                    while let Some(system) = req_rx.recv().await {
                        set_systems.mutate(|systems| {
                            systems.push(system);
                        });
                    }
                }
            }
        });

        let now = Instant::now();

        for system in systems.iter() {
            system.render(ctx, now)
        }
    }
}
