#![feature(associated_type_defaults)]

mod error;
mod profile;
pub mod tui;

#[cfg(feature = "discord")]
pub use error::CommandError;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// How often to poll for new events in ms.
pub const POLL_TIMEOUT: u64 = 16;

/// How many views to keep in history.
pub const HISTORY_SIZE: usize = 3;
