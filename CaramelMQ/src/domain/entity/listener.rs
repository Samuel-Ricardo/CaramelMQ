use super::event::Event;

pub trait Listener<T> {
    fn on_event(&self, event: &Event<T>);
}
