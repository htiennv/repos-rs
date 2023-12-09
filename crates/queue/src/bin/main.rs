use std::sync::Arc;

use queue::Queue;

fn main() {
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
        let item = q.pop();
        println!("Popped item: {:?}", item);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
