use futures::sync::mpsc;

type BroadcastSender<T> = mpsc::UnboundedSender<T>;
pub type BroadcastListener<T> = mpsc::UnboundedReceiver<T>;

pub struct Broadcast<T: Clone> {
    connections: Vec<BroadcastSender<T>>
}

impl<T: Clone> Broadcast<T> {
    pub fn new() -> Broadcast<T> {
        Broadcast { connections: vec![] }
    }

    pub fn create_listener(&mut self) -> BroadcastListener<T> {
        let (sender, listener) = mpsc::unbounded();
        self.connections.push(sender);
        listener
    }

    pub fn send(&self, value: T) {
        let size = self.connections.len();
        if size > 0 {
            for i in 0..size - 1 {
                self.connections[i].unbounded_send(value.clone());
            }
            self.connections[size - 1].unbounded_send(value);
        }
    }
}