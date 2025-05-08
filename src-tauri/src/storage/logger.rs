
use std::collections::VecDeque;
use tauri::{AppHandle, Emitter, Listener};
use crate::storage::log_entry::BasicLogEntry;


pub(crate) fn listen_to_event(app_handle: &AppHandle, basic_logs_store: &VecDeque<BasicLogEntry>, detailed_logs_store: &lru::LruCache<u32, String>, max_number_of_logs: usize) {

    println!("Listening to events...");

    let basic_logs_store_ref = std::sync::Arc::new(std::sync::Mutex::new(basic_logs_store.clone()));
    let detailed_logs_store_ref = std::sync::Arc::new(std::sync::Mutex::new(detailed_logs_store.clone()));
    let app_handle_ref = std::sync::Arc::new(std::sync::Mutex::new(app_handle.clone()));
    
    app_handle.listen("all_logs_event", {

        let basic_logs_store_ref = std::sync::Arc::clone(&basic_logs_store_ref);
        let detailed_logs_store_ref = std::sync::Arc::clone(&detailed_logs_store_ref);


        move |event| {

            let basic_logs_store_ref = std::sync::Arc::clone(&basic_logs_store_ref);
            let detailed_logs_store_ref = std::sync::Arc::clone(&detailed_logs_store_ref);
            let app_handle_ref = std::sync::Arc::clone(&app_handle_ref);

            tauri::async_runtime::spawn(async move {

                match serde_json::from_str::<serde_json::Value>(event.payload()) {
                    Ok(json_payload) => {
                        
                       BasicLogEntry::try_from(json_payload.clone())
                            .map(|log_entry| {

                                let mut basic_logs_store = basic_logs_store_ref.lock().unwrap();
                                let mut detailed_logs_store = detailed_logs_store_ref.lock().unwrap();

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

}

