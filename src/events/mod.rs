//! Event system for reactive programming
//!
//! This module provides a broadcast channel-based event system that allows
//! multiple subscribers to receive events without deadlocks. The key insight
//! is using `tokio::sync::broadcast` which is designed for multi-consumer scenarios.
//!
//! # Why the original code had deadlocks
//!
//! The original implementation likely tried to share state between tasks using
//! `mpsc` channels or mutexes, which can easily lead to deadlocks when:
//! 1. A handler tries to send a message while holding a lock
//! 2. Multiple tasks compete for the same resource in circular dependency
//! 3. Synchronous operations block the async runtime
//!
//! # Solution
//!
//! We use `broadcast::channel` which:
//! - Allows multiple receivers (unlike mpsc)
//! - Doesn't require locks for sending
//! - Never blocks the sender (if buffer is full, old messages are dropped)
//! - Is specifically designed for event broadcasting

use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Maximum number of events to buffer
const EVENT_BUFFER_SIZE: usize = 1024;

/// Event types that can be received from the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
    #[serde(rename = "message")]
    Message { data: Value },
    
    #[serde(rename = "file")]
    File { data: Value },
    
    #[serde(rename = "error")]
    Error { 
        #[serde(flatten)]
        data: ErrorData 
    },
    
    #[serde(rename = "friend")]
    Friend { data: Value },
    
    /// Custom event for any other event type
    #[serde(other)]
    Custom { 
        event: String, 
        data: Value 
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}

/// Event broadcaster that manages event distribution
pub struct EventBroadcaster {
    sender: broadcast::Sender<Event>,
}

impl EventBroadcaster {
    /// Create a new event broadcaster
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(EVENT_BUFFER_SIZE);
        Self { sender }
    }
    
    /// Get a new subscriber/receiver
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
    
    /// Send an event to all subscribers
    pub fn broadcast(&self, event: Event) -> Result<(), broadcast::error::SendError<Event>> {
        tracing::debug!("Broadcasting event: {:?}", event);
        self.sender.send(event).map(|_| ())
    }
    
    /// Get the number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

// Wrapper for convenient stream usage
pub struct EventStream {
    receiver: broadcast::Receiver<Event>,
}

impl EventStream {
    pub fn new(receiver: broadcast::Receiver<Event>) -> Self {
        Self { receiver }
    }
}

impl futures::Stream for EventStream {
    type Item = Event;
    
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        use futures::StreamExt;
        
        // Convert broadcast receiver errors to None (stream ended)
        match std::task::ready!(self.receiver.poll_recv(cx)) {
            Ok(event) => std::task::Poll::Ready(Some(event)),
            Err(broadcast::error::RecvError::Lagged(n)) => {
                tracing::warn!("Event stream lagged by {} messages", n);
                self.poll_next(cx)
            }
            Err(broadcast::error::RecvError::Closed) => {
                std::task::Poll::Ready(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_broadcast() {
        let broadcaster = EventBroadcaster::new();
        let mut sub1 = broadcaster.subscribe();
        let mut sub2 = broadcaster.subscribe();
        
        let event = Event::Message { data: Value::String("test".to_string()) };
        broadcaster.broadcast(event.clone()).unwrap();
        
        assert_eq!(sub1.try_recv(), Ok(event.clone()));
        assert_eq!(sub2.try_recv(), Ok(event));
    }
}
