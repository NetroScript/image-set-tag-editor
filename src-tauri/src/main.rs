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
    ts::export(collect_types![select_folder, get_served_dir, get_available_files], "../src/lib/bindings.ts").unwrap();

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
        .invoke_handler(tauri::generate_handler![select_folder, get_served_dir, get_available_files])
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
    let path = match dialog_result {
        Some(path) => match path.to_str() {
            Some(path_str) => Ok(path_str.to_string()),
            None => Err("Error converting path to string".to_string()),
        },
        None => Err("No folder selected".to_string()),
    };

    // Save the path to the global variable
    if let Ok(path) = &path {

        let mut served_dir_write = SERVED_DIR.write().unwrap();
        *served_dir_write = path.clone();

        // Now remove the lock
        drop(served_dir_write);

        // Print out the port number and the currently served absolute path to the console
        println!(
            "Serving files from {} on port {}",
            SERVED_DIR.read().unwrap(),
            PORT.read().unwrap().unwrap()
        );
    } else {
        println!("No folder selected");
    }

    path
}


// Expose a command to return the currently served directory, and the port number
#[tauri::command]
#[specta::specta]
async fn get_served_dir() -> Result<(String, u16), String> {
    let port = PORT.read().unwrap().unwrap();
    let served_dir = SERVED_DIR.read().unwrap().clone();

    Ok((served_dir, port))
}

// Expose a command to obtain all files available in the currently served directory
// Directory traversal is recursive, with a maximum file count of 10000
#[tauri::command]
#[specta::specta]
async fn get_available_files() -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::PathBuf;

    let served_dir = SERVED_DIR.read().unwrap().clone();

    let mut files: Vec<String> = Vec::new();

    // Recursively traverse the directory, and add all files to the vector
    let mut dir_queue: Vec<PathBuf> = Vec::new();
    dir_queue.push(PathBuf::from(served_dir.clone()));

    while let Some(dir) = dir_queue.pop() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        dir_queue.push(path);
                    } else if let Some(path_str) = path.to_str() {

                        // Only store the part of the path that is relative to the served directory
                        let path_str = path_str.replace(&served_dir, "");
                        // Also remove the leading slash (can also be a backslash on Windows)
                        let path_str = path_str.trim_start_matches('/').trim_start_matches('\\');

                        files.push(path_str.to_string());

                        // Limit the file count to 10000
                        if files.len() > 10000 {
                            return Ok(files);
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}
