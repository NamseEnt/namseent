use super::*;
use namui::animation::Animation;
use std::{
    error::Error,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

#[derive(Clone)]
pub struct AnimationHistory {
    history_system: Arc<Mutex<HistorySystem<Animation>>>,
    working_ticket: Arc<Mutex<Option<Ticket>>>,
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

    pub(crate) fn try_set_action(&self, action: impl Act<Animation>) -> Option<Ticket> {
        let mut history = self.history_system.lock().unwrap();
        if history.has_action() {
            None
        } else {
            history.set_action(action);
            let ticket = Ticket::new();
            *self.working_ticket.lock().unwrap() = Some(ticket);
            Some(ticket)
        }
    }

    pub(crate) fn act(&self, ticket: Ticket) -> Result<Arc<Animation>, ActError> {
        if Some(ticket) != *self.working_ticket.lock().unwrap() {
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
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ticket {
    id: usize,
}

const NEXT_TICKET_ID: AtomicUsize = AtomicUsize::new(0);
impl Ticket {
    pub fn new() -> Self {
        Ticket {
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
