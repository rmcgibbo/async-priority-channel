use event_listener::{Event, EventListener};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct AwaitableAtomicBool {
    event: Event,
    value: AtomicBool,
}
impl AwaitableAtomicBool {
    pub fn new(value: bool) -> Self {
        AwaitableAtomicBool {
            event: Event::new(),
            value: AtomicBool::new(value),
        }
    }
    pub fn fetch_or(&self, value: bool) -> bool {
        let prior = self.value.fetch_or(value, Ordering::SeqCst);
        self.event.notify(usize::MAX);
        prior
    }

    pub fn load(&self) -> bool {
        self.value.load(Ordering::SeqCst)
    }

    pub fn listen(&self) -> EventListener {
        self.event.listen()
    }
}

#[derive(Debug)]
pub struct AwaitableAtomicCounter {
    incr_event: Event,
    decr_event: Event,
    value: AtomicUsize,
}
impl AwaitableAtomicCounter {
    pub fn new(value: usize) -> Self {
        Self {
            incr_event: Event::new(),
            decr_event: Event::new(),
            value: AtomicUsize::new(value),
        }
    }

    pub fn incr(&self) -> usize {
        let prior = self.value.fetch_add(1, Ordering::SeqCst);
        self.incr_event.notify(usize::MAX);
        prior
    }

    pub fn decr(&self) -> usize {
        let prior = self.value.fetch_sub(1, Ordering::SeqCst);
        self.decr_event.notify(usize::MAX);
        prior
    }

    pub fn load(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn listen_incr(&self) -> EventListener {
        self.incr_event.listen()
    }
    pub fn listen_decr(&self) -> EventListener {
        self.decr_event.listen()
    }
}
