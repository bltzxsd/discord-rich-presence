//! This library provides easy access to the Discord IPC.
//!
//! It provides implementations for both Unix and Windows
//! operating systems, with both implementations using the
//! same API. Thus, this crate can be used in a platform-agnostic
//! manner.
//!
//! # Hello world
//! ```text
//! use discord_rich_presence::{activity, new_client, DiscordIpc};
//! use std::error::Error;
//! fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//!     let mut client = new_client("<some client id>")?;
//!     client.connect()?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//!
//!     Ok(())
//!  }
//! ```
#![deny(missing_docs)]

mod discord_ipc;

mod pack_unpack;
pub use discord_ipc::*;
pub mod activity;
mod error;

#[cfg(unix)]
mod ipc_unix;
#[cfg(unix)]
use ipc_unix as ipc;

#[cfg(windows)]
mod ipc_windows;
#[cfg(windows)]
use ipc_windows as ipc;

/// Creates a new client to connect to the Discord IPC.
///
/// # Examples
/// ```
/// # use discord_rich_presence::new_client;
/// let ipc_client = new_client("<some client id>")?;
/// # Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
/// ```
pub fn new_client(client_id: &str) -> Result<impl DiscordIpc, crate::error::Error> {
    let client = ipc::DiscordIpcClient {
        client_id: client_id.to_string(),
        connected: false,
        socket: None,
    };

    Ok(client)
}
