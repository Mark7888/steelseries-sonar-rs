//! Basic usage example for the SteelSeries Sonar API.
//!
//! This example demonstrates:
//! - Creating a Sonar client
//! - Getting volume data
//! - Setting volume levels
//! - Muting/unmuting channels

use steelseries_sonar::{Sonar, SonarError, CHANNEL_NAMES};

#[tokio::main]
async fn main() -> Result<(), SonarError> {
    println!("SteelSeries Sonar Basic Usage Example");
    println!("=====================================");

    // Create a new Sonar client
    println!("🔌 Connecting to SteelSeries Sonar...");
    let sonar = match Sonar::new().await {
        Ok(sonar) => {
            println!("✅ Successfully connected to SteelSeries Sonar!");
            sonar
        }
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            eprintln!("Make sure SteelSeries Engine is running and Sonar is enabled.");
            return Err(e);
        }
    };

    // Check streamer mode status
    let is_streamer_mode = sonar.is_streamer_mode().await?;
    println!("🎮 Streamer mode: {}", if is_streamer_mode { "Enabled" } else { "Disabled" });

    // Get current volume data
    println!("\n📊 Getting current volume data...");
    let volume_data = sonar.get_volume_data().await?;
    println!("Current volume data: {:#}", volume_data);

    // Demonstrate volume control
    println!("\n🔊 Volume Control Demo:");
    
    // Set master volume to 50%
    println!("Setting master volume to 50%...");
    sonar.set_volume("master", 0.5, None).await?;
    println!("✅ Master volume set to 50%");

    // Set game volume to 75%
    println!("Setting game volume to 75%...");
    sonar.set_volume("game", 0.75, None).await?;
    println!("✅ Game volume set to 75%");

    // Demonstrate muting
    println!("\n🔇 Mute Control Demo:");
    
    // Mute the media channel
    println!("Muting media channel...");
    sonar.mute_channel("media", true, None).await?;
    println!("✅ Media channel muted");

    // Wait a moment
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Unmute the media channel
    println!("Unmuting media channel...");
    sonar.mute_channel("media", false, None).await?;
    println!("✅ Media channel unmuted");

    // Show all available channels
    println!("\n📋 Available channels:");
    for channel in CHANNEL_NAMES {
        println!("  • {}", channel);
    }

    // Get chat mix data
    println!("\n🎙️ Chat Mix Demo:");
    let chat_mix_data = sonar.get_chat_mix_data().await?;
    println!("Current chat mix: {:#}", chat_mix_data);

    // Set chat mix to slightly favor game audio
    println!("Setting chat mix to favor game audio...");
    sonar.set_chat_mix(0.2).await?;
    println!("✅ Chat mix updated");

    println!("\n🎉 Demo completed successfully!");
    println!("You can now use the SteelSeries Sonar API in your own applications.");

    Ok(())
}
