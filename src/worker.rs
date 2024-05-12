use std::{
    num::NonZeroUsize,
    sync::{mpsc::Sender, Arc, Mutex, RwLock},
};

use crate::{log::VANESSA_LOGGER, sdebug, serror, swarn};

struct WorkerPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Task>>,
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

type Task = Option<Box<dyn FnOnce() + Send + 'static>>;

static GLOBAL_POOL: RwLock<WorkerPool> = RwLock::new(WorkerPool {
    sender: None,
    workers: vec![],
});

/// Initialize the workers subsystem with a specified number of threads.
/// If you just want the maximum number, use `init()` instead.
pub fn init_with(jobs: usize) {
    let mut pool = match GLOBAL_POOL.write() {
        Ok(pool) => pool,
        Err(_) => {
            serror!(
                VANESSA_LOGGER,
                "Failed to lock pool, cannot initialize background workers."
            );
            return;
        }
    };

    let (s, r) = std::sync::mpsc::channel();

    pool.sender = Some(s);

    let r = Arc::new(Mutex::new(r));

    for i in 0..jobs {
        let r = r.clone();

        pool.workers.push(Worker {
            id: i,
            thread: Some(std::thread::spawn(move || {
                sdebug!(VANESSA_LOGGER, "Initializing background worker #{i}");

                loop {
                    let task: Task = r.lock().unwrap().recv().unwrap();
                    match task {
                        Some(task) => {
                            sdebug!(VANESSA_LOGGER, "Background worker #{i} got a task!");
                            task();
                        }
                        None => {
                            sdebug!(
                                VANESSA_LOGGER,
                                "Background worker #{i} received shutdown signal!"
                            );
                            break;
                        }
                    }
                }
            })),
        });
    }
}

/// Initializes the worker subsystem with the default number of threads.
/// If we can detect a core count, it will use all of the available cores.
/// Otherwise it defaults to 1.
pub fn init() {
    let avail = match std::thread::available_parallelism() {
        Ok(v) => v,
        Err(_) => NonZeroUsize::new(1).unwrap(),
    };
    sdebug!(
        VANESSA_LOGGER,
        "Initializing background tasks subsystem with {} threads",
        avail
    );
    init_with(avail.into());
}

/// Submit a background task. It will be executed by a thread on the worker
/// pool as soon as one is available.
pub fn bg<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    let pool = match GLOBAL_POOL.read() {
        Ok(pool) => pool,
        Err(_) => {
            serror!(VANESSA_LOGGER, "Failed to submit a background task!");
            return;
        }
    };
    if pool.sender.is_none() {
        serror!(
            VANESSA_LOGGER,
            "Tried to submit a background task before subsystem initialized!"
        );
        return;
    }
    match pool.sender.as_ref().unwrap().send(Some(Box::new(f))) {
        Ok(_) => {}
        Err(_) => {
            serror!(VANESSA_LOGGER, "Failed to submit a background task!");
        }
    };
}

/// Shuts down the workers subsystem. Don't call this if you intend to use
/// workers at any future point in your program.
pub fn shutdown() {
    let pool = match GLOBAL_POOL.read() {
        Ok(pool) => pool,
        Err(_) => {
            serror!(VANESSA_LOGGER, "Failed to shutdown worker pool!");
            return;
        }
    };
    if pool.sender.is_none() {
        serror!(
            VANESSA_LOGGER,
            "Attempted to shutdown the background worker pool before it has been initialized."
        );
        return;
    }

    for w in &pool.workers {
        // this doesnt guarantee that specific worker will get the signal
        if pool.sender.as_ref().unwrap().send(None).is_err() {
            swarn!(
                VANESSA_LOGGER,
                "Failed to send shutdown signal for worker #{}",
                w.id
            );
            swarn!(VANESSA_LOGGER, "It will not be stopped.");
        }
    }
}

/// Shuts down the workers subsystem. Don't call this if you intend to use
/// workers at any future point in your program.
/// This variant blocks until all of the background workers have concluded
/// their work. Useful if your main thread doesn't do anything while everything
/// is processed on background workers.
pub fn shutdown_blocking() {
    shutdown();

    let mut pool = match GLOBAL_POOL.write() {
        Ok(pool) => pool,
        Err(_) => {
            serror!(VANESSA_LOGGER, "Failed to shutdown worker pool!");
            return;
        }
    };

    for w in &mut pool.workers {
        if w.thread.is_none() {
            continue;
        }
        match w.thread.take().unwrap().join() {
            Ok(_) => {
                sdebug!(VANESSA_LOGGER, "Thread of worker #{} joined.", w.id);
            }
            Err(_) => {
                swarn!(VANESSA_LOGGER, "Failed to join a thread. It might get accidentally killed in a main-thread exit!");
                return;
            }
        };
    }
}
