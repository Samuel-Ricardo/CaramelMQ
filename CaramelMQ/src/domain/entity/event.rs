#[derive(Clone, Debug)]
pub struct Event<T> {
    pub id: u64,
    pub payload: T,
    pub timestamp: i64,
    pub topic: String,
}

impl<T: Clone> Event<T> {
    pub fn new(id: u64, payload: T, timestamp: i64, topic: String) -> Self {
        Event {
            id,
            payload,
            timestamp,
            topic,
        }
    }
}
