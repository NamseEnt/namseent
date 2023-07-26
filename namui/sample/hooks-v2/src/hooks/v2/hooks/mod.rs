mod channel;
mod draw;
mod event;
mod hooks;
mod instance;
mod signal;
mod start;
mod value;

pub(crate) use channel::*;
pub use draw::*;
pub use event::*;
pub use hooks::*;
pub(crate) use instance::*;
use namui::RenderingTree;
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
    instance: Arc<ComponentInstance>,
    state_index: AtomicUsize,
    effect_index: AtomicUsize,
    memo_index: AtomicUsize,
    map_index: AtomicUsize,
    as_index: AtomicUsize,
}
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub(crate) fn new(instance: Arc<ComponentInstance>) -> Self {
        Self {
            instance,
            state_index: Default::default(),
            effect_index: Default::default(),
            memo_index: Default::default(),
            map_index: Default::default(),
            as_index: Default::default(),
        }
    }
}

fn mount_visit(component: &dyn Component) -> ComponentHolder {
    let component_instance = Arc::new(ComponentInstance::new(
        new_component_id(),
        component.static_type_id(),
        component.static_type_name(),
    ));

    return render(component_instance.clone(), component).component_holder;

    fn render(component_instance: Arc<ComponentInstance>, component: &dyn Component) -> RenderDone {
        ctx::set_up_before_render(component_instance);
        let done: RenderDone = component.render();
        ctx::clear_up_before_render();

        done
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
pub struct RenderDone {
    component_holder: ComponentHolder,
}

fn new_component_id() -> usize {
    static COMPONENT_ID: AtomicUsize = AtomicUsize::new(0);
    COMPONENT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub(crate) struct ComponentHolder {
    pub(crate) component_instance: Arc<ComponentInstance>,
    pub(crate) children: Vec<ComponentHolder>,
    pub(crate) rendering_tree: Option<RenderingTree>,
}

impl Debug for ComponentHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentHolder")
            .field("component_instance", &self.component_instance)
            .field("children", unsafe { &self.children.as_ptr().as_ref() })
            .finish()
    }
}

// pub enum RenderDone<'a> {
//     WithEvent {
//         event_context: Box<dyn 'static + Any>,
//         on_event: Box<dyn 'a + FnOnce(&dyn Any)>,
//         render: Box<dyn 'a + FnOnce(&dyn Any) -> Box<dyn 'a + Component>>,
//     },
//     WithoutEvent {
//         render: Box<dyn 'a + FnOnce() -> Box<dyn 'a + Component>>,
//     },
//     RenderingTree {
//         rendering_tree: &'a RenderingTree,
//     },
// }

// impl Debug for RenderDone {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RenderDone::WithEvent { .. } => write!(f, "RenderDone::WithEvent",),
//             RenderDone::WithoutEvent { .. } => {
//                 write!(f, "RenderDone::WithoutEvent",)
//             }
//             RenderDone::RenderingTree { rendering_tree } => {
//                 write!(
//                     f,
//                     "RenderDone::RenderingTree {{ rendering_tree: {:?} }}",
//                     rendering_tree
//                 )
//             }
//         }
//     }
// }

impl StaticType for RenderingTree {
    fn static_type_id(&self) -> TypeId {
        TypeId::of::<RenderingTree>()
    }
}

impl Component for RenderingTree {
    fn render(&self) -> RenderDone {
        use_render_with_rendering_tree(self.clone())
    }
    fn rendering_tree(&self) -> Option<RenderingTree> {
        Some(self.clone())
    }
}

pub trait Component: StaticType + Debug {
    fn render(&self) -> RenderDone;
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
