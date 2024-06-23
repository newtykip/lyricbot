mod error;
pub mod tui;

pub type Result<T> = std::result::Result<T, error::Error>;

/// How often to poll for new events in ms.
pub const POLL_TIMEOUT: u64 = 16;

/// How many views to keep in history.
pub const HISTORY_LIMIT: usize = 3;
