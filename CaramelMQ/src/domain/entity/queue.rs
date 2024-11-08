use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Queue<T> {
    pub(crate) queue: Arc<Mutex<VecDeque<T>>>,
}
