#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rdev::{listen, Event};

fn listen_for_keys() {
    match listen(callback) {
        Ok(_) => {}
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

fn callback(event: Event) {
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}

fn main() {
    std::thread::spawn(|| {
        listen_for_keys();
    });

    tauri::Builder::default()
        .setup(move |app| {
            use tauri::Manager;
            let window = app.get_window("overlay").unwrap();

            let hwnd = window.hwnd().unwrap().0;
            let _pre_val;
            let hwnd = windows::Win32::Foundation::HWND(hwnd);
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::*;
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW
                    | WS_EX_COMPOSITED
                    | WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOPMOST;
                _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
            };
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use log::warn;
// use screenshots::Screen;
// use std::{env, time::Instant};

// fn main() {
//     env::set_var("RUST_LOG", "debug");
//     env::set_var("RUST_LOG_STYLE", "always");
//     // let args = Args::default();
//     env_logger::init();
//     warn!("alpha build!!!");

//     let start = Instant::now();
//     let screens = Screen::all().unwrap();

//     for screen in screens {
//         println!("capturer {screen:?}");
//         let mut image = screen.capture().unwrap();
//         // image
//         //     .save(format!("imgs/{}.png", screen.display_info.id))
//         //     .unwrap();
//     }

//     // let screen = Screen::from_point(100, 100).unwrap();
//     // println!("capturer {screen:?}");

//     // let image = screen.capture_area(300, 300, 300, 300).unwrap();
//     // image.save("target/capture_display_with_point.png").unwrap();
//     // println!("运行耗时: {:?}", start.elapsed());
// }
