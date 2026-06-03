mod config;
mod discord;
mod monitor;
mod player;
mod tray;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aimp-discord-rpc", version, about = "Discord Rich Presence for AIMP music player")]
struct Cli {
    #[arg(short, long, help = "Path to config file")]
    config: Option<std::path::PathBuf>,

    #[arg(long, help = "Discord Application ID (overrides config)")]
    app_id: Option<u64>,

    #[arg(long, help = "Poll interval in milliseconds")]
    poll_ms: Option<u64>,

    #[arg(short, long, help = "Enable verbose logging")]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .init();
    }

    let mut cfg = config::Config::load(cli.config);
    if let Some(app_id) = cli.app_id {
        cfg.app_id = app_id;
    }
    if let Some(ms) = cli.poll_ms {
        cfg.poll_interval_ms = ms;
    }

    log::info!("AIMP Discord RPC starting...");
    log::info!("App ID: {}, Poll interval: {}ms", cfg.app_id, cfg.poll_interval_ms);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        log::info!("Received Ctrl+C, shutting down...");
        r.store(false, Ordering::Relaxed);
    })
    .ok();

    monitor::run(cfg, running);

    log::info!("AIMP Discord RPC stopped");
}
