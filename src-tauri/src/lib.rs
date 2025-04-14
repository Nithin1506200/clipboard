use std::{
    sync::{Arc, Mutex},
    thread,
};

use arboard::Clipboard;
use lru::Lru;

mod commands;
mod data;
mod double_linked_list;
mod double_linked_list_multi_thread;
mod lru;
mod lru_multi_thread;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn health(slug: &str) -> String {
    format!("Hello, {}! I'm healthy", slug)
}

struct ClipboardHistory {
    pub data: Mutex<lru_multi_thread::Lru>,
}
impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(lru_multi_thread::Lru::new(1000)),
        }
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let history = Arc::new(ClipboardHistory::new());
    let history_clone = Arc::clone(&history);

    thread::spawn(move || {
        let mut clipboard = Clipboard::new().expect("Failed to access Clipboard");
        let mut last_value = String::new();

        loop {
            if let Ok(current) = clipboard.get_text() {
                if current != last_value {
                    let mut lru = history_clone.data.lock().unwrap();
                    lru.insert(current.clone());
                    last_value = current;
                }
            }
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    tauri::Builder::default()
        .manage(history)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![health])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
