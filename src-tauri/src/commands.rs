use serde::{Deserialize, Serialize};

use crate::data::Data;
use crate::ClipboardHistory;
use std::sync::Arc;

#[tauri::command]
fn get_all_id(
    offset: usize,
    limit: usize,
    state: tauri::State<Arc<ClipboardHistory>>,
) -> Vec<String> {
    let map = state.data.lock().unwrap();
    map.list()
        .iter()
        .skip(offset)
        .take(limit)
        .map(|data| data.hash())
        .collect()
}

#[tauri::command]
fn get_by_id(id: &str, state: tauri::State<Arc<ClipboardHistory>>) -> Option<Data> {
    let map = state.data.lock().unwrap();
    map.get(id)
}
/**
  returns partial string array with offset and limit
*/
#[tauri::command]
fn get_partial_list(offset: usize, limit: usize) -> Vec<Data> {
    todo!()
}

#[tauri::command]
fn delete(id: &str) {}

#[derive(Deserialize, Serialize)]
struct AllData {
    pub id: String,
    pub data: Data,
}
#[tauri::command]
fn get_all_data(
    offset: usize,
    limit: usize,
    state: tauri::State<Arc<ClipboardHistory>>,
) -> Vec<AllData> {
    let map = state.data.lock().unwrap();
    map.list()
        .iter()
        .skip(offset)
        .take(limit)
        .map(|data| AllData {
            id: data.hash(),
            data,
        })
        .collect()
}
