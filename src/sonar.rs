//! SteelSeries Sonar API client.

use crate::error::{Result, SonarError};
use reqwest::Client;
use serde::{Deserialize};
use serde_json::Value;
use std::path::Path;

/// Valid audio channel names in SteelSeries Sonar.
pub const CHANNEL_NAMES: &[&str] = &["master", "game", "chatRender", "media", "aux", "chatCapture"];

/// Valid streamer slider names.
pub const STREAMER_SLIDER_NAMES: &[&str] = &["streaming", "monitoring"];

/// Core properties structure from SteelSeries Engine.
#[derive(Debug, Deserialize)]
pub struct CoreProps {
    #[serde(rename = "ggEncryptedAddress")]
    pub gg_encrypted_address: String,
}

/// Sub-application information structure.
#[derive(Debug, Deserialize)]
pub struct SubApp {
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "isReady")]
    pub is_ready: bool,
    #[serde(rename = "isRunning")]
    pub is_running: bool,
    pub metadata: SubAppMetadata,
}

/// Sub-application metadata.
#[derive(Debug, Deserialize)]
pub struct SubAppMetadata {
    #[serde(rename = "webServerAddress")]
    pub web_server_address: String,
}

/// Response from the /subApps endpoint.
#[derive(Debug, Deserialize)]
pub struct SubAppsResponse {
    #[serde(rename = "subApps")]
    pub sub_apps: SubApps,
}

/// Sub-applications container.
#[derive(Debug, Deserialize)]
pub struct SubApps {
    pub sonar: SubApp,
}

/// Main SteelSeries Sonar API client.
#[derive(Debug)]
pub struct Sonar {
    client: Client,
    #[allow(dead_code)]
    base_url: String,
    web_server_address: String,
    streamer_mode: bool,
    volume_path: String,
}

impl Sonar {
    /// Create a new Sonar client with default settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the SteelSeries Engine is not found or accessible.
    pub async fn new() -> Result<Self> {
        Self::with_config(None, None).await
    }

    /// Create a new Sonar client with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `app_data_path` - Custom path to the coreProps.json file
    /// * `streamer_mode` - Whether to use streamer mode (if None, will be auto-detected)
    ///
    /// # Errors
    ///
    /// Returns an error if the SteelSeries Engine is not found or accessible.
    pub async fn with_config(app_data_path: Option<&Path>, streamer_mode: Option<bool>) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let app_data_path = app_data_path.unwrap_or_else(|| {
            #[cfg(target_os = "windows")]
            {
                Path::new("C:\\ProgramData\\SteelSeries\\SteelSeries Engine 3\\coreProps.json")
            }
            #[cfg(not(target_os = "windows"))]
            {
                // For non-Windows systems, this would need to be adapted based on where
                // SteelSeries Engine might be installed
                Path::new("/tmp/coreProps.json") // Placeholder
            }
        });

        let base_url = Self::load_base_url(app_data_path).await?;
        let web_server_address = Self::load_server_address(&client, &base_url).await?;

        let detected_streamer_mode = match streamer_mode {
            Some(mode) => mode,
            None => Self::is_streamer_mode_internal(&client, &web_server_address).await?,
        };

        let volume_path = if detected_streamer_mode {
            "/volumeSettings/streamer".to_string()
        } else {
            "/volumeSettings/classic".to_string()
        };

