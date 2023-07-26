mod channel;
mod draw;
mod effect;
mod event;
mod instance;
mod memo;
mod render;
mod signal;
mod start;
mod state;
mod value;

pub(crate) use channel::*;
use crossbeam::atomic::AtomicCell;
pub use draw::*;
use effect::*;
pub use event::*;
pub(crate) use instance::*;
pub(crate) use memo::*;
use namui::RenderingTree;
pub(crate) use render::*;
pub use signal::*;
pub use start::*;
pub use state::*;
use std::{
    any::{Any, TypeId},
    cell::{OnceCell, RefCell},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
pub use value::*;

pub(crate) enum ContextFor {
    Mount,
    Event {
        event_callback: EventCallback,
    },
    SetState {
        updated_signals: Arc<Mutex<HashSet<SignalId>>>,
    },
}

impl Debug for ContextFor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextFor::Mount => write!(f, "ContextFor::Mount"),
            ContextFor::Event { event_callback } => write!(
                f,
                "ContextFor::Event {{ event_callback: {:?} }}",
                event_callback
            ),
            ContextFor::SetState { updated_signals } => write!(
                f,
                "ContextFor::SetState {{ updated_signals: {:?} }}",
                updated_signals.lock().unwrap()
            ),
        }
    }
}

pub struct Context {
    context_for: ContextFor,
    instance: Arc<ComponentInstance>,
    state_index: AtomicUsize,
    effect_index: AtomicUsize,
    memo_index: AtomicUsize,
}

impl Context {
    pub(crate) fn new(context_for: ContextFor, instance: Arc<ComponentInstance>) -> Self {
        Self {
            context_for,
            instance,
            state_index: AtomicUsize::new(0),
            effect_index: AtomicUsize::new(0),
            memo_index: AtomicUsize::new(0),
        }
    }

    pub fn state<State: Send + Sync + Debug + 'static>(
        &self,
        init: impl FnOnce() -> State,
    ) -> (Signal<State>, SetState<State>) {
        handle_state(self, init)
    }

    pub fn effect(&self, name: &'static str, effect: impl FnOnce()) {
        let _ = name;
        handle_effect(self, effect);
    }

    pub fn render<'a, 'b, C: Component + 'b>(
        &self,
        render: impl 'a + FnOnce() -> C,
    ) -> ContextDone {
        match &self.context_for {
            ContextFor::Mount | ContextFor::SetState { .. } => {
                let child = handle_render(self, render);
                match child {
                    Some(child) => {
                        namui::log!("rendered");
                        ContextDone::Rendered { child }
                    }
                    None => {
                        namui::log!("no render");
                        ContextDone::NoRender
                    }
                }
            }
            ContextFor::Event { .. } => {
                unreachable!()
            }
        }
    }

    pub fn render_with_event<'me, 'a, 'b, C: Component + 'b, Event: 'static + Send + Sync>(
        &'me self,
        on_event: impl 'a + FnOnce(&Event),
        render: impl 'a + FnOnce(EventContext<Event>) -> C,
    ) -> ContextDone {
        match &self.context_for {
            ContextFor::Mount | ContextFor::SetState { .. } => {
                let child = handle_render_with_event(self, render);
                match child {
                    Some(child) => {
                        namui::log!("rendered");
                        ContextDone::Rendered { child }
                    }
                    None => {
                        namui::log!("no render");
                        ContextDone::NoRender
                    }
                }
            }
            ContextFor::Event { event_callback } => {
                assert_eq!(event_callback.component_id, self.instance.component_id);
                on_event(event_callback.event.downcast_ref().unwrap());
                ContextDone::NoRender
            }
        }
    }

    pub fn memo<T: 'static + Debug + Send + Sync>(&self, memo: impl FnOnce() -> T) -> Signal<T> {
        handle_memo(self, memo)
    }

    fn is_set_state_phase(&self) -> bool {
        match &self.context_for {
            ContextFor::Mount | ContextFor::Event { .. } => false,
            ContextFor::SetState { .. } => true,
        }
    }

    fn is_used_signal_updated(&self, signal_ids: &Vec<SignalId>) -> bool {
        let result = match &self.context_for {
            ContextFor::Mount | ContextFor::Event { .. } => unreachable!(),
            ContextFor::SetState { updated_signals } => {
                namui::log!(
                    "is_used_signal_updated, updated_signals: {:?}, signal_ids: {:?}",
                    updated_signals.lock().unwrap(),
                    signal_ids
                );
                let updated_signals = updated_signals.lock().unwrap();
                signal_ids
                    .into_iter()
                    .any(|signal_id| updated_signals.contains(&signal_id))
            }
        };
        namui::log!("is_used_signal_updated, result: {}", result);
        result
    }

    fn push_used_signal_on_this_ctx(&self, signal_id: SignalId) {
        match &self.context_for {
            ContextFor::Mount => {}
            ContextFor::Event { .. } => unreachable!(),
            ContextFor::SetState { updated_signals } => {
                updated_signals.lock().unwrap().insert(signal_id);
            }
        }
    }
}

pub struct EventContext<Event: 'static> {
    component_id: usize,
    _event: std::marker::PhantomData<Event>,
}

impl<Event: 'static + Send + Sync> EventContext<Event> {
    fn new(component_id: usize) -> Self {
        Self {
            component_id,
            _event: std::marker::PhantomData,
        }
    }
    pub fn event(&self, event: Event) -> EventCallback {
        EventCallback {
            component_id: self.component_id,
            event: Arc::new(event),
        }
    }
}

#[derive(Debug)]
pub enum ContextDone {
    Rendered { child: Box<dyn Component> },
    NoRender,
}

impl StaticType for RenderingTree {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<RenderingTree>()
    }
}

impl Component for RenderingTree {
    fn component<'a>(&'a self, _ctx: &'a Context) -> ContextDone {
        ContextDone::NoRender
    }
    fn rendering_tree(&self) -> Option<RenderingTree> {
        Some(self.clone())
    }
}

pub trait Component: StaticType + Debug + 'static {
    fn component<'a>(&'a self, ctx: &'a Context) -> ContextDone;
    fn rendering_tree(&self) -> Option<RenderingTree> {
        None
    }
}

pub trait StaticType {
    fn static_type_id(&self) -> TypeId;
    /// This would be not 'static
    fn static_type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

fn update_or_push<T>(vector: &mut Vec<T>, index: usize, value: T) {
    if let Some(prev) = vector.get_mut(index) {
        *prev = value;
    } else {
        assert_eq!(vector.len(), index);
        vector.insert(index, value);
    }
}
