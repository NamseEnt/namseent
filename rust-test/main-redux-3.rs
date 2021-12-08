use once_cell::sync::OnceCell;
use std::{
    any::Any,
    collections::VecDeque,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    time::Instant,
};
type Event = Box<dyn Any + Send + Sync>;
static EVENTS: OnceCell<Mutex<VecDeque<Box<dyn Any + Send + Sync>>>> = OnceCell::new();

fn push_event(event: Event) {
    let mut events = EVENTS.get().unwrap().lock().unwrap();
    events.push_back(event);
}

fn pull_event() -> Option<Event> {
    let mut events = EVENTS.get().unwrap().lock().unwrap();
    events.pop_front()
}

trait Render {
    fn render(&self) -> String;
}

trait Update {
    fn update(&self, event: &dyn Any) -> Self;
}

fn render(state: &dyn Render) {
    // println!("{}", state.render());
}

#[derive(Clone)]
struct State {
    sub_states: Vec<SubState>,
}

enum StateAction {
    Add(SubState),
    Remove(u32),
}

impl Render for State {
    fn render(&self) -> String {
        let mut result = String::new();
        println!("sub_states.len: {}", self.sub_states.len());
        for sub_state in &self.sub_states {
            result.push_str(&sub_state.render());
            result.push_str("\n");
        }
        result
    }
}

impl Update for State {
    fn update(&self, event: &dyn Any) -> Self {
        let mut new_state = (*self).clone();
        match event.downcast_ref::<StateAction>() {
            Some(StateAction::Add(sub_state)) => {
                println!("add sub_state");
                new_state.sub_states.push(sub_state.clone());
            }
            Some(StateAction::Remove(id)) => {
                new_state.sub_states.retain(|sub_state| sub_state.id != *id);
            }
            None => {}
        };
        State {
            sub_states: new_state
                .sub_states
                .iter()
                .map(|sub_state| sub_state.update(event))
                .collect(),
            ..new_state
        }
    }
}

#[derive(Clone)]
struct SubState {
    id: u32,
    d: i32,
    e: i32,
    f: i32,
}
impl Render for SubState {
    fn render(&self) -> String {
        format!("id: {} - {} {} {}", self.id, self.d, self.e, self.f)
    }
}

impl Update for SubState {
    fn update(&self, event: &dyn Any) -> Self {
        let mut next_state = (*self).clone();
        if let Some(e) = event.downcast_ref::<i32>() {
            next_state.d += e;
        }
        if next_state.d >= 10 {
            push_event(Box::new(StateAction::Remove(next_state.id)));
        }
        next_state
    }
}

fn main() {
    EVENTS.set(Mutex::new(VecDeque::new())).unwrap();

    let mut state = State { sub_states: vec![] };

    render(&state);

    push_event(Box::new(1));
    let event = pull_event().unwrap();
    state = state.update(event.as_ref());
    render(&state);

    push_event(Box::new(StateAction::Add(SubState {
        id: 1,
        d: 1,
        e: 1,
        f: 1,
    })));
    let event = pull_event().unwrap();
    state = state.update(event.as_ref());
    render(&state);

    push_event(Box::new(StateAction::Add(SubState {
        id: 2,
        d: 0,
        e: 0,
        f: 0,
    })));
    let event = pull_event().unwrap();
    state = state.update(event.as_ref());
    render(&state);

    let now = Instant::now();
    for i in 0..1000 {
        push_event(Box::new(1));
        match pull_event() {
            Some(event) => {
                state = state.update(event.as_ref());
                render(&state);
            }
            None => {}
        }
        if state.sub_states.len() == 0 {
            println!("push");
            push_event(Box::new(StateAction::Add(SubState {
                id: i,
                d: 0,
                e: 0,
                f: 0,
            })));
        }
    }

    while let Some(event) = pull_event() {
        state = state.update(event.as_ref());
        render(&state);
    }

    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} ms.",
        elapsed_time.as_millis()
    );

    let event = pull_event().unwrap();
    state = state.update(event.as_ref());
    render(&state);
}
