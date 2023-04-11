#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwg::NativeUi;
use overlay::Overlay;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{rc, thread};

mod config;
mod overlay;
mod system_provider;

mod watcher;

fn main() {
    let (close_sender, closer) = crossbeam::channel::bounded::<()>(1);
    let (sender, receiver) = crossbeam::channel::unbounded();

    let watch_handle = thread::spawn(|| {
        let sysprovider = &system_provider::Win32Provider::new();
        let watcher = watcher::Watcher::new();
        watcher.watch(sysprovider, sender, closer);
    });

    let font_data = include_bytes!("fonts\\Ubuntu\\Ubuntu-Bold.ttf");
    let mut font_data_mut = font_data.to_vec();
    let mem_font =
        nwg::Font::add_memory_font(&mut font_data_mut).expect("Failed to load font from memory");

    nwg::init().expect("Failed to init Native Windows GUI");

    thread::spawn(move || {
        for rcv in receiver {
            let overlay = overlay::Overlay::new(rcv);

            let _ui = overlay::Overlay::build_ui(overlay).expect("Failed to build UI");
            nwg::dispatch_thread_events();
        }
    });

    let term = Arc::new(AtomicBool::new(false));
    let sigs = [
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGABRT,
    ];

    for sig in sigs.iter() {
        signal_hook::flag::register(*sig, Arc::clone(&term)).expect("failed to register flag");
    }

    while !term.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(500));
    }

    println!("exiting...");

    close_sender.send(()).expect("failed to send close message");
    watch_handle.join().expect("watch handle failed to join");
    nwg::Font::remove_memory_font(mem_font);
}
