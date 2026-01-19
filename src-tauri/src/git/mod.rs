//! Git operations module.
//!
//! Provides git status checking and repository operations.

mod status;
pub mod history;

#[cfg(test)]
mod tests;

pub use status::*;
pub use history::*;
