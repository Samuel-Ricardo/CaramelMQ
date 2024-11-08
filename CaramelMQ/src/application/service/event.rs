use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
};

use crate::domain::{
    entity::{event::Event, listener},
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
}
