//! Error types for the SteelSeries Sonar API.

use thiserror::Error;

/// Errors that can occur when using the SteelSeries Sonar API.
#[derive(Error, Debug)]
pub enum SonarError {
    #[error("SteelSeries Engine 3 not installed or not in the default location!")]
    EnginePathNotFound,

    #[error("SteelSeries server not accessible! Status code: {0}")]
    ServerNotAccessible(u16),

    #[error("SteelSeries Sonar is not enabled!")]
    SonarNotEnabled,

    #[error("SteelSeries Sonar is not ready yet!")]
    ServerNotReady,

    #[error("SteelSeries Sonar is not running!")]
    ServerNotRunning,

    #[error("Web server address not found")]
    WebServerAddressNotFound,

    #[error("Channel '{0}' not found")]
    ChannelNotFound(String),

    #[error("Slider '{0}' not found")]
    SliderNotFound(String),

    #[error("Invalid volume '{0}'! Value must be between 0.0 and 1.0!")]
    InvalidVolume(f64),

    #[error("Invalid mix volume '{0}'! Value must be between -1.0 and 1.0!")]
    InvalidMixVolume(f64),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for SteelSeries Sonar operations.
pub type Result<T> = std::result::Result<T, SonarError>;
