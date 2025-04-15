use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher as _;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::data::Data;
use crate::ClipboardHistory;
use std::sync::Arc;
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

type AbortHandleState<'a> =
    tauri::State<'a, Arc<TokioMutex<Option<tokio::task::JoinHandle<Vec<String>>>>>>;

#[tauri::command]
#[specta::specta]
pub async fn fuzzy_search(
    query: String,
    state: tauri::State<'_, Arc<ClipboardHistory>>, // Adding 'a lifetime
                                                    // abort_handle: AbortHandleState<'static>,
) -> Result<Vec<Data>, String> {
    // let mut old_handle = abort_handle.lock().await;

    // // Abort previous search if any
    // if let Some(handle) = old_handle.take() {
    //     handle.abort();
    // }

    // Start the search task
    let arc_state = state.inner().clone();
    let new_handle = tokio::task::spawn(async move {
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

        // Sort results by score
        let mut sorted = results;
        sorted.sort_by(|a, b| b.0.cmp(&a.0));

        sorted
            .into_iter()
            .map(|(_, val)| val)
            .collect::<Vec<Data>>()
    });

    match new_handle.await {
        Ok(vec) => Ok(vec),
        Err(err) => Err(err.to_string()),
    }
}
// #[tauri::command]
// #[specta::specta]
// pub async fn fuzzy_search_by_text(
//     query: String,
//     state: tauri::State<'static, Arc<ClipboardHistory>>, // Adding 'a lifetime
//     abort_handle: AbortHandleState<'static>,             // Adding 'a lifetime
// ) -> Result<Vec<String>, ()> {
//     // Await the results from the fuzzy search task
//     fuzzy_search_healper(query, state, abort_handle);
//     let new_handle = abort_handle.lock().await;
//     if let Some(vec) = new_handle.as_ref() {
//         let x = (*vec).await;
//         todo!()
//     } else {
//         Err(())
//     }
// }

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
