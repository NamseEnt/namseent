pub use super::Act;
use super::*;
pub use namui::animation::Animation;
use std::{
    error::Error,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

#[derive(Clone)]
pub struct AnimationHistory {
    history_system: Arc<Mutex<HistorySystem<Animation>>>,
    working_ticket: Arc<Mutex<Option<ActionTicket>>>,
}

pub enum Event {
    ActionUpdated,
}

impl AnimationHistory {
    pub fn new(animation: Animation) -> Self {
        Self {
            history_system: Arc::new(Mutex::new(HistorySystem::new(animation))),
            working_ticket: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn get_preview(&self) -> Arc<Animation> {
        let history = self.history_system.lock().unwrap();
        history.get_preview()
    }

    pub(crate) fn try_set_action(&self, action: impl Act<Animation>) -> Option<ActionTicket> {
        let mut history = self.history_system.lock().unwrap();
        if history.has_action() {
            None
        } else {
            history.set_action(action);
            let action_ticket = ActionTicket::new();
            *self.working_ticket.lock().unwrap() = Some(action_ticket);
            namui::event::send(Event::ActionUpdated);
            Some(action_ticket)
        }
    }

    pub(crate) fn act(&self, action_ticket: ActionTicket) -> Result<Arc<Animation>, ActError> {
        if Some(action_ticket) != *self.working_ticket.lock().unwrap() {
            return Err(ActError::WrongTicket);
        }
        *self.working_ticket.lock().unwrap() = None;

        let mut history = self.history_system.lock().unwrap();
        let result = history.act();
        match result {
            Ok(state) => {
                namui::event::send(crate::Event::AnimationUpdated(state.clone()));
                Ok(state)
            }
            Err(error) => Err(match error {
                super::ActError::ActionNotExists => ActError::ActionNotExists,
                super::ActError::ActionFailToRun(error) => ActError::ActionFailToRun(error),
            }),
        }
    }

    pub(crate) fn update_action<TAction: Act<Animation>>(
        &self,
        action_ticket: ActionTicket,
        update: impl FnOnce(&mut TAction),
    ) -> Result<(), UpdateActionError> {
        if Some(action_ticket) != *self.working_ticket.lock().unwrap() {
            return Err(UpdateActionError::WrongTicket);
        }

        let mut history = self.history_system.lock().unwrap();
        match history.update_action(update) {
            Ok(_) => Ok(()),
            Err(error) => Err(match error {
                super::UpdateActionError::NoAction => UpdateActionError::NoAction,
                super::UpdateActionError::WrongActionType => UpdateActionError::WrongActionType,
            }),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionTicket {
    id: usize,
}

static NEXT_TICKET_ID: AtomicUsize = AtomicUsize::new(0);
impl ActionTicket {
    pub fn new() -> Self {
        ActionTicket {
            id: NEXT_TICKET_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub enum ActError {
    ActionNotExists,
    ActionFailToRun(Box<dyn Error>),
    WrongTicket,
}

#[derive(Debug)]
pub enum UpdateActionError {
    WrongTicket,
    NoAction,
    WrongActionType,
}
