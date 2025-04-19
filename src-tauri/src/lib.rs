use arboard::Clipboard;
use commands::{
    delete_by_id, fuzzy_search, get_all_data, get_all_id, get_by_id, get_pool_clipboard_state,
    set_pool_clipboard_state, update_data_by_id,
};
use common::events::POOL_CLIPBOARD_UPDATED;
use specta_typescript::{BigIntExportBehavior, Typescript};
use std::{
    sync::{Arc, RwLock},
    thread,
};
use tauri::{Emitter as _, Manager as _};
use tauri_specta::{collect_commands, Builder};
mod commands;
mod common;
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
//-------------------------- STATE -------------------------------------
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
pub struct PoolClipboard {
    pub value: bool,
    pub app_handle: tauri::AppHandle,
}

impl PoolClipboard {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            value: true,
            app_handle,
        }
    }

    pub fn set(&mut self, new_value: bool) {
        self.value = new_value;
        let _ = self.app_handle.emit(POOL_CLIPBOARD_UPDATED, self.value);
    }

    pub fn get(&self) -> bool {
        self.value
    }
}
//-------------------------- APP -------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            health,
            get_all_id,
            get_by_id,
            get_all_data,
            delete_by_id,
            update_data_by_id,
            get_pool_clipboard_state,
            set_pool_clipboard_state,
            fuzzy_search,
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(
            Typescript::default().bigint(BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // .manage(history)
        // .manage(pool_clipboard)
        .plugin(tauri_plugin_opener::init())
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            let history = Arc::new(ClipboardHistory::new());
            let history_clone = Arc::clone(&history);
            app.manage(history);
            let app_handle = app.handle();
            let pool_clipboard = Arc::new(RwLock::new(PoolClipboard::new(app_handle.clone())));
            let pool_clipboard_clone = Arc::clone(&pool_clipboard);

            thread::spawn(move || {
                let mut clipboard = Clipboard::new().expect("Failed to access Clipboard");
                let mut last_value = String::new();
                loop {
                    let pool_clipboard = pool_clipboard_clone.read().unwrap();
                    if (*pool_clipboard).get() {
                        drop(pool_clipboard);
                        if let Ok(current) = clipboard.get_text() {
                            if current != last_value {
                                let mut lru = history_clone.data.write().unwrap();
                                lru.insert(current.clone());
                                last_value = current;
                            }
                        }
                    } else {
                        drop(pool_clipboard);
                    }
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            builder.mount_events(app);
            app.manage(pool_clipboard);
            Ok(())
        })
        // on an actual app, remove the string argument
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
