use std::{
    collections::{HashMap, VecDeque}, sync::{Arc, Mutex}
};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter, Listener, State};
use crate::AppState;

use super::log_entry::BasicLogEntry;

#[derive(Serialize, Deserialize, Debug)]
struct IpStat {
    ip: String,
    count: usize,
}
impl IpStat {
    fn new(ip: String, count: usize) -> Self {
        IpStat { ip, count }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ArpStat {
    source_ip: String,
    source_mac: String,
}
impl ArpStat {
    fn new(source_ip: String, source_mac: String) -> Self {
        ArpStat { source_ip, source_mac }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ProtocolStat {
    protocol: String,
    count: usize,
}
impl ProtocolStat {
    fn new(protocol: String, count: usize) -> Self {
        ProtocolStat { protocol, count }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NetworkStat {
    protocol_stats: Vec<ProtocolStat>,
    ingress_ip_stats: Vec<IpStat>,
    egress_ip_stats: Vec<IpStat>,
    arp_stats: Vec<ArpStat>,
}
impl NetworkStat {
    fn new(
        protocol_stats: Vec<ProtocolStat>,
        ingress_ip_stats: Vec<IpStat>,
        egress_ip_stats: Vec<IpStat>,
        arp_stats: Vec<ArpStat>,
    ) -> Self {
        NetworkStat {
            protocol_stats,
            ingress_ip_stats,
            egress_ip_stats,
            arp_stats,
        }
    }
}


pub(crate) fn publish_stats(
    app_handle: &AppHandle,
    state: State<AppState>,
) {
    
    println!("Listening to publish_stats events...");

    let app_handle_ref = Arc::new(Mutex::new(app_handle.clone()));
    let basic_logs_store_arc = Arc::clone(&state.basic_logs_store);
    let detailed_logs_store_arc = Arc::clone(&state.detailed_logs_store);

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
                
                let stats = get_stats(&basic_logs_store, &detailed_logs_store);
                println!("Stats: {:?}", stats);

                app_handle.emit(
                    "stats",
                    stats,
                ).unwrap();
                
            });
        }
    });
}


fn get_stats(basic_logs_store: &VecDeque<BasicLogEntry>, detailed_logs_store: &LruCache<u32, String>) -> serde_json::Value {
    println!("Getting stats...");

    if basic_logs_store.is_empty() {
        println!("No logs available.");
         return json!({
            "protocol_stats": {},
            "local_devices_list": {},
            "top_source_ip_counts": [],
            "top_destination_ip_counts": []
        });
    }

    let mut protocol_stats: HashMap<String, usize> = HashMap::new();
    let mut source_ip_stat_map: HashMap<String, usize> = HashMap::new();
    let mut destination_ip_stat_map: HashMap<String, usize> = HashMap::new();
    let mut arp_stats_map: HashMap<String, String> = HashMap::new();

    for log_entry in basic_logs_store.iter() {

        let protocol = &log_entry.protocol;
        let source = log_entry.source.clone();
        let destination = log_entry.destination.clone();

        *protocol_stats.entry(protocol.clone()).or_insert(0) += 1;

        if protocol != "ARP" && protocol != "ICMP" && protocol != "ICMPv6" && protocol != "Ethernet" {
            *source_ip_stat_map.entry(source).or_insert(0) += 1;
            *destination_ip_stat_map.entry(destination).or_insert(0) += 1;
        }

        if protocol == "ARP" {
            let npid = log_entry.npid.parse::<u32>();
            if let Some(json_str) = detailed_logs_store.peek(&npid.unwrap()) {
                let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
                let source_mac = json_value["sender_hw_addr"].as_str().unwrap_or("");
                let source_ip = json_value["sender_proto_addr"].as_str().unwrap_or("");

                arp_stats_map.insert(source_ip.to_string(), source_mac.to_string());
            }

        }
    }

    let protocol_stats: Vec<ProtocolStat> = protocol_stats
        .into_iter()
        .map(|(protocol, count)| ProtocolStat::new(protocol, count))
        .collect();
    
    let mut source_ip_counts: Vec<_> = source_ip_stat_map.into_iter().collect();
    source_ip_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_source_ip_counts: Vec<IpStat> = source_ip_counts
    .into_iter()
    .take(10)
    .map(|(ip, count)| IpStat::new(ip, count))
    .collect();

    let mut destination_ip_counts: Vec<_> = destination_ip_stat_map.into_iter().collect();
    destination_ip_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_destination_ip_counts: Vec<IpStat> = destination_ip_counts
    .into_iter()
    .take(10)
    .map(|(ip, count)| IpStat::new(ip, count))
    .collect();

    let arp_stats: Vec<ArpStat> = arp_stats_map
        .iter()
        .map(|(source_ip, source_mac)| ArpStat::new(source_ip.clone(), source_mac.clone()))
        .collect();

    let network_stats = NetworkStat::new(
        protocol_stats,
        top_source_ip_counts,
        top_destination_ip_counts,
        arp_stats
    );

    serde_json::to_value(network_stats).unwrap()

}

#[tauri::command]
pub(crate) fn get_packet_data(state: State<AppState>, npid: String) -> Result<Vec<serde_json::Value>, String> {
    
    let npid_s = npid.parse::<u32>().unwrap();
    let detailed_logs_store = state.detailed_logs_store.read().unwrap();

    if let Some(json_str) = detailed_logs_store.peek(&npid_s) {
        let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let mut packet_data = Vec::new();
        packet_data.push(json_value.clone());
        Ok(packet_data)

    } else {
        return Err("No detailed log found".to_string());
    }
}