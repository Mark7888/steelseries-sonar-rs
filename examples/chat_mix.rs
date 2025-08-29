//! Chat mix example for the SteelSeries Sonar API.
//!
//! This example demonstrates:
//! - Getting chat mix data
//! - Setting chat mix levels
//! - Understanding chat mix range (-1.0 to 1.0)

use steelseries_sonar::{Sonar, SonarError};

#[tokio::main]
async fn main() -> Result<(), SonarError> {
    println!("SteelSeries Sonar Chat Mix Example");
    println!("==================================");

    // Create a new Sonar client
    println!("🔌 Connecting to SteelSeries Sonar...");
    let sonar = Sonar::new().await?;
    println!("✅ Connected!");

    // Get current chat mix data
    println!("\n🎙️ Getting current chat mix data...");
    let current_chat_mix = sonar.get_chat_mix_data().await?;
    println!("Current chat mix: {:#}", current_chat_mix);

    println!("\n📖 Chat Mix Range Information:");
    println!("  • -1.0: Maximum game audio, minimum chat");
    println!("  •  0.0: Balanced mix");
    println!("  • +1.0: Maximum chat audio, minimum game");

    // Demonstrate different chat mix levels
    println!("\n🎛️ Chat Mix Control Demo:");

    // Favor game audio
    println!("\n1. Setting chat mix to favor game audio (-0.5)...");
    sonar.set_chat_mix(-0.5).await?;
    let mix_data = sonar.get_chat_mix_data().await?;
    println!("   Chat mix set: {:#}", mix_data);
    
    // Wait a moment for user to hear the change
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Balanced mix
    println!("\n2. Setting chat mix to balanced (0.0)...");
    sonar.set_chat_mix(0.0).await?;
    let mix_data = sonar.get_chat_mix_data().await?;
    println!("   Chat mix set: {:#}", mix_data);
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Favor chat audio
    println!("\n3. Setting chat mix to favor chat audio (+0.5)...");
    sonar.set_chat_mix(0.5).await?;
    let mix_data = sonar.get_chat_mix_data().await?;
    println!("   Chat mix set: {:#}", mix_data);
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Extreme settings demonstration
    println!("\n🎯 Extreme Settings Demo:");
    
    // Maximum game audio
    println!("\n4. Maximum game audio (-1.0)...");
    sonar.set_chat_mix(-1.0).await?;
    println!("   Chat audio is now at minimum");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Maximum chat audio  
    println!("\n5. Maximum chat audio (+1.0)...");
    sonar.set_chat_mix(1.0).await?;
    println!("   Game audio is now at minimum");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Reset to balanced
    println!("\n🔄 Resetting to balanced mix...");
    sonar.set_chat_mix(0.0).await?;
    println!("✅ Chat mix reset to balanced (0.0)");

    // Error handling demonstration
    println!("\n⚠️ Error Handling Demo:");
    
    println!("Trying to set invalid chat mix value (2.0)...");
    match sonar.set_chat_mix(2.0).await {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(SonarError::InvalidMixVolume(volume)) => {
            println!("   ✅ Correctly caught invalid volume: {}", volume);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    println!("Trying to set invalid chat mix value (-2.0)...");
    match sonar.set_chat_mix(-2.0).await {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(SonarError::InvalidMixVolume(volume)) => {
            println!("   ✅ Correctly caught invalid volume: {}", volume);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Final status
    println!("\n📊 Final chat mix status:");
    let final_mix = sonar.get_chat_mix_data().await?;
    println!("{:#}", final_mix);

    println!("\n✅ Chat mix demo completed!");
    println!("\n💡 Key takeaways:");
    println!("  • Chat mix range is -1.0 to +1.0");
    println!("  • Negative values favor game audio");
    println!("  • Positive values favor chat audio");  
    println!("  • 0.0 provides a balanced mix");
    println!("  • Invalid values are automatically caught and rejected");

    Ok(())
}
