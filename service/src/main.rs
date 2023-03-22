#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwg::NativeUi;
use overlay::Overlay;

mod config;
mod overlay;
mod system_provider;

mod watcher;

fn main() {
    let font_data = include_bytes!("fonts\\Ubuntu\\Ubuntu-Bold.ttf");
    let mut font_data_mut = font_data.to_vec();
    let mem_font =
        nwg::Font::add_memory_font(&mut font_data_mut).expect("Failed to load font from memory");

    nwg::init().expect("Failed to init Native Windows GUI");

    let mut win: Overlay = Default::default();

    let (close_sender, closer) = crossbeam::channel::bounded(1);
    win.close_sender = Some(close_sender);
    win.closer = Some(closer);

    let _ui = overlay::Overlay::build_ui(win).expect("Failed to build UI");
    nwg::dispatch_thread_events();

    nwg::Font::remove_memory_font(mem_font);
}
