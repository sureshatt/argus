use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
};

use lru::LruCache;
use tauri::{AppHandle, Emitter, Listener};

use super::log_entry::BasicLogEntry;
use super::mac_lookup::lookup_vendor;
use super::mac_lookup::guess_device_type;

pub(crate) fn publish_stats(
    app_handle: &AppHandle,
    basic_logs_store_arc: &Arc<RwLock<VecDeque<BasicLogEntry>>>,
    detailed_logs_store_arc: &Arc<RwLock<LruCache<u32, String>>>,
) {
    
    println!("Listening to publish_stats events...");

    let app_handle_ref = Arc::new(Mutex::new(app_handle.clone()));

    app_handle.listen("publish_stats", {
        let basic_logs_store_ref_0 = Arc::clone(&basic_logs_store_arc);
        let detailed_logs_store_ref_0 = Arc::clone(&detailed_logs_store_arc);

        move |_event| {
            let basic_logs_store_ref = Arc::clone(&basic_logs_store_ref_0);
            let detailed_logs_store_ref = Arc::clone(&detailed_logs_store_ref_0);
            let app_handle_ref = Arc::clone(&app_handle_ref);

            tauri::async_runtime::spawn(async move {
                let basic_logs_store = basic_logs_store_ref.read().unwrap();
                let detailed_logs_store = detailed_logs_store_ref.read().unwrap();
                let app_handle = app_handle_ref.lock().unwrap();
                
                get_stats(&basic_logs_store);

                app_handle.emit(
                    "stats",
                    serde_json::json!({
                        "basic_logs": basic_logs_store.len(),
                        "detailed_logs": detailed_logs_store.len(),
                    }),
                ).unwrap();
                
            });
        }
    });
}


fn get_stats(basic_logs_store: &VecDeque<BasicLogEntry>) {
    println!("Getting stats...");

    if basic_logs_store.is_empty() {
        println!("No logs available.");
        return;
    }

    let mut protocol_stats: HashMap<String, usize> = HashMap::new();
    let mut source_ip_count_map: HashMap<String, usize> = HashMap::new();
    let mut destination_ip_count_map: HashMap<String, usize> = HashMap::new();

    for log_entry in basic_logs_store.iter() {

        let protocol = &log_entry.protocol;
        let source = log_entry.source.clone();
        let destination = log_entry.destination.clone();

        *protocol_stats.entry(protocol.clone()).or_insert(0) += 1;

        if protocol != "ARP" && protocol != "ICMP" && protocol != "ICMPv6" && protocol != "Ethernet" {
            *source_ip_count_map.entry(source).or_insert(0) += 1;
            *destination_ip_count_map.entry(destination).or_insert(0) += 1;
        }

        if protocol == "ARP" {
            // Handle ARP protocol separately
        }
    }

    let mut source_ip_counts: Vec<_> = source_ip_count_map.into_iter().collect();
    source_ip_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_source_ip_counts: Vec<(String, usize)> = source_ip_counts.into_iter().take(10).collect();

    let mut destination_ip_counts: Vec<_> = destination_ip_count_map.into_iter().collect();
    destination_ip_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_destination_ip_counts: Vec<(String, usize)> = destination_ip_counts.into_iter().take(10).collect();


    println!("Protocol stats: {:?}", protocol_stats);
    println!("Source count map: {:?}", top_source_ip_counts);
    println!("Destination count map: {:?}", top_destination_ip_counts);
}