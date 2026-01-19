//! Tauri IPC command handlers.
//!
//! All commands exposed to the frontend are defined here.

pub mod health;

#[cfg(test)]
mod tests;

pub use health::*;
