use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::{park_timeout, JoinHandle};
use std::time::Duration;

pub struct Builder {
    /// Set the name of the thread running this task.
    /// Equivalent to `std::thread` call.
    name: Option<String>,

    /// Set the stack size for the thread running this task, in bytes.
    /// Equivalent to the `std::thread` call.
    stack_size: Option<usize>,
}

pub struct TaskJoinHandle {
    thread_handle: JoinHandle<()>,
    cancel_token: Arc<AtomicBool>,
}

impl Builder {
    pub fn name(mut self, name: String) -> Builder {
        self.name = Some(name);
        self
    }

    pub fn stack_size(mut self, size: usize) -> Builder {
        self.stack_size = Some(size);
        self
    }

    /// Repeatedly execute `f`, with a period of `p` until canceled.
    pub fn spawn<F>(self, p: Duration, f: F) -> Result<TaskJoinHandle>
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        let cancel_token = Arc::new(AtomicBool::new(false));
        let is_cancelled = cancel_token.clone();
        let thread_handle = std::thread::spawn(move || {
            while !is_cancelled.load(std::sync::atomic::Ordering::Relaxed) {
                f();
                park_timeout(p);
            }
        });

        Ok(TaskJoinHandle {
            thread_handle,
            cancel_token,
        })
    }
}

impl TaskJoinHandle {
    pub fn cancel(self) -> std::thread::Result<()> {
        self.cancel_token
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.thread_handle.thread().unpark();
        self.thread_handle.join()
    }
}
