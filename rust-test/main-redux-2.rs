use std::any::Any;

trait Render {
    fn render(&self) -> String;
}

trait Update {
    fn update(&self, event: &dyn Any) -> Self;
}

fn render(state: &dyn Render) {
    println!("{}", state.render());
}

struct State {
    a: i32,
    b: i32,
    c: i32,
    sub_state: SubState,
}

impl Render for State {
    fn render(&self) -> String {
        format!(
            "{} {} {} - {}",
            self.a,
            self.b,
            self.c,
            self.sub_state.render()
        )
    }
}

impl Update for State {
    fn update(&self, event: &dyn Any) -> Self {
        State {
            a: if let Some(e) = event.downcast_ref::<i32>() {
                self.a + e
            } else {
                self.a
            },
            b: self.b,
            c: self.c,
            sub_state: self.sub_state.update(event),
        }
    }
}

#[derive(Clone)]
struct SubState {
    d: i32,
    e: i32,
    f: i32,
}
impl Render for SubState {
    fn render(&self) -> String {
        format!("{} {} {}", self.d, self.e, self.f)
    }
}

impl Update for SubState {
    fn update(&self, event: &dyn Any) -> Self {
        let mut next_state = (*self).clone();
        if let Some(e) = event.downcast_ref::<i32>() {
            next_state.d += e;
        }
        next_state
    }
}

fn main() {
    let mut state = State {
        a: 1,
        b: 2,
        c: 3,
        sub_state: SubState { d: 4, e: 5, f: 6 },
    };
    render(&state);
    state = state.update(&1);
    render(&state);
}
