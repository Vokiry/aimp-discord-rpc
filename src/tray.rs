use std::path::Path;

use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};
use tray_icon::menu::MenuEvent;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

use crate::player::mpris::PlayerCommand;
use crate::player::types::TrackInfo;

pub struct TrayContext {
    pub play_pause: MenuItem,
    pub stop: MenuItem,
    pub next: MenuItem,
    pub prev: MenuItem,
    pub quit: MenuItem,
    pub tray: TrayIcon,
}

const ICON_SIZE: u32 = 48;

fn make_default_icon() -> Icon {
    let mut rgba = Vec::with_capacity((ICON_SIZE * ICON_SIZE * 4) as usize);
    let center = ICON_SIZE as f64 / 2.0;
    let radius = center - 2.0;

    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let dx = x as f64 - center;
            let dy = y as f64 - center;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < radius - 1.0 {
                let t = dist / radius;
                let r = (0.0 + (1.0 - t) * 40.0) as u8;
                let g = (100.0 + (1.0 - t) * 60.0) as u8;
                let b = (180.0 + (1.0 - t) * 75.0) as u8;
                rgba.extend_from_slice(&[r, g, b, 255]);
            } else if dist < radius + 1.0 {
                let alpha = ((radius + 1.0 - dist) * 255.0) as u8;
                rgba.extend_from_slice(&[0x00, 0x64, 0xE0, alpha]);
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }

    Icon::from_rgba(rgba, ICON_SIZE, ICON_SIZE).unwrap_or_else(|e| {
        log::error!("Failed to create icon: {}", e);
        panic!("icon required");
    })
}

fn load_icon_from_file(path: &Path) -> Option<Icon> {
    let img = image::open(path).ok()?;
    let resized = img.resize_exact(ICON_SIZE, ICON_SIZE, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();
    Icon::from_rgba(rgba.into_raw(), ICON_SIZE, ICON_SIZE).ok()
}

pub fn update_icon(ctx: &TrayContext, info: &TrackInfo) {
    if !info.art_url.is_empty() {
        if let Ok(parsed) = url::Url::parse(&info.art_url) {
            if parsed.scheme() == "file" {
                if let Ok(path) = parsed.to_file_path() {
                    if let Some(icon) = load_icon_from_file(&path) {
                        ctx.tray.set_icon(Some(icon)).ok();
                        return;
                    }
                }
            }
        }
    }

    ctx.tray.set_icon(Some(make_default_icon())).ok();
}

pub fn create_tray() -> TrayContext {
    let icon = make_default_icon();

    let play_pause = MenuItem::new("Воспроизвести / Пауза", true, None);
    let stop = MenuItem::new("Стоп", true, None);
    let next = MenuItem::new("Следующий", true, None);
    let prev = MenuItem::new("Предыдущий", true, None);
    let quit = MenuItem::new("Выход", true, None);

    let menu = Menu::new();
    menu.append(&play_pause).ok();
    menu.append(&stop).ok();
    menu.append(&next).ok();
    menu.append(&prev).ok();
    menu.append(&PredefinedMenuItem::separator()).ok();
    menu.append(&quit).ok();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("AIMP Discord RPC")
        .with_icon(icon)
        .build()
        .expect("Failed to build tray icon");

    TrayContext {
        play_pause,
        stop,
        next,
        prev,
        quit,
        tray,
    }
}

pub fn handle_event(ctx: &TrayContext, event: &MenuEvent) -> Option<PlayerCommand> {
    if event.id == *ctx.play_pause.id() {
        Some(PlayerCommand::PlayPause)
    } else if event.id == *ctx.stop.id() {
        Some(PlayerCommand::Stop)
    } else if event.id == *ctx.next.id() {
        Some(PlayerCommand::Next)
    } else if event.id == *ctx.prev.id() {
        Some(PlayerCommand::Previous)
    } else {
        None
    }
}

pub fn update_tooltip(ctx: &TrayContext, info: &TrackInfo) {
    ctx.tray.set_tooltip(Some(&info.format_tooltip())).ok();
}
