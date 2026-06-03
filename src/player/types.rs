#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration_ms: u32,
    pub position_ms: u32,
    pub state: PlayerState,
    pub is_radio: bool,
    pub art_url: String,
}

impl TrackInfo {
    pub fn format_tooltip(&self) -> String {
        if self.state == PlayerState::Stopped {
            return "AIMP — не играет".into();
        }
        let mut text = format!("{} — {}", self.title, self.artist);
        if self.duration_ms > 0 {
            let current = self.position_ms / 1000;
            let total = self.duration_ms / 1000;
            use std::fmt::Write;
            write!(
                text,
                "\n{:02}:{:02} / {:02}:{:02}",
                current / 60, current % 60,
                total / 60, total % 60
            )
            .ok();
        }
        text
    }
}
