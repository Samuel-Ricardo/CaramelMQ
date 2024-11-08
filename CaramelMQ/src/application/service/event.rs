use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
};

use crate::domain::{
    entity::{
        event::Event,
        listener::{self, Listener},
    },
    service::service::EventService,
};

impl<T: Send + 'static> EventService<T> {
    pub fn new() -> (Self, Receiver<Event<T>>) {
        let (sender, receiver) = mpsc::channel::<Event<T>>();
        let listener = Arc::new(Mutex::new(HashMap::<
            u64,
            Vec<Box<dyn listener::Listener<T> + Send>>,
        >::new()));

        (
            Self {
                listeners: listener,
                senders: sender,
            },
            receiver,
        )
    }

    pub fn publish(&self, event: Event<T>) {
        self.senders
            .send(event)
            .expect("Failed to publish event ${event}");
    }

    pub fn listen(&self, event_id: u64, listener: Box<dyn Listener<T> + Send>) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(event_id).or_insert(vec![]).push(listener);
    }
}
