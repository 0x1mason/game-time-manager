// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use std::env;
use std::io;
use std::path::PathBuf;
use std::{fmt::format, process::Command};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use tao::event::DeviceEvent;
use tao::event_loop::DeviceEventFilter;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    TrayEvent, TrayIconBuilder,
};

use winreg::enums::*;
use winreg::RegKey;

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

    let sm = Submenu::new("Run on start", true);
    let enable_run = MenuItem::new("Enable", false, None);
    let disable_run = MenuItem::new("Disable", true, None);
    sm.append_items(&[&enable_run, &disable_run]);

    let tray_menu = Menu::new();
    tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("Game Time Manager".to_string()),
                website: Some("http://gametimemanager.app".to_string()),
                license: Some(
                    "Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)"
                        .to_string(),
                ),
                copyright: Some("Copyright 2023".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &settings,
        &stop,
        &start,
        &sm,
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
    event_loop.run(move |tray_event, t, control_flow| {
        *control_flow = ControlFlow::Wait;

        // possible that the main process was killed in Task Manager, etc, so update state
        if let Ok(event) = tray_channel.try_recv() {
            match tray_event {
                tao::event::Event::NewEvents(cause) => {
                    let gtm_is_running = get_gtm_proc_running();
                    start.set_enabled(!gtm_is_running);
                    stop.set_enabled(gtm_is_running);

                    let run_on_start = hkcu_value_exists(
                        r"Software\Microsoft\Windows\CurrentVersion\Run",
                        "GameTimeManager",
                    );
                    enable_run.set_enabled(!run_on_start);
                    disable_run.set_enabled(run_on_start);
                }
                _ => (),
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            match event.id {
                id if id == settings.id() => {
                    std::process::Command::new("notepad")
                        .arg(config_path.as_str())
                        .spawn();
                }
                id if id == stop.id() => {
                    let mut system = System::new_all();
                    for x in system.processes_by_exact_name("GameTimeManager.exe") {
                        x.kill();
                    }
                    start.set_enabled(true);
                    stop.set_enabled(false);
                }
                id if id == start.id() => {
                    let mut gtm_path = env::current_exe().unwrap();
                    gtm_path.pop();
                    gtm_path.push("GameTimeManager.exe");
                    let gtm_path_str = gtm_path
                        .canonicalize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(r"\\?\", "");

                    Command::new(gtm_path_str).spawn();
                    start.set_enabled(false);
                    stop.set_enabled(true);
                }
                id if id == enable_run.id() => {
                    let mut gtm_path = env::current_exe().unwrap();
                    gtm_path.pop();
                    gtm_path.push("GameTimeManager.exe");
                    let gtm_path_str = gtm_path
                        .canonicalize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(r"\\?\", "");

                    hkcu_set_str_value(
                        r"Software\Microsoft\Windows\CurrentVersion\Run",
                        "GameTimeManager",
                        gtm_path_str.as_str(),
                    );
                    enable_run.set_enabled(false);
                    disable_run.set_enabled(true);
                }
                id if id == disable_run.id() => {
                    hkcu_delete_value(
                        r"Software\Microsoft\Windows\CurrentVersion\Run",
                        "GameTimeManager",
                    );
                    disable_run.set_enabled(false);
                    enable_run.set_enabled(true);
                }
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

fn hkcu_set_str_value(key_path: &str, value_name: &str, value_data: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (subkey, _) = hkcu
        .create_subkey(key_path)
        .expect("Failed to create or open registry key");

    subkey
        .set_value(value_name, &value_data)
        .expect("Failed to set registry value");
}

fn hkcu_delete_value(key_path: &str, value_name: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let sk = hkcu.open_subkey_with_flags(key_path, KEY_ALL_ACCESS);
    sk.unwrap()
        .delete_value(value_name)
        .expect("Failed to delete registry value");
}

fn hkcu_value_exists(key_path: &str, value_name: &str) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey(key_path) {
        Ok(sk) => return sk.get_raw_value(value_name).is_ok(),
        _ => return false,
    };
}
