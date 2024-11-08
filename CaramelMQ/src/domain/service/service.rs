use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::domain::entity::{event::Event, listener::Listener};

pub struct EventService<T> {
    pub listeners: Arc<Mutex<HashMap<u64, Vec<Box<dyn Listener<T> + Send>>>>>,
    pub sender: Sender<Event<T>>,
}
