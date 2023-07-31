use super::*;
use ::futures::task::{self};
use std::sync::Mutex;
use std::task::Context;
use std::{fmt::Debug, future::Future};
use std::{pin::Pin, sync::OnceLock};

static MINI_TOKIO: OnceLock<MiniTokio> = OnceLock::new();

pub(crate) fn init() -> InitResult {
    MINI_TOKIO
        .set(MiniTokio::new())
        .expect("Failed to initialize mini-tokio");

    Ok(())
}

pub fn spawn_local<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    MINI_TOKIO.get().unwrap().spawn(future)
}

pub(crate) fn execute_async_tasks() {
    MINI_TOKIO.get().unwrap().run_tick();
}

// below code is from https://tokio.rs/tokio/tutorial/async

struct MiniTokio {
    tasks: Mutex<Vec<Task>>,
}
unsafe impl Sync for MiniTokio {}
unsafe impl Send for MiniTokio {}

impl Debug for MiniTokio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniTokio")
            .field("tasks", &self.tasks.lock().unwrap().len())
            .finish()
    }
}

type Task = Pin<Box<dyn Future<Output = ()>>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: Default::default(),
        }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.lock().unwrap().push(Box::pin(future));
    }

    fn run_tick(&self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        let queued = {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.drain(..).collect::<Vec<_>>()
        };

        for mut task in queued {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.lock().unwrap().push(task);
            }
        }
    }
}
