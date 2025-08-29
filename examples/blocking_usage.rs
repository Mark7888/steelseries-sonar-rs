//! Blocking (synchronous) usage example for the SteelSeries Sonar API.
//!
//! This example demonstrates the blocking API which doesn't require async/await.

use steelseries_sonar::{BlockingSonar, SonarError, CHANNEL_NAMES};

fn main() -> Result<(), SonarError> {
    println!("SteelSeries Sonar Blocking API Example");
    println!("======================================");

    // Create a new blocking Sonar client
    println!("🔌 Connecting to SteelSeries Sonar (blocking)...");
    let mut sonar = match BlockingSonar::new() {
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
    let is_streamer_mode = sonar.is_streamer_mode()?;
    println!("🎮 Streamer mode: {}", if is_streamer_mode { "Enabled" } else { "Disabled" });

    // Get current volume data
    println!("\n📊 Getting current volume data...");
    let volume_data = sonar.get_volume_data()?;
    println!("Current volume data: {:#}", volume_data);

    // Demonstrate volume control
    println!("\n🔊 Volume Control Demo:");
    
    // Set master volume to 60%
    println!("Setting master volume to 60%...");
    sonar.set_volume("master", 0.6, None)?;
    println!("✅ Master volume set to 60%");

    // Set game volume to 80%
    println!("Setting game volume to 80%...");
    sonar.set_volume("game", 0.8, None)?;
    println!("✅ Game volume set to 80%");

    // Demonstrate muting
    println!("\n🔇 Mute Control Demo:");
    
    // Mute the aux channel
    println!("Muting aux channel...");
    sonar.mute_channel("aux", true, None)?;
    println!("✅ Aux channel muted");

    // Wait a moment (using std::thread::sleep since we're in blocking mode)
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Unmute the aux channel
    println!("Unmuting aux channel...");
    sonar.mute_channel("aux", false, None)?;
    println!("✅ Aux channel unmuted");

    // Show all available channels
    println!("\n📋 Available channels:");
    for channel in CHANNEL_NAMES {
        println!("  • {}", channel);
    }

    // Chat mix demonstration
    println!("\n🎙️ Chat Mix Demo:");
    let chat_mix_data = sonar.get_chat_mix_data()?;
    println!("Current chat mix: {:#}", chat_mix_data);

    // Set chat mix to favor chat slightly
    println!("Setting chat mix to favor chat audio...");
    sonar.set_chat_mix(0.3)?;
    println!("✅ Chat mix updated");

    // Streamer mode toggle demonstration (if currently in classic mode)
    if !is_streamer_mode {
        println!("\n🎮 Streamer Mode Toggle Demo:");
        
        println!("Enabling streamer mode...");
        sonar.set_streamer_mode(true)?;
        println!("✅ Streamer mode enabled");

        // Show how to use streamer sliders
        println!("Setting game volume for streaming slider...");
        sonar.set_volume("game", 0.7, Some("streaming"))?;
        
        println!("Setting game volume for monitoring slider...");
        sonar.set_volume("game", 0.5, Some("monitoring"))?;
        
        println!("✅ Different volumes set for streaming and monitoring");

        // Switch back to classic mode
        println!("Switching back to classic mode...");
        sonar.set_streamer_mode(false)?;
        println!("✅ Back to classic mode");
    }

    // Error handling demonstration
    println!("\n⚠️ Error Handling Demo:");
    
    // Try to set an invalid volume
    match sonar.set_volume("master", 1.5, None) {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(SonarError::InvalidVolume(volume)) => {
            println!("   ✅ Correctly caught invalid volume: {}", volume);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Try to use an invalid channel
    match sonar.set_volume("invalid_channel", 0.5, None) {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(SonarError::ChannelNotFound(channel)) => {
            println!("   ✅ Correctly caught invalid channel: {}", channel);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    println!("\n🎉 Blocking API demo completed successfully!");
    println!("The blocking API is perfect for:");
    println!("  • Simple scripts and utilities");
    println!("  • Applications that don't use async/await");
    println!("  • Quick prototyping and testing");
    println!("  • Integration with existing synchronous code");

    Ok(())
}
