use std::sync::{Arc, Mutex};
use tauri::{Manager as _, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[derive(Default)]
pub struct PopupWindow {
    pub webview_handle: Arc<Mutex<Option<WebviewWindow>>>,
}
pub fn popup_show(app_handle: tauri::AppHandle) -> Result<(), String> {
    let state = app_handle.state::<PopupWindow>();

    let webwiew_handle = state.webview_handle.clone();

    let handle_lock = webwiew_handle
        .lock()
        .map_err(|_| "mutex err".to_string())?
        .clone(); // Clone the Arc

    match handle_lock {
        Some(webview) => webview.show().unwrap(),
        None => {
            std::thread::spawn(move || {
                let app_handle_clone = app_handle.clone();
                let webview_window = WebviewWindowBuilder::new(
                    &app_handle_clone,
                    "label",
                    WebviewUrl::App("popup.html".into()),
                )
                .inner_size(300., 900.)
                .always_on_top(true) // Ensures it stays on top
                .focused(true)
                .decorations(false)
                .build()
                .map_err(|err| {
                    println!("fskdfas kkk{}", err.to_string());
                    err.to_string()
                })
                .unwrap();
                webview_window.show().unwrap();
                {
                    let mut handle_guard = webwiew_handle
                        .lock()
                        .map_err(|err| err.to_string())
                        .unwrap();
                    *handle_guard = Some(webview_window.clone());
                }
                ()
            });
        }
    }
    Ok(())
}

pub fn key_listner() {}
