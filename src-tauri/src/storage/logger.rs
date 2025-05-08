use tauri::{AppHandle, Listener};


pub(crate) fn listen_to_event(app_handle: &AppHandle) {

    app_handle.listen("update", move |event| {
        

        tauri::async_runtime::spawn(async move {
            match serde_json::from_str::<serde_json::Value>(event.payload()) {
                Ok(json_payload) => {
                    // use VecDeque to store the logs
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON payload: {}", e);
                }
            }
        });
    });
}