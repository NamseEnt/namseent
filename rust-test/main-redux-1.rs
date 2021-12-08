use std::any::Any;

trait Render {
    fn render(&self) -> String;
}

trait Update {
    fn update(&mut self, event: &dyn Any);
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
    fn update(&mut self, event: &dyn Any) {
        if let Some(e) = event.downcast_ref::<i32>() {
            self.a += e;
        }
        self.sub_state.update(event);
    }
}

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
    fn update(&mut self, event: &dyn Any) {
        if let Some(e) = event.downcast_ref::<i32>() {
            self.d += e;
        }
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
    state.update(&1);
    render(&state);
}
