use crate::data::Data;
/**
  returns partial string array with offset and limit
*/
#[tauri::command]
fn get_partial_list(offset: usize, limit: usize) -> Vec<Data> {
    todo!()
}

#[tauri::command]
fn delete(id: &str) {}

#[tauri::command]
fn get_full_data(id: &str) -> Option<String> {
    todo!()
}
