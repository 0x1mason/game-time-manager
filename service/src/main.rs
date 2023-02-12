#![allow(clippy::single_match)]

use futures::executor::block_on;
use futures::StreamExt;
use futures_ticker::Ticker;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, DeviceEventFilter, EventLoop},
    window::{Theme, WindowBuilder},
};

fn main() {
    thread::spawn(check_procs_sync);
    run_loop();
    // block_on(check_procs())

    // https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-serde
    // /mnt/c/Program\ Files\ \(x86\)/Steam/SteamApps/libraryfolders.vdf
    // C:\Program Files (x86)\Steam\SteamApps\libraryfolders.vdf
}

fn run_loop() {
    let event_loop = EventLoop::new();
    event_loop.set_device_event_filter(DeviceEventFilter::Always);

    let window = WindowBuilder::new()
        //.with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .with_decorations(false)
        .with_transparent(true)
        //.with_theme(Some(Theme::Dark))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{event:?}");

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

// ISteamApps/GetAppList
// api/appdetails?appids=

fn check_procs_sync() {
    let dur = Duration::from_secs(60);

    let ignore_procs = vec!["GameOverlayUI.exe", "steamwebhelper.exe"];
    let mut s = System::new_all();
    let mut proc_pid: HashMap<String, Pid> = HashMap::new();
    let mut proc_start: HashMap<Pid, SystemTime> = HashMap::new();
    let mut last = Instant::now();
    let mut shown = false;

    loop {
        s.refresh_all();
        shown = false;

        for steam_proc in s.processes_by_exact_name("steam.exe") {
            for (pid, proc) in s.processes() {
                if proc.parent() == Some(steam_proc.pid()) && !ignore_procs.contains(&proc.name()) {
                    proc_pid.entry(proc.name().to_string()).or_insert(*pid);

                    if !proc_start.contains_key(pid) {
                        proc_start.insert(*pid, SystemTime::now());
                        println!("updated map for {} {}", pid, proc.name());
                    } else {
                        let time = proc_start.get(pid);
                        let max = 2 * 60 * 60; // 2 hours
                        let allowed = Duration::from_secs(max);

                        match time.expect("time should be valid").elapsed() {
                            Err(e) => println!("unexpected error: {}", e.to_string()),
                            Ok(used) => {
                                if used >= allowed {
                                    println!("DIE!");
                                    proc.kill();
                                }
                            }
                        };
                    }

                    if !shown && last.elapsed() >= Duration::from_secs(15 * 60) {
                        println!("15 minutes");
                        last = Instant::now();
                        shown = true;
                    }
                }
            }
        }

        thread::sleep(dur);
    }
}

async fn check_procs() {
    let dur = Duration::from_secs(60);
    let mut tick = Ticker::new(dur);

    let ignore_procs = vec!["GameOverlayUI.exe", "steamwebhelper.exe"];
    let mut s = System::new_all();
    let mut proc_start: HashMap<Pid, SystemTime> = HashMap::new();
    //let mut i = 0;
    unsafe {
        // https://www.jendrikillner.com/post/rust-game-part-2/
        // https://users.rust-lang.org/t/code-review-on-windows-api-usage/62921
        //         let hwnd = GetTopWindow(None);
        //        println!("{:?}", hwnd);
        // let x: i32 = hwnd.into();
        // get the details of the window
        //   GetWindowInfo
        // get pid
        //   GetWindowThreadProcessId

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
    }

    loop {
        // }
        s.refresh_all();

        for steam_proc in s.processes_by_exact_name("steam.exe") {
            for (pid, proc) in s.processes() {
                if proc.parent() == Some(steam_proc.pid()) && !ignore_procs.contains(&proc.name()) {
                    if !proc_start.contains_key(pid) {
                        proc_start.insert(*pid, SystemTime::now());
                        println!("updated map for {} {}", pid, proc.name());
                    } else {
                        let time = proc_start.get(pid).unwrap();
                        let max = 2 * 60 * 60; // 2 hours
                        let allowed = Duration::from_secs(max);

                        if time.elapsed().unwrap() >= allowed {
                            println!("DIE!");
                            proc.kill();
                        }
                    }
                }
            }
        }

        // i+=1;
        // if i >6 {
        //     break
        // }

        tick.next().await;
    }
}

async fn really_do_it() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .with_decorations(false)
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        println!("{event:?}");

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
