//! Integration tests for the SteelSeries Sonar API.
//!
//! These tests require a running SteelSeries Engine with Sonar enabled.
//! They will be skipped if the engine is not available.

use steelseries_sonar::{Sonar, SonarError, CHANNEL_NAMES, STREAMER_SLIDER_NAMES};

async fn create_test_client() -> Result<Sonar, SonarError> {
    match Sonar::new().await {
        Ok(sonar) => Ok(sonar),
        Err(e) => {
            println!("Skipping integration tests - SteelSeries Engine not available: {}", e);
            Err(e)
        }
    }
}

#[tokio::test]
async fn test_connection() {
    if let Ok(_sonar) = create_test_client().await {
        // If we get here, connection was successful
        println!("✅ Successfully connected to SteelSeries Sonar");
    }
}

#[tokio::test]
async fn test_volume_data() {
    if let Ok(sonar) = create_test_client().await {
        let volume_data = sonar.get_volume_data().await;
        assert!(volume_data.is_ok(), "Should be able to get volume data");
        println!("Volume data: {:#}", volume_data.unwrap());
    }
}

#[tokio::test]
async fn test_streamer_mode_detection() {
    if let Ok(sonar) = create_test_client().await {
        let is_streamer_mode = sonar.is_streamer_mode().await;
        assert!(is_streamer_mode.is_ok(), "Should be able to detect streamer mode");
        println!("Streamer mode: {}", is_streamer_mode.unwrap());
    }
}

#[tokio::test]
async fn test_chat_mix_data() {
    if let Ok(sonar) = create_test_client().await {
        let chat_mix_data = sonar.get_chat_mix_data().await;
        assert!(chat_mix_data.is_ok(), "Should be able to get chat mix data");
        println!("Chat mix data: {:#}", chat_mix_data.unwrap());
    }
}

#[tokio::test]
async fn test_volume_control() {
    if let Ok(sonar) = create_test_client().await {
        // Test setting volume for master channel
        let result = sonar.set_volume("master", 0.5, None).await;
        if result.is_ok() {
            println!("✅ Successfully set master volume to 50%");
        } else {
            println!("❌ Failed to set volume: {}", result.unwrap_err());
        }
    }
}

#[tokio::test]
async fn test_invalid_volume() {
    if let Ok(sonar) = create_test_client().await {
        // Test invalid volume (too high)
        let result = sonar.set_volume("master", 2.0, None).await;
        assert!(matches!(result, Err(SonarError::InvalidVolume(_))));

        // Test invalid volume (too low)
        let result = sonar.set_volume("master", -1.0, None).await;
        assert!(matches!(result, Err(SonarError::InvalidVolume(_))));
    }
}

#[tokio::test]
async fn test_invalid_channel() {
    if let Ok(sonar) = create_test_client().await {
        let result = sonar.set_volume("invalid_channel", 0.5, None).await;
        assert!(matches!(result, Err(SonarError::ChannelNotFound(_))));
    }
}

#[tokio::test]
async fn test_invalid_chat_mix() {
    if let Ok(sonar) = create_test_client().await {
        // Test invalid chat mix (too high)
        let result = sonar.set_chat_mix(2.0).await;
        assert!(matches!(result, Err(SonarError::InvalidMixVolume(_))));

        // Test invalid chat mix (too low)
        let result = sonar.set_chat_mix(-2.0).await;
        assert!(matches!(result, Err(SonarError::InvalidMixVolume(_))));
    }
}

#[tokio::test]
async fn test_mute_control() {
    if let Ok(sonar) = create_test_client().await {
        // Test muting
        let result = sonar.mute_channel("media", true, None).await;
        if result.is_ok() {
            println!("✅ Successfully muted media channel");
            
            // Test unmuting
            let result = sonar.mute_channel("media", false, None).await;
            if result.is_ok() {
                println!("✅ Successfully unmuted media channel");
            }
        }
    }
}

#[tokio::test]
async fn test_constants() {
    // Test that constants are not empty
    assert!(!CHANNEL_NAMES.is_empty());
    assert!(!STREAMER_SLIDER_NAMES.is_empty());
    
    // Test specific expected channels
    assert!(CHANNEL_NAMES.contains(&"master"));
    assert!(CHANNEL_NAMES.contains(&"game"));
    assert!(CHANNEL_NAMES.contains(&"chatRender"));
    assert!(CHANNEL_NAMES.contains(&"media"));
    assert!(CHANNEL_NAMES.contains(&"aux"));
    assert!(CHANNEL_NAMES.contains(&"chatCapture"));
    
    // Test streamer slider names
    assert!(STREAMER_SLIDER_NAMES.contains(&"streaming"));
    assert!(STREAMER_SLIDER_NAMES.contains(&"monitoring"));
}
