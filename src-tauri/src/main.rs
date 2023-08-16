// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::*;
use modman::MODMAN_DIR;
use specta::collect_types;
use std::fs;
use tauri::generate_handler;
use tauri_specta::ts;

mod commands;

#[tokio::main]
async fn main() {
    ts::export(
        collect_types![save_profile, load_profiles],
        "../src/lib/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .setup(|#[allow(unused_variables)] app| {
            // apply window shadow
            #[cfg(any(target_os = "windows", target_os = "macos"))]
            {
                use tauri::Manager;

                let window = app.get_window("main").unwrap();
                window_shadows::set_shadow(&window, true).unwrap();
            }

            // ensure profiles directory exists
            fs::create_dir_all(MODMAN_DIR.join("profiles"))?;

            Ok(())
        })
        .invoke_handler(generate_handler![save_profile, load_profiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
