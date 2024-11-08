#[cfg(test)]
mod test {
    use crate::domain::entity::queue::Queue;

    #[test]
    fn create_a_queue() {
        let queue = Queue::<String>::new();
        assert_eq!(queue.queue.lock().unwrap().len(), 0);
    }

    #[test]
    fn can_enqueue() {
        let queue = Queue::<String>::new();
        queue.enqueue("Hello, Rust with Tokio!".to_string());

        assert_eq!(queue.queue.lock().unwrap().len(), 1);
    }

    #[test]
    fn can_dequeue() {
        let queue = Queue::<String>::new();
        queue.enqueue("Hello, Rust with Tokio!".to_string());

        assert_eq!(queue.dequeue(), Some("Hello, Rust with Tokio!".to_string()));
        assert_eq!(queue.queue.lock().unwrap().len(), 0);
    }
}
