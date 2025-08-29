//! # SteelSeries Sonar API
//!
//! A Rust library for interacting with the SteelSeries Sonar application API.
//! This library allows you to control audio volumes, mute channels, and manage
//! chat mix settings programmatically.
//!
//! ## Features
//!
//! - Control volume levels for different audio channels
//! - Mute/unmute specific channels
//! - Manage chat mix settings
//! - Support for both classic and streamer modes
//! - Async/await support with tokio
//!
//! ## Quick Start
//!
//! ```no_run
//! use steelseries_sonar::Sonar;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new Sonar client
//!     let sonar = Sonar::new().await?;
//!     
//!     // Set master volume to 50%
//!     sonar.set_volume("master", 0.5, None).await?;
//!     
//!     // Mute the game channel
//!     sonar.mute_channel("game", true, None).await?;
//!     
//!     // Get current volume data
//!     let volume_data = sonar.get_volume_data().await?;
//!     println!("Current volume data: {}", volume_data);
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod sonar;
pub mod blocking;

pub use error::{Result, SonarError};
pub use sonar::{Sonar, CHANNEL_NAMES, STREAMER_SLIDER_NAMES};
pub use blocking::BlockingSonar;
