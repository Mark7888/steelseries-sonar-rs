//! Streamer mode example for the SteelSeries Sonar API.
//!
//! This example demonstrates:
//! - Working with streamer mode
//! - Using different sliders (streaming vs monitoring)
//! - Toggling between classic and streamer modes

use steelseries_sonar::{Sonar, SonarError, STREAMER_SLIDER_NAMES};

#[tokio::main]
async fn main() -> Result<(), SonarError> {
    println!("SteelSeries Sonar Streamer Mode Example");
    println!("=======================================");

    // Create a new Sonar client
    println!("🔌 Connecting to SteelSeries Sonar...");
    let mut sonar = Sonar::new().await?;
    println!("✅ Connected!");

    // Check current mode
    let current_mode = sonar.is_streamer_mode().await?;
    println!("Current mode: {}", if current_mode { "Streamer" } else { "Classic" });

    // Enable streamer mode if not already enabled
    if !current_mode {
        println!("\n🎮 Enabling streamer mode...");
        sonar.set_streamer_mode(true).await?;
        println!("✅ Streamer mode enabled!");
    }

    // Show available sliders
    println!("\n📊 Available streamer sliders:");
    for slider in STREAMER_SLIDER_NAMES {
        println!("  • {}", slider);
    }

    // Demonstrate dual slider control
    println!("\n🎛️ Dual Slider Control Demo:");
    
    // Set different volumes for streaming and monitoring
    println!("Setting game volume for streaming slider to 80%...");
    sonar.set_volume("game", 0.8, Some("streaming")).await?;
    
    println!("Setting game volume for monitoring slider to 60%...");
    sonar.set_volume("game", 0.6, Some("monitoring")).await?;
    
    println!("✅ Different volumes set for each slider!");

    // Mute control with sliders
    println!("\n🔇 Streamer Mute Control:");
    
    // Mute chat capture for streaming but keep it for monitoring
    println!("Muting chat capture for streaming slider...");
    sonar.mute_channel("chatCapture", true, Some("streaming")).await?;
    
    println!("Keeping chat capture unmuted for monitoring slider...");
    sonar.mute_channel("chatCapture", false, Some("monitoring")).await?;
    
    println!("✅ Chat capture muted for stream but audible for monitoring!");

    // Show volume data in streamer mode
    println!("\n📈 Volume data in streamer mode:");
    let volume_data = sonar.get_volume_data().await?;
    println!("{:#}", volume_data);

    // Demonstrate mode switching
    println!("\n🔄 Mode Switching Demo:");
    
    println!("Switching back to classic mode...");
    sonar.set_streamer_mode(false).await?;
    println!("✅ Now in classic mode");

    // Show how volume control works in classic mode
    println!("\nSetting master volume in classic mode...");
    sonar.set_volume("master", 0.7, None).await?;
    println!("✅ Master volume set (no slider parameter needed)");

    // Switch back to streamer mode for final demo
    println!("\nSwitching back to streamer mode...");
    sonar.set_streamer_mode(true).await?;

    // Reset volumes
    println!("\n🔄 Resetting volumes for demo cleanup...");
    sonar.set_volume("game", 0.5, Some("streaming")).await?;
    sonar.set_volume("game", 0.5, Some("monitoring")).await?;
    sonar.mute_channel("chatCapture", false, Some("streaming")).await?;
    
    println!("✅ Demo completed!");
    println!("\n💡 Key takeaways:");
    println!("  • Streamer mode provides separate streaming and monitoring controls");
    println!("  • Use the slider parameter when in streamer mode");
    println!("  • Classic mode doesn't require the slider parameter");
    println!("  • Mode changes affect the API behavior and available controls");

    Ok(())
}
