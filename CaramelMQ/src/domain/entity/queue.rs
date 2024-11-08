use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct Queue<T> {
    pub(crate) queue: Arc<Mutex<VecDeque<T>>>,
}
