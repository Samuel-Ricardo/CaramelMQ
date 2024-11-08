use std::sync::{Arc, Mutex};

use chrono::Utc;

use crate::domain::{
    entity::{broker::MessageBroker, event::Event, listener::Listener},
    service::service::EventService,
};

struct E2EListener {
    received_events: Arc<Mutex<Vec<String>>>,
}

impl E2EListener {
    fn new() -> Self {
        Self {
            received_events: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Listener<String> for E2EListener {
    fn on_event(&self, event: &Event<String>) {
        let mut received = self.received_events.lock().unwrap();
        received.push(event.payload.clone());
    }
}

#[tokio::test]
async fn end_to_end_event_flow() {
    let now = Utc::now().timestamp_millis();

    let (service, receiver) = EventService::<String>::new(10);
    let service = Arc::new(service);

    let broker = MessageBroker::new(service.clone());
    let broker_ref = Arc::new(broker.clone());

    let listener = E2EListener::new();
    let listener_ref = Arc::new(listener.received_events.clone());

    service.listen(1, Box::new(listener));

    tokio::spawn(async move {
        broker.start(receiver).await;
    });

    let event = Event::new(1, "E2E Test Event".to_string(), now, "topic".to_string());
    broker_ref.enqueue(event).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let received = listener_ref.lock().unwrap();

    assert_eq!(received.len(), 1);
    assert_eq!(received[0], "E2E Test Event");
}
