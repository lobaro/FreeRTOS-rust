use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::base::*;
use crate::mutex::*;
use crate::queue::*;
use crate::units::Duration;

/// A pub-sub queue. An item sent to the publisher is sent to every subscriber.
pub struct QueuePublisher<T: Sized + Copy> {
    inner: Arc<Mutex<PublisherInner<T>>>,
}

/// A subscribtion to the publisher.
pub struct QueueSubscriber<T: Sized + Copy> {
    inner: Arc<SubscriberInner<T>>,
}

impl<T: Sized + Copy> QueuePublisher<T> {
    /// Create a new publisher
    pub fn new() -> Result<QueuePublisher<T>, FreeRtosError> {
        let inner = PublisherInner {
            subscribers: Vec::new(),
            queue_next_id: 1,
        };

        Ok(QueuePublisher {
            inner: Arc::new(Mutex::new(inner)?),
        })
    }

    /// Send an item to every subscriber. Returns the number of
    /// subscribers that have received the item.
    pub fn send(&self, item: T, max_wait: Duration) -> usize {
        let mut sent_to = 0;

        if let Ok(m) = self.inner.lock(max_wait) {
            for subscriber in &m.subscribers {
                if subscriber.queue.send(item, max_wait).is_ok() {
                    sent_to += 1;
                }
            }
        }

        sent_to
    }

    /// Subscribe to this publisher. Can accept a fixed amount of items.
    pub fn subscribe(
        &self,
        max_size: usize,
        create_max_wait: Duration,
    ) -> Result<QueueSubscriber<T>, FreeRtosError> {
        let mut inner = self.inner.lock(create_max_wait)?;

        let queue = Queue::new(max_size)?;

        let id = inner.queue_next_id;
        inner.queue_next_id += 1;

        let subscriber = SubscriberInner {
            id,
            queue,
            publisher: self.inner.clone(),
        };
        let subscriber = Arc::new(subscriber);

        inner.subscribers.push(subscriber.clone());

        Ok(QueueSubscriber { inner: subscriber })
    }
}

impl<T: Sized + Copy> Clone for QueuePublisher<T> {
    fn clone(&self) -> Self {
        QueuePublisher {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Sized + Copy> Drop for QueueSubscriber<T> {
    fn drop(&mut self) {
        if let Ok(mut l) = self.inner.publisher.lock(Duration::infinite()) {
            l.unsubscribe(&self.inner);
        }
    }
}

impl<T: Sized + Copy> QueueSubscriber<T> {
    /// Wait for an item to be posted from the publisher.
    pub fn receive(&self, max_wait: Duration) -> Result<T, FreeRtosError> {
        self.inner.queue.receive(max_wait)
    }
}

struct PublisherInner<T: Sized + Copy> {
    subscribers: Vec<Arc<SubscriberInner<T>>>,
    queue_next_id: usize,
}

impl<T: Sized + Copy> PublisherInner<T> {
    fn unsubscribe(&mut self, subscriber: &SubscriberInner<T>) {
        self.subscribers.retain(|x| x.id != subscriber.id);
    }
}

struct SubscriberInner<T: Sized + Copy> {
    id: usize,
    queue: Queue<T>,
    publisher: Arc<Mutex<PublisherInner<T>>>,
}
