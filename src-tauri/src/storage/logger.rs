
use std::sync::{Arc, Mutex};
use log::{error, info};
use tauri::{AppHandle, Emitter, Listener, State};
use crate::storage::log_entry::BasicLogEntry;
use crate::AppState;


pub(crate) fn listen_to_event(app_handle: &AppHandle, state: State<AppState>, max_number_of_logs: usize) {

    info!("Listening to log events...");

    let app_handle_ref = Arc::new(Mutex::new(app_handle.clone()));
    let basic_logs_store_arc = Arc::clone(&state.basic_logs_store);
    let detailed_logs_store_arc = Arc::clone(&state.detailed_logs_store);
    
    app_handle.listen("all_logs_event", {

        let basic_logs_store_ref_0 = Arc::clone(&basic_logs_store_arc);
        let detailed_logs_store_ref_0 = Arc::clone(&detailed_logs_store_arc);

        move |event| {

            let basic_logs_store_ref = Arc::clone(&basic_logs_store_ref_0);
            let detailed_logs_store_ref = Arc::clone(&detailed_logs_store_ref_0);
            let app_handle_ref = Arc::clone(&app_handle_ref);

            tauri::async_runtime::spawn(async move {

                match serde_json::from_str::<serde_json::Value>(event.payload()) {
                    Ok(json_payload) => {
                        
                        match BasicLogEntry::try_from(json_payload.clone()) {
                            Ok(log_entry) => {
                                let mut basic_logs_store = match basic_logs_store_ref.write() {
                                    Ok(guard) => guard,
                                    Err(e) => {
                                        error!("Failed to acquire write lock for basic_logs_store: {}", e);
                                        return;
                                    }
                                };
                                let mut detailed_logs_store = match detailed_logs_store_ref.write() {
                                    Ok(guard) => guard,
                                    Err(e) => {
                                        error!("Failed to acquire write lock for detailed_logs_store: {}", e);
                                        return;
                                    }
                                };

                                let current_number_of_logs = basic_logs_store.len();
                                if current_number_of_logs == max_number_of_logs {
                                    basic_logs_store.pop_front();
                                }
                                basic_logs_store.push_back(log_entry.clone());
                                match log_entry.npid.parse::<u32>() {
                                    Ok(npid) => {
                                        detailed_logs_store.put(npid, json_payload.to_string());
                                    }
                                    Err(e) => {
                                        error!("Failed to parse npid as u32: {}", e);
                                    }
                                }

                                match app_handle_ref.lock() {
                                    Ok(app_handle) => {
                                        publish_log_entry(&app_handle, &log_entry);
                                    }
                                    Err(e) => {
                                        error!("Failed to acquire lock for app_handle_ref: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse log entry: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse JSON payload: {}", e);
                    }
                }
            });
        }
    });
}


fn publish_log_entry(app_handle: &AppHandle, log_entry: &BasicLogEntry) {
    let _ = app_handle.emit("update",  log_entry);
    let _ = app_handle.emit("publish_stats",  "");
}





