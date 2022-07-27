use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap, time::Instant};

use catus::symtbl::JsValue;

pub struct TimerQueue {
    heap: BinaryHeap<TimerObject>,
}

impl TimerQueue {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, timeout: Instant, value: JsValue) {
        self.heap.push(TimerObject { timeout, value });
    }

    pub fn peek(&self) -> Option<Instant> {
        self.heap.peek().map(|t| t.timeout)
    }

    pub fn pop(&mut self) -> Option<JsValue> {
        self.heap.pop().map(|t| t.value)
    }
}

struct TimerObject {
    pub timeout: Instant,
    pub value: JsValue,
}

impl PartialEq for TimerObject {
    fn eq(&self, other: &Self) -> bool {
        self.timeout == other.timeout
    }
}

impl Eq for TimerObject {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for TimerObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.timeout.partial_cmp(&self.timeout)
    }
}

impl Ord for TimerObject {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timeout.cmp(&self.timeout)
    }
}

thread_local! {
    pub static TIMER_QUEUE: RefCell<TimerQueue> = RefCell::new(TimerQueue::new());
}
