use event_listener::{Event, EventListener};
use std::sync::atomic::{AtomicUsize, Ordering};

const USIZE_TOP_BIT_MASK: usize = 0x1000000000000000;

#[derive(Debug)]
pub struct AwaitableAtomicCounterAndBit {
    incr_event: Event,
    decr_event: Event,
    value: AtomicUsize,
}

impl AwaitableAtomicCounterAndBit {
    pub fn new(value: usize) -> Self {
        if value & USIZE_TOP_BIT_MASK > 0 {
            panic!("Initial value cannot be larger than 2**63");
        }
        Self {
            incr_event: Event::new(),
            decr_event: Event::new(),
            value: AtomicUsize::new(value),
        }
    }

    pub fn set_bit(&self) -> bool {
        let prior = self.value.fetch_or(USIZE_TOP_BIT_MASK, Ordering::SeqCst);
        self.incr_event.notify(usize::MAX);
        self.decr_event.notify(usize::MAX);
        prior & USIZE_TOP_BIT_MASK > 0
    }

    pub fn incr(&self) -> (bool, usize) {
        let prior = self.value.fetch_add(1, Ordering::SeqCst);
        if prior & !USIZE_TOP_BIT_MASK >= (1 << 63) - 1 {
            panic!("Cannot increase size past 2**63-1");
        }
        self.incr_event.notify(usize::MAX);
        (prior & USIZE_TOP_BIT_MASK > 0, prior & !USIZE_TOP_BIT_MASK)
    }

    pub fn decr(&self) -> (bool, usize) {
        let prior = self.value.fetch_sub(1, Ordering::SeqCst);
        self.decr_event.notify(usize::MAX);
        (prior & USIZE_TOP_BIT_MASK > 0, prior & !USIZE_TOP_BIT_MASK)
    }

    pub fn load(&self) -> (bool, usize) {
        let value = self.value.load(Ordering::SeqCst);
        (value & USIZE_TOP_BIT_MASK > 0, value & !USIZE_TOP_BIT_MASK)
    }

    pub fn listen_incr(&self) -> EventListener {
        self.incr_event.listen()
    }
    pub fn listen_decr(&self) -> EventListener {
        self.decr_event.listen()
    }
}