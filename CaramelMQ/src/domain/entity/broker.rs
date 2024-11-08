use std::sync::Arc;

use crate::domain::service::service::EventService;

use super::{event::Event, queue::Queue};

pub struct MessageBroker<T> {
    pub(crate) event_service: Arc<EventService<T>>,
    pub(crate) queue: Queue<Event<T>>,
}
