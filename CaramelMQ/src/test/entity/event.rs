#[cfg(test)]
mod test {
    use chrono::Utc;

    use crate::domain::entity::event::Event;

    #[test]
    fn can_craete_a_event() {
        let now = Utc::now().timestamp_millis();
        let event = Event::new(
            1,
            "Hello, Rust with Tokio!".to_string(),
            now,
            "topic".to_string(),
        );

        assert_eq!(event.id, 1);
        assert_eq!(event.payload, "Hello, Rust with Tokio!".to_string());
        assert_eq!(event.timestamp, now);
        assert_eq!(event.topic, "topic".to_string());
    }
}
