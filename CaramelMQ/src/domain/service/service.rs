use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::domain::entity::{event::Event, listener::Listener};

pub struct EventService<T> {
    pub listeners: Arc<Mutex<HashMap<u64, Vec<Box<dyn Listener<T> + Send>>>>>,
    pub senders: Sender<Event<T>>,
}
