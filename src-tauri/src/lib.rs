use std::{
    sync::{Arc, RwLock},
    thread,
};

use arboard::Clipboard;
use commands::{
    delete_by_id, fuzzy_search, get_all_data, get_all_id, get_by_id, update_data_by_id,
};
use lru::Lru;
use specta_typescript::{BigIntExportBehavior, Typescript};
use tauri_specta::{collect_commands, Builder};
mod commands;
mod data;
mod double_linked_list;
mod double_linked_list_multi_thread;
mod lru;
mod lru_multi_thread;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn health(slug: &str) -> String {
    format!("Hello, {}! I'm healthy", slug)
}

struct ClipboardHistory {
    pub data: RwLock<lru_multi_thread::Lru>,
}
impl ClipboardHistory {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(lru_multi_thread::Lru::new(1000)),
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
                    let mut lru = history_clone.data.write().unwrap();
                    lru.insert(current.clone());
                    last_value = current;
                }
            }
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // tauri::Builder::default()
    //     .manage(history)
    //     .plugin(tauri_plugin_opener::init())
    //     .invoke_handler(tauri::generate_handler![health])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let mut builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            health,
            get_all_id,
            get_by_id,
            get_all_data,
            delete_by_id,
            update_data_by_id,
            fuzzy_search
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().bigint(BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .manage(history)
        .plugin(tauri_plugin_opener::init())
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            builder.mount_events(app);

            Ok(())
        })
        // on an actual app, remove the string argument
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
