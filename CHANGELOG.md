# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of the SteelSeries Sonar Rust API
- Complete async API for SteelSeries Sonar control
- Support for both classic and streamer modes
- Volume control for all audio channels
- Mute/unmute functionality
- Chat mix control
- Comprehensive error handling
- Full documentation and examples
- GitHub Actions CI/CD pipeline

### Features
- Control volume levels for different audio channels (master, game, chatRender, media, aux, chatCapture)
- Mute/unmute specific channels
- Manage chat mix settings (-1.0 to 1.0 range)
- Support for streamer mode with dual sliders (streaming/monitoring)
- Async/await support with tokio
- Cross-platform support (Windows, macOS, Linux)
- Safe, idiomatic Rust API

## [0.1.0] - 2024-XX-XX

### Added
- Initial public release
- Core SteelSeries Sonar API functionality
- Basic volume and mute controls
- Chat mix management
- Streamer mode support
- Comprehensive documentation
- Example applications
- CI/CD pipeline
