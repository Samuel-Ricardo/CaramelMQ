use std::{
    sync::Arc,
    thread::{sleep, spawn},
};

use crate::domain::{
    entity::{broker::MessageBroker, event::Event, queue::Queue},
    service::service::EventService,
};

impl<T: Clone + Send + Sync + 'static> MessageBroker<T> {
    pub fn new(event_service: Arc<EventService<T>>) -> Self {
        Self {
            event_service,
            queue: Queue::new(),
        }
    }

    pub fn start(&self) {
        let queue = self.queue.clone();
        let service = self.event_service.clone();

        spawn(move || loop {
            if let Some(event) = queue.dequeue() {
                let listeners = service.listeners.lock().unwrap();

                if let Some(listener_group) = listeners.get(&event.id) {
                    for listener in listener_group {
                        listener.on_event(&event);
                    }
                }
            }
            sleep(std::time::Duration::from_millis(10)); // Tunable to optimize performance
        });
    }

    pub fn enqueue(&self, event: Event<T>) {
        self.queue.enqueue(event);
    }
}
