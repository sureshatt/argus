
use std::collections::VecDeque;

use tauri::{AppHandle, Emitter, Listener};
use crate::storage::log_entry::{parse_log_entry, LogEntry, BasicLogEntry};


pub(crate) fn listen_to_event(app_handle: &AppHandle, logs_store: &VecDeque<LogEntry>) {

    println!("Listening to events...");

    let logs_store_ref = std::sync::Arc::new(std::sync::Mutex::new(logs_store.clone()));
    let app_handle_ref = std::sync::Arc::new(std::sync::Mutex::new(app_handle.clone()));
    
    app_handle.listen("all_logs_event", {
        let logs_store_ref = std::sync::Arc::clone(&logs_store_ref);
        move |event| {

            let logs_store_ref = std::sync::Arc::clone(&logs_store_ref);
            let app_handle_ref = std::sync::Arc::clone(&app_handle_ref);

            tauri::async_runtime::spawn(async move {

                match serde_json::from_str::<serde_json::Value>(event.payload()) {
                    Ok(json_payload) => {
                        if let Some(log_entry) = parse_log_entry(json_payload) {


                            let mut logs_store = logs_store_ref.lock().unwrap();
                            logs_store.push_back(log_entry.clone());
                            publish_log_entry(&app_handle_ref.lock().unwrap(), &log_entry);
                            println!("Log entry added: {:?}", log_entry);

                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON payload: {}", e);
                    }
                }
                println!("Number of logs: {}", logs_store_ref.lock().unwrap().len());
            });
        }
    });
}

fn publish_log_entry(app_handle: &AppHandle, log_entry: &LogEntry) {
    let _ = app_handle.emit("update",  BasicLogEntry::new(log_entry));
}