#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use opencv::core;
use opencv::core::Vector;
use opencv::dnn;
use opencv::imgproc;
use opencv::prelude::*;
use screenshots::Screen;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use tesseract::{Tesseract, OCR};

fn capture_screenshot() -> Result<Mat, Box<dyn std::error::Error>> {
    // use log::warn;
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

    let screen = Screen::from_point(100, 100)?;
    println!("capturer {screen:?}");

    let image = screen.capture_area(300, 300, 300, 300)?;
    // image.save("sus.png").unwrap();
    //     // println!("运行耗时: {:?}", start.elapsed());
    // }
    let rows = image.height() as i32;
    let cols = image.width() as i32;
    let raw_data = image.into_raw();
    let mat = unsafe {
        core::Mat::new_rows_cols_with_data(
            rows,
            cols,
            core::CV_8UC4, // Assuming the image is in BGRA format (4 channels)
            raw_data.as_ptr() as *mut c_void,
            (4 * cols) as usize, // step, assuming 4 channels
        )?
    };

    // let mat = imgcodecs::imdecode(&opencv::core::Mat::from(&data)?, imgcodecs::IMREAD_COLOR)?;
    // let mat = Mat::default();

    // Convert BGRA to BGR (if necessary for your use case)
    let mut bgr_mat = Mat::default();
    imgproc::cvt_color(&mat, &mut bgr_mat, imgproc::COLOR_BGRA2BGR, 0)?;

    let r = detect_text(&bgr_mat);
    dbg!(r);
    Ok(bgr_mat)
}

fn detect_text(mat: &opencv::core::Mat) -> Result<Vec<opencv::core::Rect>, opencv::Error> {
    // let mut net = dnn::read_net_from_darknet(
    //     r"C:\Users\wgmlg\code\cyberpsychosis\net\yolov3-tiny.cfg",
    //     r"C:\Users\wgmlg\code\cyberpsychosis\net\yolov3-tiny.weights",
    // )?;
    // net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;
    // net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

    // // Prepare the image for DNN processing
    // let mut blob = dnn::blob_from_image(
    //     mat,
    //     1.0,
    //     core::Size::new(416, 416),
    //     core::Scalar::new(0.0, 0.0, 0.0, 0.0),
    //     true,
    //     false,
    //     core::CV_32F,
    // )?;

    // // Set the input for the neural network
    // net.set_input(&blob, "", 1.0, core::Scalar::all(0.0))?;

    // // Forward pass
    // let mut output_blobs: Vector<Mat> = Vector::new();
    // let out_blob_names = net.get_unconnected_out_layers_names()?;
    // net.forward(&mut output_blobs, &out_blob_names)?;

    // // The first output blob contains the detection
    // if output_blobs.len() == 0 {
    //     return Err(opencv::Error::new(
    //         0,
    //         "No output blobs detected".to_string(),
    //     ));
    // }
    // dbg!(&out_blob_names);
    // let detection = output_blobs.get(0).unwrap_or_else(|_| {
    //     panic!("No output blobs detected");
    // });

    // dbg!(&detection);
    // let mut boxes: Vec<opencv::core::Rect> = Vec::new();
    // let data = detection.data_typed::<f32>().unwrap();
    // let rows = detection.rows();
    // let cols = detection.cols();

    // // Assuming YOLOv3-tiny with 80 classes and 3 anchor boxes
    // let num_classes = 80;
    // let num_anchors = 3;
    // let box_attributes = 4 + 1; // 4 for bounding box, 1 for objectness score

    // for i in 0..rows {
    //     let row_data_start = i * cols;
    //     for j in 0..num_anchors {
    //         let anchor_data_start = row_data_start + j * (box_attributes + num_classes);
    //         let confidence = data.[anchor_data_start + 4]; // objectness score

    //         if confidence > 0.5 {
    //             let class_scores_start = anchor_data_start + 5; // after box_attributes
    //             let class_scores = &data[class_scores_start..class_scores_start + num_classes];

    //             // Finding the class with max score
    //             let (class_idx, &class_score) = class_scores
    //                 .iter()
    //                 .enumerate()
    //                 .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    //                 .unwrap();

    //             if class_score > 0.5 {
    //                 println!(
    //                     "Detected class: {} with confidence: {}",
    //                     class_idx, class_score
    //                 );
    //             }
    //         }
    //     }
    // }

    let mut detected_texts: Vec<String> = Vec::new();
    let mut tess = Tesseract::new(None, "eng").unwrap();

    // for box in boxes.iter() {
    //     let cropped = mat(*box)?;
    //     if let Ok(text) = tess.recognize(&cropped) {
    //         detected_texts.push(tess.get_utf8_text().unwrap());
    //     }
    // }

    Ok(boxes)
}

fn listen_for_keys(
    _interactivity_state: Arc<Mutex<bool>>,
    _hwnd: windows::Win32::Foundation::HWND,
) {
    // match listen(move |event| {
    //     dbg!(&event);
    //     if let EventType::KeyPress(key) = event.event_type {
    //         if key == Key::CapsLock {
    //             let mut state = interactivity_state.lock().unwrap();
    //             *state = !*state; // Toggle the state

    //             unsafe {
    //                 use windows::Win32::UI::WindowsAndMessaging::*;
    //                 let nindex = GWL_EXSTYLE;
    //                 let style = if *state {
    //                     WS_EX_APPWINDOW
    //                         | WS_EX_COMPOSITED
    //                         | WS_EX_LAYERED
    //                         | WS_EX_TRANSPARENT
    //                         | WS_EX_TOPMOST
    //                 } else {
    //                     WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED
    //                 };
    //                 SetWindowLongA(hwnd, nindex, style.0 as i32);
    //             };

    //             println!("Toggled interactivity to: {}", *state);
    //         }
    //     }
    // }) {
    //     Ok(_) => {}
    //     Err(error) => {
    //         println!("Error: {:?}", error);
    //     }
    // }
}

// fn callback(event: Event) {
//     // dbg!(event);
//     match event.name {
//         Some(string) => println!("User wrote {:?}", string),
//         None => (),
//     }
// }

fn main() {
    let res = capture_screenshot();
    dbg!(res);

    tauri::Builder::default()
        .setup(move |app| {
            use tauri::Manager;
            let window = app.get_window("overlay").unwrap();
            let hwnd = window.hwnd().unwrap().0;
            let hwnd = windows::Win32::Foundation::HWND(hwnd);

            // State to determine if window is currently interactive or not
            let interactivity_state = Arc::new(Mutex::new(false));

            std::thread::spawn(move || {
                listen_for_keys(interactivity_state.clone(), hwnd);
            });

            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::*;
                let nindex = GWL_EXSTYLE;
                let style = WS_EX_APPWINDOW
                    | WS_EX_COMPOSITED
                    | WS_EX_LAYERED
                    | WS_EX_TRANSPARENT
                    | WS_EX_TOPMOST;
                SetWindowLongA(hwnd, nindex, style.0 as i32);
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
