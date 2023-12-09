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

    pub fn len(&self) -> usize {
        self.items.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::Queue;

    #[test]
    fn test_push_pop() {
        let queue = Arc::new(Queue::new());

        let q = queue.clone();
        // Create threads to push and pop elements from the queue
        let thread1 = std::thread::spawn(move || {
            q.push(1);
            q.push(2);
            q.push(3);
        });
        let q = queue.clone();
        let thread2 = std::thread::spawn(move || loop {
            if q.len() == 0 {
                break;
            }
            let item = q.pop();
            println!("Popped item: {:?}", item);
        });
    
        thread1.join().unwrap();
        thread2.join().unwrap();
    }
}
