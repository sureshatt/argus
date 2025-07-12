use super::log_entry::BasicLogEntry;
use crate::network::ip_utils::{LOCAL_ORIGIN, UNKNOWN_ORIGIN};
use crate::{network::network_interface::get_net_iface_by_name, AppState};
use log::{error, info, warn};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter, Listener, State};

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
struct CountryStat {
    country: String,
}
impl CountryStat {
    fn new(country: String) -> Self {
        CountryStat { country }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ArpStat {
    source_ip: String,
    source_mac: String,
}
impl ArpStat {
    fn new(source_ip: String, source_mac: String) -> Self {
        ArpStat {
            source_ip,
            source_mac,
        }
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
    country_stats: Vec<CountryStat>,
    arp_stats: Vec<ArpStat>,
}
impl NetworkStat {
    fn new(
        protocol_stats: Vec<ProtocolStat>,
        ingress_ip_stats: Vec<IpStat>,
        egress_ip_stats: Vec<IpStat>,
        country_stats: Vec<CountryStat>,
        arp_stats: Vec<ArpStat>,
    ) -> Self {
        NetworkStat {
            protocol_stats,
            ingress_ip_stats,
            egress_ip_stats,
            country_stats,
            arp_stats,
        }
    }
}

pub(crate) fn publish_stats(app_handle: &AppHandle, state: State<AppState>) {
    info!("Listening to publish_stats events...");

    let app_handle_ref = Arc::new(Mutex::new(app_handle.clone()));
    let basic_logs_store_arc = Arc::clone(&state.basic_logs_store);
    let detailed_logs_store_arc = Arc::clone(&state.detailed_logs_store);
    let network_interface_arc = Arc::clone(&state.selected);

    app_handle.listen("publish_stats", {
        let basic_logs_store_ref_0 = Arc::clone(&basic_logs_store_arc);
        let detailed_logs_store_ref_0 = Arc::clone(&detailed_logs_store_arc);
        let network_interface_ref_0 = Arc::clone(&network_interface_arc);

        move |_event| {
            let basic_logs_store_ref = Arc::clone(&basic_logs_store_ref_0);
            let detailed_logs_store_ref = Arc::clone(&detailed_logs_store_ref_0);
            let app_handle_ref = Arc::clone(&app_handle_ref);
            let network_interface_ref = Arc::clone(&network_interface_ref_0);

            tauri::async_runtime::spawn(async move {
                let basic_logs_store = match basic_logs_store_ref.read() {
                    Ok(store) => store,
                    Err(e) => {
                        error!("Failed to acquire read lock on basic_logs_store: {}", e);
                        return;
                    }
                };
                let detailed_logs_store = match detailed_logs_store_ref.read() {
                    Ok(store) => store,
                    Err(e) => {
                        error!("Failed to acquire read lock on detailed_logs_store: {}", e);
                        return;
                    }
                };
                let app_handle = match app_handle_ref.lock() {
                    Ok(handle) => handle,
                    Err(e) => {
                        error!("Failed to acquire lock on app_handle: {}", e);
                        return;
                    }
                };
                let network_interface = match network_interface_ref.read() {
                    Ok(iface) => iface,
                    Err(e) => {
                        error!("Failed to acquire read lock on network_interface: {}", e);
                        return;
                    }
                };

                let stats =
                    match get_stats(&basic_logs_store, &detailed_logs_store, &network_interface) {
                        Ok(stats) => stats,
                        Err(e) => {
                            error!("Failed to get stats: {}", e);
                            return;
                        }
                    };

                let _ = match app_handle.emit("stats", stats) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Failed to emit stats event: {}", e);
                    }
                };
            });
        }
    });
}

fn get_stats(
    basic_logs_store: &VecDeque<BasicLogEntry>,
    detailed_logs_store: &LruCache<u32, String>,
    network_interface_str: &String,
) -> Result<serde_json::Value, String> {
    let empty_stats = json!({
        "protocol_stats": {},
        "local_devices_list": {},
        "top_source_ip_counts": [],
        "top_destination_ip_counts": []
    });

    if basic_logs_store.is_empty() {
        warn!("No logs available.");
        return Ok(empty_stats);
    }

    let network_interface = get_net_iface_by_name(network_interface_str);
    let netcard_mac = match network_interface {
        Some(iface) => iface.mac.clone(),
        None => {
            error!("Network interface not found");
            return Err("Network interface not found".into());
        }
    };

    let mut protocol_stats_map: HashMap<String, usize> = HashMap::new();
    let mut ingress_ip_stat_map: HashMap<String, usize> = HashMap::new();
    let mut egress_ip_stat_map: HashMap<String, usize> = HashMap::new();
    let mut country_stats_map: HashMap<String, usize> = HashMap::new();
    let mut arp_stats_map: HashMap<String, String> = HashMap::new();

    for log_entry in basic_logs_store.iter() {
        let protocol = &log_entry.protocol;
        let source = log_entry.source.clone();
        let destination = log_entry.destination.clone();

        let npid_parsed = match log_entry.npid.parse::<u32>() {
            Ok(npid) => npid,
            Err(e) => {
                error!("Failed to parse npid for log entry: {}", e);
                continue;
            }
        };

        *protocol_stats_map.entry(protocol.clone()).or_insert(0) += 1;

        if protocol == "IPv4" || protocol == "IPv6" {
            if let Some(json_str) = detailed_logs_store.peek(&npid_parsed) {
                let json_value: serde_json::Value = match serde_json::from_str(json_str) {
                    Ok(value) => value,
                    Err(e) => {
                        error!("Failed to parse JSON for npid {}: {}", npid_parsed, e);
                        continue;
                    }
                };

                let source_origin = match json_value["source_ip_origin"].as_str() {
                    Some(origin) => origin,
                    None => {
                        error!("Failed to get source_ip_origin for npid {}", npid_parsed);
                        continue;
                    }
                };
                let destination_origin = match json_value["destination_ip_origin"].as_str() {
                    Some(origin) => origin,
                    None => {
                        error!(
                            "Failed to get destination_ip_origin for npid {}",
                            npid_parsed
                        );
                        continue;
                    }
                };

                // ignore the local IPs for IP stats
                if source_origin != LOCAL_ORIGIN {
                    *ingress_ip_stat_map.entry(source).or_insert(0) += 1;

                    if source_origin.to_lowercase() != UNKNOWN_ORIGIN
                        && source_origin.to_lowercase() != "zz"
                    {
                        *country_stats_map
                            .entry(source_origin.to_string())
                            .or_insert(0) += 1;
                    }
                }
                if destination_origin != LOCAL_ORIGIN {
                    *egress_ip_stat_map.entry(destination).or_insert(0) += 1;

                    if destination_origin.to_lowercase() != UNKNOWN_ORIGIN
                        && destination_origin.to_lowercase() != "zz"
                    {
                        *country_stats_map
                            .entry(destination_origin.to_string())
                            .or_insert(0) += 1;
                    }
                }
            }
        }

        if protocol == "ARP" {
            if let Some(json_str) = detailed_logs_store.peek(&npid_parsed) {
                let json_value: serde_json::Value = match serde_json::from_str(json_str) {
                    Ok(value) => value,
                    Err(e) => {
                        error!("Failed to parse JSON for ARP npid {}: {}", npid_parsed, e);
                        continue;
                    }
                };
                let source_mac = match json_value["sender_hw_addr"].as_str() {
                    Some(mac) => mac,
                    None => {
                        error!("Failed to get sender_hw_addr for ARP npid {}", npid_parsed);
                        continue;
                    }
                };

                if netcard_mac == source_mac {
                    continue;
                }
                let source_ip = match json_value["sender_proto_addr"].as_str() {
                    Some(ip) => ip,
                    None => {
                        error!(
                            "Failed to get sender_proto_addr for ARP npid {}",
                            npid_parsed
                        );
                        continue;
                    }
                };
                arp_stats_map.insert(source_ip.to_string(), source_mac.to_string());
            }
        }
    }

    let mut protocol_stats_sorted: Vec<_> = protocol_stats_map.iter().collect();
    protocol_stats_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let protocol_stats: Vec<ProtocolStat> = protocol_stats_sorted
        .into_iter()
        .take(10)
        .map(|(protocol, count)| ProtocolStat::new(protocol.clone(), *count))
        .collect();

    let mut ingress_ip_stats_sorted: Vec<_> = ingress_ip_stat_map.into_iter().collect();
    ingress_ip_stats_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let ingress_ip_stats: Vec<IpStat> = ingress_ip_stats_sorted
        .into_iter()
        .take(10)
        .map(|(ip, count)| IpStat::new(ip, count))
        .collect();

    let mut egress_ip_start_sorted: Vec<_> = egress_ip_stat_map.into_iter().collect();
    egress_ip_start_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let egress_ip_stats: Vec<IpStat> = egress_ip_start_sorted
        .into_iter()
        .take(10)
        .map(|(ip, count)| IpStat::new(ip, count))
        .collect();

    let mut country_stats_sorted: Vec<_> = country_stats_map.into_iter().collect();
    country_stats_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    let country_stats: Vec<CountryStat> = country_stats_sorted
        .into_iter()
        .take(10)
        .map(|(country, _)| CountryStat::new(country))
        .collect();

    let arp_stats: Vec<ArpStat> = arp_stats_map
        .iter()
        .map(|(source_ip, source_mac)| ArpStat::new(source_ip.clone(), source_mac.clone()))
        .collect();

    let network_stats = NetworkStat::new(
        protocol_stats,
        ingress_ip_stats,
        egress_ip_stats,
        country_stats,
        arp_stats,
    );

    match serde_json::to_value(network_stats) {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Failed to serialize network_stats to JSON: {}", e);
            return Err("Failed to serialize network_stats".into());
        }
    }
}

#[tauri::command]
pub(crate) fn get_packet_data(
    state: State<AppState>,
    npid: String,
) -> Result<Vec<serde_json::Value>, String> {
    let npid_parsed = match npid.parse::<u32>() {
        Ok(npid) => npid,
        Err(e) => {
            error!("Failed to parse npid for log entry: {}", e);
            return Err("Failed to parse npid".into());
        }
    };

    let detailed_logs_store = match state.detailed_logs_store.read() {
        Ok(store) => store,
        Err(e) => {
            error!("Failed to acquire read lock on detailed_logs_store: {}", e);
            return Err("Failed to access detailed logs store".to_string());
        }
    };

    if let Some(json_str) = detailed_logs_store.peek(&npid_parsed) {
        let json_value: serde_json::Value = match serde_json::from_str(json_str) {
            Ok(value) => value,
            Err(e) => {
                error!("Failed to parse JSON for ARP npid {}: {}", npid_parsed, e);
                return Err("Failed to parse JSON".into());
            }
        };
        let mut packet_data = Vec::new();
        packet_data.push(json_value.clone());
        Ok(packet_data)
    } else {
        return Err("No detailed log found".to_string());
    }
}
