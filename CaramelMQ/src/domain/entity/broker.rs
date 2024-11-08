use std::sync::Arc;

use crate::domain::service::service::EventService;

use super::{event::Event, queue::Queue};

struct MessageBroker<T> {
    event_service: Arc<EventService<T>>,
    queue: Queue<Event<T>>,
}
