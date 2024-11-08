use std::sync::Arc;

use chrono::Utc;
use domain::{
    entity::{broker::MessageBroker, event::Event, listener::Listener},
    service::service::EventService,
};

mod application;
mod domain;
mod infrastructure;

mod test;

struct PrintListener;

impl Listener<String> for PrintListener {
    fn on_event(&self, event: &Event<String>) {
        println!("Received event: {:?}", event);
    }
}

#[tokio::main]
async fn main() {
    const BUFFER_SIZE: usize = 100;

    let (event_service, receiver) = EventService::<String>::new(BUFFER_SIZE);
    let event_service = Arc::new(event_service);
    let broker = MessageBroker::new(event_service.clone());

    event_service.listen(1, Box::new(PrintListener));

    broker.start(receiver).await;

    for i in 0..100 {
        let event = Event::new(
            1,
            "Hello, Rust with Tokio! ".to_string() + &i.to_string(),
            Utc::now().timestamp_millis(),
            "topic".to_string(),
        );
        broker.enqueue(event).await;
    }

    tokio::spawn(async {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    })
    .await
    .unwrap();
}
