use tauri::{AppHandle, Listener};
use crate::storage::log_entry::{LogEntry, parse_log_entry};


pub(crate) fn listen_to_event(app_handle: &AppHandle) {

    println!("Listening to events...");

    app_handle.listen("update", move |event| {
        
        tauri::async_runtime::spawn(async move {
            match serde_json::from_str::<serde_json::Value>(event.payload()) {
                Ok(json_payload) => {
                    parse_log_entry(json_payload).map(|log_entry| {
                        // Here you can handle the log entry, e.g., save it to a database or file
                        println!("Parsed log entry: {:?}", log_entry);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON payload: {}", e);
                }
            }
        });
    });
}