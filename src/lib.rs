//! KoalaNetty - A modern Rust client for the Koala network protocol
//!
//! # Architecture
//!
//! This crate uses a clean architecture with the following modules:
//!
//! - `core` - Core types, errors, and utilities
//! - `packet` - Packet encoding/decoding logic
//! - `net::model` - Data models for requests/responses
//! - `net::client` - Client implementation with API
//! - `net::handlers` - Packet handlers
//! - `events` - Event system for reactive programming
//!
//! # Example
//!
//! ```no_run
//! use koala_netty::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut client = Client::connect("ag6.ru:8000", "your_token").await?;
//!     client.start().await?;
//!     
//!     // Subscribe to events before starting
//!     let mut event_stream = client.subscribe_events();
//!     
//!     tokio::spawn(async move {
//!         while let Some(event) = event_stream.next().await {
//!             println!("Received event: {:?}", event);
//!         }
//!     });
//!     
//!     client.api().nick_change("New Nick").await?;
//!     client.send_message(12729, "Привет").await?;
//!     
//!     client.run().await?;
//!     Ok(())
//! }
//! ```

pub mod core;
pub mod events;
pub mod net;
pub mod packet;

pub mod prelude {
    pub use crate::core::error::{Error, Result};
    pub use crate::net::client::Client;
    pub use crate::events::Event;
}
