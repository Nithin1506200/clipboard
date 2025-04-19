use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher as _;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::data::Data;
use crate::{ClipboardHistory, PoolClipboard};
use std::sync::{Arc, RwLock};
use std::vec;
use tokio::sync::Mutex as TokioMutex;
#[tauri::command]
#[specta::specta]
pub fn get_all_id(
    offset: u64,
    limit: u64,
    state: tauri::State<Arc<ClipboardHistory>>,
) -> Vec<String> {
    let lru = state.data.read().unwrap();
    lru.list()
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|data| data.hash())
        .collect()
}

#[tauri::command]
#[specta::specta]
pub fn get_by_id(id: &str, state: tauri::State<Arc<ClipboardHistory>>) -> Option<Data> {
    let map = state.data.read().unwrap();
    map.get(id)
}

#[tauri::command]
#[specta::specta]
pub fn delete_by_id(id: &str, state: tauri::State<Arc<ClipboardHistory>>) -> Result<(), ()> {
    let mut lru = state.data.write().unwrap();
    lru.delete(id)
}

#[derive(Deserialize, Serialize, Type)]
pub struct AllData {
    pub id: String,
    pub data: Data,
}
#[tauri::command]
#[specta::specta]
pub fn get_all_data(
    offset: u64,
    limit: u64,
    state: tauri::State<Arc<ClipboardHistory>>,
) -> Vec<AllData> {
    let map = state.data.read().unwrap();
    map.list()
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .map(|data| AllData {
            id: data.hash(),
            data,
        })
        .collect()
}

// ----------------------- FUZZY SEARCH --------------------- //
#[tauri::command]
#[specta::specta]
pub async fn fuzzy_search(
    query: String,
    state: tauri::State<'_, Arc<ClipboardHistory>>,
) -> Result<Vec<Data>, String> {
    let arc_state = state.inner().clone();
    let new_handle = std::thread::spawn(move || {
        // use std thread
        let matcher = SkimMatcherV2::default();
        let lru = arc_state.data.read().expect("lru unlocked".into());
        // Perform fuzzy search on the data stored in your hashmap
        let results: Vec<_> = lru
            .list()
            .iter()
            .filter_map(|data| {
                matcher
                    .fuzzy_match(&data.val(), &query)
                    .map(|score| (score, data))
            })
            .collect();
        let mut sorted = results;
        sorted.sort_by(|a, b| b.0.cmp(&a.0));
        sorted
            .into_iter()
            .map(|(_, val)| val)
            .collect::<Vec<Data>>()
    });
    match new_handle.join() {
        Ok(vec) => Ok(vec),
        Err(err) => Err(format!("Thread panicked: {:?}", err)),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_data_by_id(
    id: &str,
    new_data: Data,
    state: tauri::State<'_, Arc<ClipboardHistory>>,
) -> Result<(), ()> {
    let lru = state.data.write().unwrap();
    if let Some(data) = lru.get_mutex(id) {
        let mut data = data.lock().unwrap();
        (*data).set_value(new_data);
        Ok(())
    } else {
        Err(())
    }
}

#[tauri::command]
#[specta::specta]
pub fn get_pool_clipboard_state(state: tauri::State<Arc<RwLock<PoolClipboard>>>) -> bool {
    let pool_clipboard = state.read().unwrap();
    pool_clipboard.get()
}

#[tauri::command]
#[specta::specta]
pub fn set_pool_clipboard_state(state: tauri::State<Arc<RwLock<PoolClipboard>>>, value: bool) {
    let mut pool_clipboard = state.write().unwrap();
    pool_clipboard.set(value);
}
