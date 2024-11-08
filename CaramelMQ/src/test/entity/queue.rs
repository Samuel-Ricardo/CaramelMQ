#[cfg(test)]
mod test {
    use crate::domain::entity::queue::Queue;

    #[test]
    fn create_a_queue() {
        let queue = Queue::<String>::new();
        assert_eq!(queue.queue.lock().unwrap().len(), 0);
    }
}
