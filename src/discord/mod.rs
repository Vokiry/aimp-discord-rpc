pub mod activity;

use discord_rpc_client::Client;

use crate::player::types::TrackInfo;
use crate::config::Config;

pub struct DiscordClient {
    client: Client,
}

impl DiscordClient {
    pub fn new(app_id: u64) -> Self {
        let client = Client::new(app_id);
        Self { client }
    }

    pub fn start(&mut self) {
        self.client.start();
        log::info!("Discord RPC client started");
    }

    pub fn set_activity(&mut self, track: &TrackInfo, config: &Config) {
        let builder = activity::ActivityBuilder::new(config.clone());
        let build_fn = builder.build(track);

        if let Err(e) = self.client.set_activity(build_fn) {
            log::error!("Failed to set Discord activity: {}", e);
        }
    }

    pub fn clear_activity(&mut self) {
        if let Err(e) = self.client.clear_activity() {
            log::error!("Failed to clear Discord activity: {}", e);
        }
    }
}
