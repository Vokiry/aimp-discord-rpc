use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_app_id")]
    pub app_id: u64,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_ms: u64,
    #[serde(default = "default_large_image")]
    pub large_image_key: String,
    #[serde(default = "default_small_play")]
    pub small_image_play: String,
    #[serde(default = "default_small_pause")]
    pub small_image_pause: String,
    #[serde(default = "default_true")]
    pub show_timestamps: bool,
    #[serde(default = "default_true")]
    pub show_album: bool,
}

fn default_app_id() -> u64 { 429559336982020107 }
fn default_poll_interval() -> u64 { 2000 }
fn default_large_image() -> String { "aimp_logo".into() }
fn default_small_play() -> String { "play".into() }
fn default_small_pause() -> String { "pause".into() }
fn default_true() -> bool { true }

impl Default for Config {
    fn default() -> Self {
        Self {
            app_id: default_app_id(),
            poll_interval_ms: default_poll_interval(),
            large_image_key: default_large_image(),
            small_image_play: default_small_play(),
            small_image_pause: default_small_pause(),
            show_timestamps: true,
            show_album: true,
        }
    }
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Self {
        let path = path.unwrap_or_else(default_config_path);
        if !path.exists() {
            log::warn!("Config file not found at {:?}, using defaults", path);
            return Config::default();
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to read config {:?}: {}", path, e);
                return Config::default();
            }
        };
        match toml::from_str(&content) {
            Ok(cfg) => {
                log::info!("Loaded config from {:?}", path);
                cfg
            }
            Err(e) => {
                log::error!("Failed to parse config {:?}: {}", path, e);
                Config::default()
            }
        }
    }
}

fn default_config_path() -> PathBuf {
    let mut path = dirs_config_dir();
    path.push("aimp-discord-rpc");
    path.push("config.toml");
    path
}

fn dirs_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."))
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var("HOME")
                    .ok()
                    .map(|h| PathBuf::from(h).join(".config"))
            })
            .unwrap_or_else(|| PathBuf::from("."))
    }
}
