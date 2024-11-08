use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::domain::{
    entity::{
        event::Event,
        listener::{self, Listener},
    },
    service::service::EventService,
};

impl<T: Clone + Send + 'static> EventService<T> {
    pub fn new(buffer_size: usize) -> (Self, Receiver<Event<T>>) {
        let (sender, receiver) = mpsc::channel(buffer_size);
        let listener = Arc::new(Mutex::new(HashMap::<
            u64,
            Vec<Box<dyn listener::Listener<T> + Send>>,
        >::new()));

        (
            Self {
                listeners: listener,
                sender,
            },
            receiver,
        )
    }

    pub async fn publish(&self, event: Event<T>) {
        self.sender
            .send(event)
            .await
            .expect("Failed to publish event ${event}");
    }

    pub fn listen(&self, event_id: u64, listener: Box<dyn Listener<T> + Send>) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(event_id).or_insert(vec![]).push(listener);
    }
}
