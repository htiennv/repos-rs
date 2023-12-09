use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Queue<T> {
    items: Mutex<VecDeque<T>>,
    cv: Condvar,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) {
        self.items.lock().unwrap().push_back(item);
        self.cv.notify_one();
    }

    pub fn pop(&self) -> Option<T> {
        let mut s = self
            .cv
            .wait_while(self.items.lock().unwrap(), |s| s.is_empty())
            .unwrap();
        let value = s.pop_front()?;
        Some(value)
    }
}
