# SteelSeries Sonar Rust API

[![Crates.io](https://img.shields.io/crates/v/steelseries-sonar.svg)](https://crates.io/crates/steelseries-sonar)
[![Documentation](https://docs.rs/steelseries-sonar/badge.svg)](https://docs.rs/steelseries-sonar)
[![Build Status](https://github.com/Mark7888/steelseries-sonar-rs/workflows/CI/badge.svg)](https://github.com/Mark7888/steelseries-sonar-rs/actions)

A Rust library for interacting with the SteelSeries Sonar application API. This crate provides a convenient interface for controlling audio volumes, muting channels, and managing chat mix settings programmatically.

## Features

- 🎵 Control volume levels for different audio channels
- 🔇 Mute/unmute specific channels  
- 🎙️ Manage chat mix settings
- 🎮 Support for both classic and streamer modes
- ⚡ Async/await support with tokio
- 🦀 Safe, idiomatic Rust API
- 📦 Cross-platform support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
steelseries-sonar = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use steelseries_sonar::{Sonar, SonarError};

#[tokio::main]
async fn main() -> Result<(), SonarError> {
    // Create a new Sonar client
    let sonar = Sonar::new().await?;
    
    // Set master volume to 50%
    sonar.set_volume("master", 0.5, None).await?;
    
    // Mute the game channel
    sonar.mute_channel("game", true, None).await?;
    
    // Get current volume data
    let volume_data = sonar.get_volume_data().await?;
    println!("Current volume data: {}", volume_data);
    
    Ok(())
}
```

## Usage

### Initializing the Sonar Client

#### Default Configuration

```rust
use steelseries_sonar::Sonar;

let sonar = Sonar::new().await?;
```

#### Custom Configuration

```rust
use steelseries_sonar::Sonar;
use std::path::Path;

// With custom app data path and explicit streamer mode
let sonar = Sonar::with_config(
    Some(Path::new("C:\\Custom\\Path\\coreProps.json")),
    Some(true), // Enable streamer mode
).await?;
```

### Volume Control

#### Setting Volume

```rust
// Set master volume to 75%
sonar.set_volume("master", 0.75, None).await?;

// In streamer mode, specify the slider
sonar.set_volume("game", 0.8, Some("streaming")).await?;
```

#### Getting Volume Data

```rust
let volume_data = sonar.get_volume_data().await?;
println!("Volume data: {:#}", volume_data);
```

### Channel Muting

```rust
// Mute the media channel
sonar.mute_channel("media", true, None).await?;

// Unmute the media channel
sonar.mute_channel("media", false, None).await?;

// In streamer mode
sonar.mute_channel("chatRender", true, Some("monitoring")).await?;
```

### Chat Mix Control

```rust
// Set chat mix (range: -1.0 to 1.0)
sonar.set_chat_mix(0.3).await?;

// Get current chat mix data
let chat_mix_data = sonar.get_chat_mix_data().await?;
```

### Streamer Mode

```rust
// Check if streamer mode is enabled
let is_streamer_mode = sonar.is_streamer_mode().await?;
println!("Streamer mode: {}", is_streamer_mode);

// Toggle streamer mode
let mut sonar = Sonar::new().await?;
sonar.set_streamer_mode(true).await?;
```

## Available Channels

The following audio channels are supported:

- `"master"` - Master volume
- `"game"` - Game audio
- `"chatRender"` - Chat playback
- `"media"` - Media/music
- `"aux"` - Auxiliary audio
- `"chatCapture"` - Microphone/chat capture

## Streamer Mode

SteelSeries Sonar supports streamer mode, which provides two separate slider controls:

- `"streaming"` - Audio levels for the stream output
- `"monitoring"` - Audio levels for personal monitoring

When streamer mode is enabled, you can specify which slider to control:

```rust
// Control the streaming slider
sonar.set_volume("game", 0.8, Some("streaming")).await?;

// Control the monitoring slider  
sonar.set_volume("game", 0.6, Some("monitoring")).await?;
```

## Error Handling

The library provides comprehensive error handling through the `SonarError` enum:

```rust
use steelseries_sonar::{Sonar, SonarError};

match sonar.set_volume("invalid_channel", 0.5, None).await {
    Ok(_) => println!("Volume set successfully"),
    Err(SonarError::ChannelNotFound(channel)) => {
        eprintln!("Invalid channel: {}", channel);
    }
    Err(SonarError::InvalidVolume(volume)) => {
        eprintln!("Invalid volume: {}", volume);
    }
    Err(err) => eprintln!("Other error: {}", err),
}
```

## Requirements

- SteelSeries Engine 3 must be installed and running
- SteelSeries Sonar must be enabled in the Engine
- Windows, macOS, or Linux (where SteelSeries Engine is available)

## Examples

Check out the `examples/` directory for more comprehensive usage examples:

- `basic_usage.rs` - Basic volume and mute controls
- `streamer_mode.rs` - Working with streamer mode
- `chat_mix.rs` - Chat mix management

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

```bash
# Clone the repository
git clone https://github.com/Mark7888/steelseries-sonar-rs.git
cd steelseries-sonar-rs

# Run tests
cargo test

# Build documentation
cargo doc --open

# Run examples
cargo run --example basic_usage
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [steelseries-sonar-py](https://github.com/Mark7888/steelseries-sonar-py) - Python implementation
- [steelseries-sonar-js](https://github.com/Mark7888/steelseries-sonar-js) - JavaScript/TypeScript implementation

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.
