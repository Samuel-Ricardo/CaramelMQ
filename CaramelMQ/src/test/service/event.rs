use std::sync::{mpsc::Receiver, Arc, Mutex};

use chrono::Utc;

use crate::domain::{
    entity::{event::Event, listener::Listener},
    service::service::EventService,
};

struct MockListener {
    received_events: Arc<Mutex<Vec<String>>>,
}

impl MockListener {
    fn new() -> Self {
        Self {
            received_events: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Listener<String> for MockListener {
    fn on_event(&self, event: &Event<String>) {
        self.received_events
            .lock()
            .unwrap()
            .push(event.payload.clone());
    }
}

#[tokio::test]
async fn publish_event() {
    let now = Utc::now().timestamp_millis();

    let (service, mut receiver) = EventService::<String>::new(10000);

    let listener = MockListener::new();
    //    let listener_ref = Arc::new(listener.received_events.clone());

    service.listen(1, Box::new(listener));

    let event = Event::new(
        1,
        "Hello, Rust with Tokio!".to_string(),
        now,
        "topic".to_string(),
    );

    let expected = event.clone();

    service.publish(event).await;

    if let Some(received_event) = receiver.recv().await {
        assert_eq!(received_event.timestamp, expected.timestamp);
    } else {
        panic!("Nenhum evento recebido");
    }
}
