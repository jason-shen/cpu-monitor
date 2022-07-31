#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use perf_monitor::cpu::ProcessStat;
use std::time::Duration;
use tauri::Manager;
use tokio::time::sleep;
#[derive(Clone, serde::Serialize)]
struct Payload {
    usage: String,
}

fn main() {
    let mut stat_p = ProcessStat::cur().unwrap();
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    let usage_p = stat_p.cpu().unwrap() * 100f64;
                    sleep(Duration::from_millis(2000)).await;
                    println!("{:.2}%", usage_p);
                    app_handle
                        .emit_all(
                            "usage",
                            Payload {
                                usage: format!("{:.2}%", usage_p),
                            },
                        )
                        .unwrap();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
