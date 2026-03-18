//! Claw Extensions - Advanced features for claw-core
//!
//! This crate provides optional extensions for the claw-core cellular agent engine.
//!
//! ## Features
//!
//! - **equipment**: Advanced multi-slot equipment system
//! - **social**: Multi-agent coordination patterns
//! - **learning**: Seed learning and model distillation
//! - **bot**: Simple automation loops without ML
//! - **websocket**: Real-time bidirectional communication
//! - **gpu**: GPU acceleration (CUDA/WGPU)
//! - **monitoring**: Advanced monitoring and telemetry
//!
//! ## Usage
//!
//! ```rust,no_run
//! use claw_extensions::equipment::EquipmentManager;
//! use claw_extensions::social::SocialManager;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut equipment_mgr = EquipmentManager::new();
//!     let social_mgr = SocialManager::new();
//!
//!     // Use extensions
//!     Ok(())
//! }
//! ```

pub mod error;

#[cfg(feature = "equipment")]
pub mod equipment;

#[cfg(feature = "social")]
pub mod social;

#[cfg(feature = "learning")]
pub mod learning;

#[cfg(feature = "bot")]
pub mod bot;

#[cfg(feature = "websocket")]
pub mod websocket;

#[cfg(feature = "gpu")]
pub mod gpu;

#[cfg(feature = "monitoring")]
pub mod monitoring;

// Re-export common types
pub use error::{ExtensionError, Result};

/// Extension version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Extension name
pub const NAME: &str = env!("CARGO_PKG_NAME");
