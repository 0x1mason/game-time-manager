// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayEvent, TrayIconBuilder,
};

fn main() {
    let mut exe_path = std::env::current_exe().unwrap();
    exe_path.pop();
    let dir = exe_path.to_str().unwrap().to_string();

    let config_path = format!("{}\\{}", dir, "config.toml");
    let icon_path = format!("{}\\{}", dir, "icons\\timer256.png");
    let icon = load_icon(std::path::Path::new(icon_path.as_str()));

    let gtm_is_running = get_gtm_proc_running();

    let settings = MenuItem::new("Settings", true, None);
    let stop = MenuItem::new("Stop", gtm_is_running, None);
    let start = MenuItem::new("Start", !gtm_is_running, None);

    let tray_menu = Menu::new();
    tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("Game Time Manager".to_string()),
                copyright: Some("Copyright 2023".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &settings,
        &stop,
        &start,
    ]);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("Game Time Manager")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayEvent::receiver();

    // see
    //   https://github.com/tauri-apps/muda/blob/dev/examples/tao.rs
    //   https://github.com/tauri-apps/tray-icon/blob/dev/examples/tao.rs

    let event_loop = EventLoop::new();
    event_loop.run(move |tray_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // possible that the main process was killed in Task Manager, etc, so update state
        if let Ok(event) = tray_channel.try_recv() {
            match tray_event {
                tao::event::Event::NewEvents(cause) => {
                    let gtm_is_running = get_gtm_proc_running();
                    start.set_enabled(!gtm_is_running);
                    stop.set_enabled(gtm_is_running);
                }
                _ => (),
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");

            match event.id {
                id if id == settings.id() => {
                    std::process::Command::new("notepad")
                        .arg(config_path.as_str())
                        .spawn();
                }
                id if id == stop.id() => {}
                id if id == start.id() => {}
                _ => (),
            }
        }
    })
}

fn load_icon(path: &std::path::Path) -> tray_icon::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}

fn get_gtm_proc_running() -> bool {
    let system = System::new_all();
    let gtm_procs = system.processes_by_exact_name("GameTimeManager.exe");
    gtm_procs.count() > 0
}
