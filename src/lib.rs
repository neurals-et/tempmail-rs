pub mod client;
pub mod error;
pub mod models;

pub use client::TempMailClient;
pub use error::{Result, TempMailError};
pub use models::{Mailbox, MessageDetail, MessageSummary};
