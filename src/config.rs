use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Engine / game type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EngineVersion {
    RSDKv3,
    RSDKv4,
    RSDKv5,
    Sonic1Forever,
    Sonic2Absolute,
    Sonic3AIR,
}

impl EngineVersion {
    pub fn as_str(&self) -> &str {
        match self {
            EngineVersion::RSDKv3 => "RSDKv3",
            EngineVersion::RSDKv4 => "RSDKv4",
            EngineVersion::RSDKv5 => "RSDKv5",
            EngineVersion::Sonic1Forever => "Sonic 1 Forever",
            EngineVersion::Sonic2Absolute => "Sonic 2 Absolute",
            EngineVersion::Sonic3AIR => "Sonic 3 AIR",
        }
    }

    pub fn from_index(i: u32) -> Self {
        match i {
            0 => EngineVersion::RSDKv3,
            1 => EngineVersion::RSDKv4,
            2 => EngineVersion::RSDKv5,
            3 => EngineVersion::Sonic1Forever,
            4 => EngineVersion::Sonic2Absolute,
            5 => EngineVersion::Sonic3AIR,
            _ => EngineVersion::RSDKv5,
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            EngineVersion::RSDKv3 => 0,
            EngineVersion::RSDKv4 => 1,
            EngineVersion::RSDKv5 => 2,
            EngineVersion::Sonic1Forever => 3,
            EngineVersion::Sonic2Absolute => 4,
            EngineVersion::Sonic3AIR => 5,
        }
    }
}

/// A single game profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProfile {
    pub id: String,
    pub name: String,
    pub data_path: String,
    pub executable_path: String,
    pub engine_version: EngineVersion,
    pub mods_folder: String,
    pub cover_image: String,
}

impl GameProfile {
    pub fn new(
        name: String,
        data_path: String,
        executable_path: String,
        engine_version: EngineVersion,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            data_path,
            executable_path,
            engine_version,
            mods_folder: String::new(),
            cover_image: String::new(),
        }
    }
}

/// App configuration stored in ~/.config/rsdk-launcher/config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub games: Vec<GameProfile>,
    pub selected_game_id: Option<String>,
    pub deploy_method: String, // "symlink" or "copy"
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            games: Vec::new(),
            selected_game_id: None,
            deploy_method: "symlink".to_string(),
        }
    }
}

impl AppConfig {
    /// Get the config file path
    fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("rsdk-launcher");
        config_dir.join("config.json")
    }

    /// Load config from disk, or return default
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Save config to disk
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Cannot create config dir: {}", e))?;
        }
        let json =
            serde_json::to_string_pretty(self).map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&path, json).map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    /// Add a game profile
    pub fn add_game(&mut self, game: GameProfile) {
        self.games.push(game);
    }

    /// Remove a game profile by ID
    pub fn remove_game(&mut self, id: &str) {
        self.games.retain(|g| g.id != id);
        if self.selected_game_id.as_deref() == Some(id) {
            self.selected_game_id = self.games.first().map(|g| g.id.clone());
        }
    }

    /// Get a game by ID
    pub fn get_game(&self, id: &str) -> Option<&GameProfile> {
        self.games.iter().find(|g| g.id == id)
    }

    /// Update a game profile
    pub fn update_game(&mut self, updated: GameProfile) {
        if let Some(game) = self.games.iter_mut().find(|g| g.id == updated.id) {
            *game = updated;
        }
    }
}
