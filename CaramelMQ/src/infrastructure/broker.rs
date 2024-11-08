use std::sync::Arc;

use crate::domain::{
    entity::{broker::MessageBroker, queue::Queue},
    service::service::EventService,
};

impl<T: Send + Sync + 'static> MessageBroker<T> {
    pub fn new(event_service: Arc<EventService<T>>) -> Self {
        Self {
            event_service,
            queue: Queue::new(),
        }
    }
}
