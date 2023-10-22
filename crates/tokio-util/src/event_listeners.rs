use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// A collection of event listeners for a task.
#[derive(Clone, Debug)]
pub struct EventListeners<T> {
    /// All listeners for events
    listeners: Vec<mpsc::UnboundedSender<T>>,
}

impl<T> Default for EventListeners<T> {
    fn default() -> Self {
        Self { listeners: Vec::new() }
    }
}

impl<T: Clone> EventListeners<T> {
    /// Send an event to all listeners.
    ///
    /// Channels that were closed are removed.
    pub fn notify(&mut self, event: T) {
        self.listeners.retain(|listener| listener.send(event.clone()).is_ok())
    }

    /// Add a new event listener.
    pub fn new_listener(&mut self) -> UnboundedReceiverStream<T> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.listeners.push(sender);
        UnboundedReceiverStream::new(receiver)
    }

    /// Push new event listener.
    pub fn push_listener(&mut self, listener: mpsc::UnboundedSender<T>) {
        self.listeners.push(listener);
    }

    /// Removes a listener if it exists.
    pub fn remove_listener(&mut self, listener: &mpsc::UnboundedSender<T>) -> bool {
        let listener_ptr = listener as *const _;
        if let Some(pos) = self.listeners.iter().position(|l| l as *const _ == listener_ptr) {
            self.listeners.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns the number of registered listeners.
    pub fn listener_count(&self) -> usize {
        self.listeners.len()
    }

    /// Returns true if there are no registered listeners.
    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }
}
