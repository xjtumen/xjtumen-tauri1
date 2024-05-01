// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};

use tauri::{Manager, Url};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
            .setup(|app| {
                // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
                let (mut rx, mut child) = Command::new_sidecar("xmentccnbg")
                        .expect("failed to create `xmentccnbg` binary command")
                        .spawn()
                        .expect("Failed to spawn sidecar");
                let main_window = app.get_window("main").unwrap();
                tauri::async_runtime::spawn(async move {
                    // read events such as stdout
                    while let Some(event) = rx.recv().await {
                        if let CommandEvent::Stdout(line) = event {
                            main_window
                                    .emit("message", Some(format!("'{}'", line)))
                                    .expect("failed to emit event");
                            // write to stdin
                            child.write("message from Rust\n".as_bytes()).unwrap();
                        }
                    }
                });
                let window = tauri::WindowBuilder::new(app, "label",
                                                       tauri::WindowUrl::External("https://xjtu.men/".parse().unwrap()))
                        .build()?;
                let url = Url::parse("https://xjtu.men:443")?;

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
