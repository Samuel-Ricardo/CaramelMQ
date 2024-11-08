use std::{
    sync::Arc,
    thread::{sleep, spawn},
};

use crate::domain::{
    entity::{
        broker::MessageBroker,
        event::Event,
        listener,
        queue::{self, Queue},
    },
    service::service::EventService,
};

use tokio::sync::mpsc::Receiver;

impl<T: Clone + Send + Sync + 'static + std::fmt::Debug> MessageBroker<T> {
    pub fn new(event_service: Arc<EventService<T>>) -> Self {
        Self {
            event_service,
            queue: Queue::new(),
        }
    }

    pub async fn start(&self, mut receiver: Receiver<Event<T>>) {
        let queue = self.queue.clone();
        let service = self.event_service.clone();

        tokio::spawn(async move {
            print!("Starting message broker...");
            loop {
                if let Some(event) = receiver.recv().await {
                    let listeners = service.listeners.lock().expect("Failed to lock listeners");

                    if let Some(listener_group) = listeners.get(&event.id) {
                        for listener in listener_group {
                            listener.on_event(&event);
                        }
                    }
                } else {
                    if let Some(event) = queue.dequeue() {
                        service.publish(event).await;
                    }
                }

                if 1 < 0 {
                    break;
                }
            }
            println!("Message broker stopped");
        });
    }

    pub async fn enqueue(&self, event: Event<T>) {
        let event2 = event.clone();
        self.queue.enqueue(event);
        self.event_service.publish(event2).await;
    }
}
