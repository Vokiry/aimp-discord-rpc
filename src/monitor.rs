use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam_channel::unbounded;

use crate::config::Config;
use crate::discord::DiscordClient;
use crate::player::mpris::PlayerClient;
use crate::player::types::{PlayerState, TrackInfo};
use crate::tray;

enum DiscordMessage {
    Update(TrackInfo),
    Clear,
}

pub fn run(config: Config, running: Arc<AtomicBool>) {
    gtk::init().expect("Failed to initialize GTK");

    let mut player = loop {
        if !running.load(Ordering::Relaxed) {
            return;
        }
        match PlayerClient::new() {
            Some(p) => break p,
            None => {
                log::info!("Waiting for AIMP (MPRIS)...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    };

    let (discord_tx, discord_rx) = unbounded::<DiscordMessage>();
    let discord_config = config.clone();
    thread::Builder::new()
        .name("discord".into())
        .spawn(move || {
            let mut client = DiscordClient::new(discord_config.app_id);
            client.start();
            while let Ok(msg) = discord_rx.recv() {
                match msg {
                    DiscordMessage::Update(ref track) => {
                        client.set_activity(track, &discord_config);
                    }
                    DiscordMessage::Clear => {
                        client.clear_activity();
                    }
                }
            }
        })
        .expect("Failed to spawn Discord thread");

    let tray_ctx = tray::create_tray();
    log::info!("Tray icon initialized");

    let mut previous: Option<TrackInfo> = None;

    while running.load(Ordering::Relaxed) {
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }

        match player.get_info() {
            Some(info) => {
                let should_update = match &previous {
                    Some(prev) => {
                        prev.title != info.title
                            || prev.artist != info.artist
                            || prev.state != info.state
                            || prev.album != info.album
                            || prev.art_url != info.art_url
                            || (info.state == PlayerState::Playing)
                    }
                    None => true,
                };

                if should_update {
                    if info.state == PlayerState::Stopped {
                        let _ = discord_tx.send(DiscordMessage::Clear);
                        log::info!("Activity cleared (stopped)");
                    } else {
                        let _ = discord_tx.send(DiscordMessage::Update(info.clone()));
                        log::info!(
                            "Updated: {} - {} [{}]",
                            info.artist,
                            info.title,
                            if info.state == PlayerState::Playing {
                                "▶"
                            } else {
                                "⏸"
                            }
                        );
                    }

                    tray::update_icon(&tray_ctx, &info);
                    previous = Some(info.clone());
                }

                tray::update_tooltip(&tray_ctx, &info);
            }
            None => {
                log::warn!("Lost connection to AIMP, reconnecting...");
                match PlayerClient::new() {
                    Some(p) => {
                        log::info!("Reconnected to AIMP");
                        player = p;
                    }
                    None => {
                        thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                }
                continue;
            }
        }

        while let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
            if event.id == *tray_ctx.quit.id() {
                running.store(false, Ordering::Relaxed);
                break;
            }
            if let Some(cmd) = tray::handle_event(&tray_ctx, &event) {
                player.send_command(cmd);
            }
        }
        if !running.load(Ordering::Relaxed) {
            break;
        }

        thread::sleep(Duration::from_millis(config.poll_interval_ms));
    }

    let _ = discord_tx.send(DiscordMessage::Clear);
    log::info!("Monitor stopped");
}
