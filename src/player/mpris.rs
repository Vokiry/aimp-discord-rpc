use std::time::Duration;

use mpris::{PlaybackStatus, Player, PlayerFinder};

use super::types::{PlayerState, TrackInfo};

#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
    PlayPause,
    Stop,
    Next,
    Previous,
}

pub struct PlayerClient {
    player: Player,
}

impl PlayerClient {
    pub fn new() -> Option<Self> {
        let finder = PlayerFinder::new().ok()?;
        let players = finder.find_all().ok()?;
        let player = players
            .into_iter()
            .find(|p| p.identity().to_lowercase().contains("aimp"))?;
        log::info!("Connected to MPRIS player: {}", player.identity());
        Some(Self { player })
    }

    pub fn get_info(&self) -> Option<TrackInfo> {
        let status = self.player.get_playback_status().ok()?;
        let metadata = self.player.get_metadata().ok()?;
        let position = self.player.get_position().ok().unwrap_or(Duration::ZERO);

        let state = match status {
            PlaybackStatus::Playing => PlayerState::Playing,
            PlaybackStatus::Paused => PlayerState::Paused,
            PlaybackStatus::Stopped => PlayerState::Stopped,
        };

        let title = metadata.title().unwrap_or("").to_string();
        let artist = metadata
            .artists()
            .and_then(|a| a.first().copied())
            .unwrap_or("")
            .to_string();
        let album = metadata.album_name().unwrap_or("").to_string();
        let duration = metadata.length().unwrap_or(Duration::ZERO);
        let url = metadata.url().unwrap_or("");
        let art_url = metadata.art_url().unwrap_or("").to_string();
        let is_radio = url.starts_with("http://") || url.starts_with("https://");

        Some(TrackInfo {
            title,
            artist,
            album,
            duration_ms: duration.as_millis() as u32,
            position_ms: (position.as_micros() / 1000) as u32,
            state,
            is_radio,
            art_url,
        })
    }

    pub fn send_command(&self, cmd: PlayerCommand) {
        let result = match cmd {
            PlayerCommand::PlayPause => self.player.play_pause(),
            PlayerCommand::Stop => self.player.stop(),
            PlayerCommand::Next => self.player.next(),
            PlayerCommand::Previous => self.player.previous(),
        };
        if let Err(e) = result {
            log::error!("Failed to send player command {:?}: {}", cmd, e);
        }
    }
}