        Ok(Self {
            client,
            base_url,
            web_server_address,
            streamer_mode: detected_streamer_mode,
            volume_path,
        })
    }

    /// Check if streamer mode is currently enabled.
    pub async fn is_streamer_mode(&self) -> Result<bool> {
        Self::is_streamer_mode_internal(&self.client, &self.web_server_address).await
    }

    async fn is_streamer_mode_internal(client: &Client, web_server_address: &str) -> Result<bool> {
        let url = format!("{}/mode/", web_server_address);
        let response = client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let mode: String = response.json().await?;
        Ok(mode == "stream")
    }

    /// Set streamer mode on or off.
    ///
    /// # Arguments
    ///
    /// * `streamer_mode` - Whether to enable streamer mode
    ///
    /// # Returns
    ///
    /// Returns the new streamer mode state.
    pub async fn set_streamer_mode(&mut self, streamer_mode: bool) -> Result<bool> {
        let mode = if streamer_mode { "stream" } else { "classic" };
        let url = format!("{}/mode/{}", self.web_server_address, mode);
        
        let response = self.client.put(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let new_mode: String = response.json().await?;
        self.streamer_mode = new_mode == "stream";
        
        self.volume_path = if self.streamer_mode {
            "/volumeSettings/streamer".to_string()
        } else {
            "/volumeSettings/classic".to_string()
        };

        Ok(self.streamer_mode)
    }

    /// Get volume data for all channels.
    pub async fn get_volume_data(&self) -> Result<Value> {
        let url = format!("{}{}", self.web_server_address, self.volume_path);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let volume_data: Value = response.json().await?;
        Ok(volume_data)
    }

    /// Set the volume for a specific channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - The audio channel name
    /// * `volume` - Volume level (0.0 to 1.0)
    /// * `streamer_slider` - Streamer slider to use in streamer mode
    pub async fn set_volume(&self, channel: &str, volume: f64, streamer_slider: Option<&str>) -> Result<Value> {
        if !CHANNEL_NAMES.contains(&channel) {
            return Err(SonarError::ChannelNotFound(channel.to_string()));
        }

        if !(0.0..=1.0).contains(&volume) {
            return Err(SonarError::InvalidVolume(volume));
        }

        let streamer_slider = streamer_slider.unwrap_or("streaming");
        if self.streamer_mode && !STREAMER_SLIDER_NAMES.contains(&streamer_slider) {
            return Err(SonarError::SliderNotFound(streamer_slider.to_string()));
        }

        let full_volume_path = if self.streamer_mode {
            format!("{}/{}", self.volume_path, streamer_slider)
        } else {
            self.volume_path.clone()
        };

        let url = format!("{}{}/{}/Volume/{}", 
            self.web_server_address, full_volume_path, channel, serde_json::to_string(&volume)?);
        
        let response = self.client.put(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Mute or unmute a specific channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - The audio channel name
    /// * `muted` - Whether to mute the channel
    /// * `streamer_slider` - Streamer slider to use in streamer mode
    pub async fn mute_channel(&self, channel: &str, muted: bool, streamer_slider: Option<&str>) -> Result<Value> {
        if !CHANNEL_NAMES.contains(&channel) {
            return Err(SonarError::ChannelNotFound(channel.to_string()));
        }

        let streamer_slider = streamer_slider.unwrap_or("streaming");
        if self.streamer_mode && !STREAMER_SLIDER_NAMES.contains(&streamer_slider) {
            return Err(SonarError::SliderNotFound(streamer_slider.to_string()));
        }

        let full_volume_path = if self.streamer_mode {
            format!("{}/{}", self.volume_path, streamer_slider)
        } else {
            self.volume_path.clone()
        };

        let mute_keyword = if self.streamer_mode { "isMuted" } else { "Mute" };

        let url = format!("{}{}/{}/{}/{}", 
            self.web_server_address, full_volume_path, channel, mute_keyword, serde_json::to_string(&muted)?);
        
        let response = self.client.put(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Get chat mix data.
    pub async fn get_chat_mix_data(&self) -> Result<Value> {
        let url = format!("{}/chatMix", self.web_server_address);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let chat_mix_data: Value = response.json().await?;
        Ok(chat_mix_data)
    }

    /// Set the chat mix volume.
    ///
    /// # Arguments
    ///
    /// * `mix_volume` - Mix volume level (-1.0 to 1.0)
    pub async fn set_chat_mix(&self, mix_volume: f64) -> Result<Value> {
        if !(-1.0..=1.0).contains(&mix_volume) {
            return Err(SonarError::InvalidMixVolume(mix_volume));
        }

        let url = format!("{}/chatMix?balance={}", 
            self.web_server_address, serde_json::to_string(&mix_volume)?);
        
        let response = self.client.put(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let result: Value = response.json().await?;
        Ok(result)
    }

    async fn load_base_url(app_data_path: &Path) -> Result<String> {
        if !app_data_path.exists() {
            return Err(SonarError::EnginePathNotFound);
        }

        let content = tokio::fs::read_to_string(app_data_path).await?;
        let core_props: CoreProps = serde_json::from_str(&content)?;
        
        Ok(format!("https://{}", core_props.gg_encrypted_address))
    }

    async fn load_server_address(client: &Client, base_url: &str) -> Result<String> {
        let url = format!("{}/subApps", base_url);
        let response = client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(SonarError::ServerNotAccessible(response.status().as_u16()));
        }

        let sub_apps_response: SubAppsResponse = response.json().await?;
        let sonar = &sub_apps_response.sub_apps.sonar;

        if !sonar.is_enabled {
            return Err(SonarError::SonarNotEnabled);
        }

        if !sonar.is_ready {
            return Err(SonarError::ServerNotReady);
        }

        if !sonar.is_running {
            return Err(SonarError::ServerNotRunning);
        }

        let web_server_address = &sonar.metadata.web_server_address;
        if web_server_address.is_empty() || web_server_address == "null" {
            return Err(SonarError::WebServerAddressNotFound);
        }

        Ok(web_server_address.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_names() {
        assert!(CHANNEL_NAMES.contains(&"master"));
        assert!(CHANNEL_NAMES.contains(&"game"));
        assert!(CHANNEL_NAMES.contains(&"chatRender"));
        assert!(CHANNEL_NAMES.contains(&"media"));
        assert!(CHANNEL_NAMES.contains(&"aux"));
        assert!(CHANNEL_NAMES.contains(&"chatCapture"));
    }

    #[test]
    fn test_streamer_slider_names() {
        assert!(STREAMER_SLIDER_NAMES.contains(&"streaming"));
        assert!(STREAMER_SLIDER_NAMES.contains(&"monitoring"));
    }
}
