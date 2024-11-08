use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct Queue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}
