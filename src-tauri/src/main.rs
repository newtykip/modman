// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use modman::MODMAN_DIR;
use std::fs::create_dir_all;

mod commands;

macro_rules! load_commands {
    ($builder:expr; $($command:ident),+) => {
        // create ts bindings
        tauri_specta::ts::export(specta::collect_types![$(commands::$command,)*], "../src/lib/bindings.ts").unwrap();

        // load tauri commands
        $builder = $builder.invoke_handler(tauri::generate_handler![$(commands::$command,)*]);
    }
}

#[tokio::main]
async fn main() {
    let mut builder = tauri::Builder::default().setup(|#[allow(unused_variables)] app| {
        // apply window shadow
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            use tauri::Manager;

            let window = app.get_window("main").unwrap();
            window_shadows::set_shadow(&window, true).unwrap();
        }

        // ensure profiles directory exists
        create_dir_all(MODMAN_DIR.join("profiles"))?;

        Ok(())
    });

    load_commands!(builder; create_slug, save_profile, load_profiles, get_profile);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
