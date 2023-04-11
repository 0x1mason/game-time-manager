extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use crossbeam::channel::{Receiver, Sender};
use nwd::NwgUi;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use windows::Win32::{
    Graphics::Gdi::{GetMonitorInfoA, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTOPRIMARY},
    UI::WindowsAndMessaging::GetForegroundWindow,
};

use winapi::um::wingdi::{SetTextColor, RGB};
use winapi::um::winuser::{GetDC, SetLayeredWindowAttributes, LWA_COLORKEY};

use crate::{config, system_provider, watcher};

#[derive(NwgUi, Default)]
pub struct Overlay {
    #[nwg_control(size: (400, 120), position: (400, 0), flags: "POPUP", ex_flags: winapi::um::winuser::WS_EX_TOPMOST|winapi::um::winuser::WS_EX_LAYERED)]
    #[nwg_events( OnInit: [Overlay::on_init], OnWindowClose: [Overlay::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, margin: [0,0,0,0], spacing: 0)]
    layout: nwg::GridLayout,

    #[nwg_resource(family: "Ubuntu", size: 120, weight: 700)]
    font: nwg::Font,

    #[nwg_control(text: "", size: (400, 120), font: Some(&data.font), h_align: HTextAlign::Right, background_color: Some([255, 0, 0]))]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    time_label: nwg::Label,

    #[nwg_control]
    #[nwg_events(OnNotice: [Overlay::on_notice])]
    notice: nwg::Notice,

    text: Arc<Mutex<String>>,
}

impl Overlay {
    pub fn new(text: String) -> Self {
        let mut s = Self {
            ..Default::default()
        };

        s.text = Arc::new(Mutex::new(text));

        return s;
    }

    fn on_init(&self) {
        match self.window.handle {
            nwg::ControlHandle::Hwnd(hwnd) => unsafe {
                SetLayeredWindowAttributes(hwnd, RGB(255, 0, 0), 0, LWA_COLORKEY);
            },
            _ => {
                panic!("Bad handle type for window!")
            }
        }

        let notice = self.notice.sender();

        let display_text = self.text.clone();

        thread::spawn(move || {
            let cfg = match config::load() {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let text = display_text.lock().unwrap();

            for p in cfg.overlay.show_pattern.iter() {
                let delay: i32 = *p;

                if delay > 0 {
                    *display_text.lock().unwrap() = text.to_string();
                } else {
                    *display_text.lock().unwrap() = String::from("");
                }

                notice.notice();

                thread::sleep(Duration::from_secs(delay.abs() as u64));
            }

            // TODO use proper signaling
            *display_text.lock().unwrap() = String::from("@@close");
            notice.notice();
        });
    }

    fn on_notice(&self) {
        let cfg = match config::load() {
            Ok(c) => c,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        match self.text.lock().unwrap().as_str() {
            "@@close" => self.window.close(),
            "" => {
                self.window.set_visible(false);
                // unsafe {
                //     let hwnd = self.time_label.handle.hwnd().unwrap();
                //     let hdc = GetDC(hwnd);
                //     // // Color constants.
                //     // const COLORREF rgbRed   =  0x000000FF;
                //     // const COLORREF rgbGreen =  0x0000FF00;
                //     // const COLORREF rgbBlue  =  0x00FF0000;
                //     // const COLORREF rgbBlack =  0x00000000;
                //     // const COLORREF rgbWhite =  0x00FFFFFF;
                //     SetTextColor(hdc, 0x00FFFFFF);
                // }
            }
            text => {
                self.time_label.set_text(text);

                let (x, y) = get_position(
                    self.time_label.size(),
                    cfg.overlay.x_offset,
                    cfg.overlay.y_offset,
                );

                self.window.set_position(x, y);
                self.window.set_visible(true);
            }
        }
    }

    fn close(&self) {
        nwg::stop_thread_dispatch()
    }
}

fn get_position(label_sz: (u32, u32), x_offset: u32, y_offset: u32) -> (i32, i32) {
    let mut minf = MONITORINFO::default();
    minf.cbSize = std::mem::size_of::<MONITORINFO>() as _;

    unsafe {
        let hwnd = GetForegroundWindow();
        let hmnt = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
        let _res = GetMonitorInfoA(hmnt, &mut minf as _);
    }

    let lft_boundary = minf.rcMonitor.left;
    let rt_boundary = minf.rcMonitor.right - (label_sz.0 as i32);
    let pos_x = ((rt_boundary - lft_boundary) as f32 * x_offset as f32 / 100_f32).floor() as i32;

    let top_boundary = minf.rcMonitor.top;
    let bottom_boundary = minf.rcMonitor.bottom - (label_sz.1 as i32);
    let pos_y =
        ((bottom_boundary - top_boundary) as f32 * y_offset as f32 / 100_f32).floor() as i32;

    return (pos_x, pos_y);
}
