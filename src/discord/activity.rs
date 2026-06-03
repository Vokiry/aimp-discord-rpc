use discord_rpc_client::models::rich_presence::Activity;

use crate::player::types::{PlayerState, TrackInfo};
use crate::config::Config;

pub struct ActivityBuilder {
    config: Config,
}

impl ActivityBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn build(&self, track: &TrackInfo) -> impl FnOnce(Activity) -> Activity {
        let details = self.format_field(&track.title);
        let state = self.format_field(&track.artist);
        let large_text = if self.config.show_album && !track.album.is_empty() {
            self.format_field(&track.album)
        } else {
            "AIMP".into()
        };
        let small_image = if track.state == PlayerState::Playing {
            self.config.small_image_play.clone()
        } else {
            self.config.small_image_pause.clone()
        };
        let small_text: String = if track.state == PlayerState::Playing {
            "Playing".into()
        } else {
            "Paused".into()
        };
        let show_timestamps = self.config.show_timestamps && track.state == PlayerState::Playing;
        let large_image = self.config.large_image_key.clone();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let start = now - track.position_ms as u64;
        let end = if track.duration_ms > 0 && !track.is_radio {
            Some(start + track.duration_ms as u64)
        } else {
            None
        };
        let is_radio = track.is_radio;

        move |activity: Activity| {
            let act = activity
                .kind(2) // ActivityType::Listening
                .details(&details)
                .state(&state)
                .assets(|a| a
                    .large_image(&large_image)
                    .large_text(&large_text)
                    .small_image(&small_image)
                    .small_text(&small_text)
                )
                .instance(true);

            if show_timestamps {
                if is_radio || end.is_none() {
                    act.timestamps(|t| t.start(start))
                } else {
                    act.timestamps(|t| t.start(start).end(end.unwrap()))
                }
            } else {
                act
            }
        }
    }

    fn format_field(&self, field: &str) -> String {
        if field.len() > 120 {
            format!("{}...", &field[..117])
        } else {
            field.to_string()
        }
    }
}
