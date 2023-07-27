// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;

use std::thread;
use once_cell::sync::Lazy;
use specta::collect_types;
use std::env;
use std::sync::{Arc, RwLock};
use tauri_specta::ts;

// Store where our files are served from
static SERVED_DIR: Lazy<Arc<RwLock<String>>> = Lazy::new(|| {
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .into_owned();
    Arc::new(RwLock::new(current_dir))
});

// This will store the port number
static PORT: Lazy<Arc<RwLock<Option<u16>>>> = Lazy::new(|| Arc::new(RwLock::new(Some(0))));

fn main() {
    #[cfg(debug_assertions)]
    ts::export(collect_types![select_folder], "../src/lib/bindings.ts").unwrap();

    // Run the server as long as tauri runs

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Expose a tauri command to open a folder selection dialog
#[tauri::command]
#[specta::specta]
async fn select_folder() -> Result<String, String> {
    use std::path::PathBuf;

    use tauri::api::dialog::blocking::FileDialogBuilder; // Note the updated import

    let dialog_result: Option<PathBuf> = FileDialogBuilder::new().pick_folder();

    // Return the path as a string if the user selected a folder
    match dialog_result {
        Some(path) => match path.to_str() {
            Some(path_str) => Ok(path_str.to_string()),
            None => Err("Error converting path to string".to_string()),
        },
        None => Err("No folder selected".to_string()),
    }
}
