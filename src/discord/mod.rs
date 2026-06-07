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

    pub fn set_activity(&mut self, track: &TrackInfo, config: &Config) -> Result<(), String> {
        let builder = activity::ActivityBuilder::new(config.clone());
        let build_fn = builder.build(track);

        self.client.set_activity(build_fn).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn clear_activity(&mut self) -> Result<(), String> {
        self.client.clear_activity().map_err(|e| e.to_string())?;
        Ok(())
    }
}
