
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use lru::LruCache;
use tauri::{AppHandle, Emitter, Listener};
use crate::storage::log_entry::BasicLogEntry;


pub(crate) fn listen_to_event(app_handle: &AppHandle, basic_logs_store_arc: &Arc<RwLock<VecDeque<BasicLogEntry>>>, detailed_logs_store_arc: &Arc<RwLock<LruCache<u32, String>>>, max_number_of_logs: usize) {

    println!("Listening to log events...");

    let app_handle_ref = Arc::new(Mutex::new(app_handle.clone()));
    
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
                        
                       BasicLogEntry::try_from(json_payload.clone())
                            .map(|log_entry| {

                                let mut basic_logs_store = basic_logs_store_ref.write().unwrap();
                                let mut detailed_logs_store = detailed_logs_store_ref.write().unwrap();

                                let current_number_of_logs = basic_logs_store.len();
                                if current_number_of_logs == max_number_of_logs {
                                    basic_logs_store.pop_front();
                                }
                                basic_logs_store.push_back(log_entry.clone());
                                detailed_logs_store.put(log_entry.npid.parse::<u32>().unwrap(), json_payload.to_string());

                                publish_log_entry(&app_handle_ref.lock().unwrap(), &log_entry);


                            })
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to parse log entry: {}", e);
                            });

                        
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON payload: {}", e);
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





