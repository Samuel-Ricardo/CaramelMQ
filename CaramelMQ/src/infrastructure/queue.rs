use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct Queue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}
