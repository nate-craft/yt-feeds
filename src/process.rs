use std::thread::{self, JoinHandle};

pub struct Process<R> {
    action: JoinHandle<anyhow::Result<R>>,
}

impl<R> Process<R> {
    pub fn background<F>(action: F) -> Self
    where
        F: 'static + Send + Sync + FnOnce() -> anyhow::Result<R>,
        R: 'static + Send + Sync,
    {
        Self {
            action: thread::spawn(|| action()),
        }
    }

    pub fn join(self) -> anyhow::Result<R> {
        match self.action.join() {
            Ok(result) => result,
            Err(err) => Err(anyhow::Error::msg("Could not join process")),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.action.is_finished()
    }
}
