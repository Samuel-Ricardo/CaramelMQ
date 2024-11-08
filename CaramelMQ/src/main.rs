use std::sync::{mpsc::Receiver, Arc};

use chrono::Utc;
use domain::{
    entity::{broker::MessageBroker, event::Event, listener::Listener},
    service::service::EventService,
};

mod application;
mod domain;
mod infrastructure;

struct PrintListener;

impl Listener<String> for PrintListener {
    fn on_event(&self, event: &Event<String>) {
        println!("Received event with ID {}: {}", event.id, event.payload);
    }
}

fn main() {
    let (event_service, receiver) = EventService::<String>::new();
    let event_service = Arc::new(event_service);
    let broker = MessageBroker::new(event_service.clone());

    event_service.listen(1, Box::new(PrintListener));

    broker.start();

    for i in 0..100 {
        let event = Event::<String>::new(
            1,
            "Hello, World! ".to_string() + &i.to_string(),
            Utc::now().timestamp_millis(),
            "topic".to_string(),
        );

        broker.enqueue(event.clone());
    }

    for received_event in receiver.iter() {
        println!(
            "Processing event with ID {}: {}",
            received_event.id, received_event.payload
        );
    }
}
