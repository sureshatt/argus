mod network;
use network::network_interface::{get_net_ifaces, NetIface};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use tauri::{AppHandle, Listener, State};

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    db: Arc<RwLock<Surreal<Db>>>,
    counter: Arc<RwLock<Counter>>,
}

#[tauri::command]
async fn get_protocol_stats(
    state: State<'_, AppState>,
    net_iface: String,
) -> Result<Vec<serde_json::Value>, String> {
    let db = state
        .db
        .read()
        .map_err(|e| format!("Failed to read DB: {}", e))?
        .clone();
    let query = format!("SELECT protocol, COUNT() as count FROM logs WHERE interface = '{}' GROUP BY protocol ORDER BY count DESC", net_iface);
    match db.query(query).await {
        Ok(mut db_response) => {
            let data: Vec<serde_json::Value> = db_response
                .take(0)
                .map_err(|e| format!("Failed to take data from response: {}", e))?;
            Ok(data)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("DB query failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_ingress_ip_stats(
    state: State<'_, AppState>,
    netiface: NetIface,
) -> Result<Vec<serde_json::Value>, String> {
    println!("Getting ingress IP stats");
    let db = state
        .db
        .read()
        .map_err(|e| format!("Failed to read DB: {}", e))?
        .clone();
    let ipv4 = netiface.ipv4_address.split('/').next().unwrap();
    let ipv6: Vec<String> = netiface
        .ipv6_addresses
        .iter()
        .map(|ip| ip.split('/').next().unwrap().to_string())
        .collect();

    let query = "SELECT source, COUNT() as count 
         FROM logs 
         WHERE interface = $iface 
           AND protocol IN ['IPv4', 'IPv6'] 
           AND (protocol = 'IPv4' AND source != $ipv4 
                OR protocol = 'IPv6' AND source NOT IN $ipv6) 
         GROUP BY source 
         ORDER BY count DESC";

    match db
        .query(query)
        .bind(("iface", &netiface.name))
        .bind(("ipv4", &ipv4))
        .bind(("ipv6", &ipv6))
        .await
    {
        Ok(mut db_response) => {
            let data: Vec<serde_json::Value> = db_response
                .take(0)
                .map_err(|e| format!("Failed to take data from response: {}", e))?;
            Ok(data)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("DB query failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_egress_ip_stats(
    state: State<'_, AppState>,
    netiface: NetIface,
) -> Result<Vec<serde_json::Value>, String> {
    println!("Getting Egress IP stats");
    let db = state
        .db
        .read()
        .map_err(|e| format!("Failed to read DB: {}", e))?
        .clone();
    let ipv4 = netiface.ipv4_address.split('/').next().unwrap();
    let ipv6: Vec<String> = netiface
        .ipv6_addresses
        .iter()
        .map(|ip| ip.split('/').next().unwrap().to_string())
        .collect();

    let query = "SELECT destination, COUNT() as count 
         FROM logs 
         WHERE interface = $iface 
           AND protocol IN ['IPv4', 'IPv6'] 
           AND (protocol = 'IPv4' AND destination != $ipv4 
                OR protocol = 'IPv6' AND destination NOT IN $ipv6) 
         GROUP BY destination 
         ORDER BY count DESC";

    match db
        .query(query)
        .bind(("iface", &netiface.name))
        .bind(("ipv4", &ipv4))
        .bind(("ipv6", &ipv6))
        .await
    {
        Ok(mut db_response) => {
            let data: Vec<serde_json::Value> = db_response
                .take(0)
                .map_err(|e| format!("Failed to take data from response: {}", e))?;
            Ok(data)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("DB query failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_arp_ip_stats(
    state: State<'_, AppState>,
    netiface: NetIface,
) -> Result<Vec<serde_json::Value>, String> {
    println!("Getting ARP IP stats");
    let db = state
        .db
        .read()
        .map_err(|e| format!("Failed to read DB: {}", e))?
        .clone();

    let query = "SELECT sender_proto_addr, sender_hw_addr,  COUNT() as count 
        FROM logs 
        WHERE interface = $iface 
        AND protocol = 'ARP' 
        GROUP BY sender_proto_addr, sender_hw_addr 
        ORDER BY count DESC";

    match db.query(query).bind(("iface", &netiface.name)).await {
        Ok(mut db_response) => {
            let data: Vec<serde_json::Value> = db_response
                .take(0)
                .map_err(|e| format!("Failed to take data from response: {}", e))?;
            Ok(data)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(format!("DB query failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_packet_data(
    state: State<'_, AppState>,
    parent_id: String,
    net_iface: String,
) -> Result<Vec<serde_json::Value>, String> {
    println!(
        "Looking for packets with parent : {} and interface: {}",
        parent_id, net_iface
    );
    let db = state.db.read().unwrap().clone();
    let query = format!(
        "SELECT * FROM logs WHERE parent = '{}' AND interface = '{}' ORDER BY npid ASC",
        parent_id, net_iface
    );
    match db.query(query).await {
        Ok(mut response) => {
            let data: Vec<serde_json::Value> = response.take(0).unwrap();
            Ok(data)
        }
        Err(e) => Err(format!("Failed to get packet data: {}", e)),
    }
}

#[tauri::command]
fn set_selection(state: State<AppState>, selection: String) {
    println!("set_channel called with selection: {}", selection);
    let mut selected = state.selected.write().unwrap();
    *selected = selection;
}

#[tauri::command]
fn get_network_interfaces() -> Vec<NetIface> {
    get_net_ifaces()
        .into_iter()
        .filter(|iface| iface.has_ipv4())
        .collect()
}

fn listen_to_event(app_handle: &AppHandle, db: &Surreal<Db>) {
    let db_clone = db.clone();
    app_handle.listen("update", move |event| {
        let db_clone = db_clone.clone(); // Clone the db reference to move it into the closure

        tauri::async_runtime::spawn(async move {
            match serde_json::from_str::<serde_json::Value>(event.payload()) {
                Ok(json_payload) => {
                    let _: Vec<serde_json::Value> = db_clone
                        .create("logs")
                        .content(&json_payload)
                        .await
                        .unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON payload: {}", e);
                }
            }
        });
    });
}

struct Counter {
    value: AtomicUsize,
}

impl Counter {
    fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }

    fn next(&self) -> String {
        let number = self.value.fetch_add(1, Ordering::Relaxed);
        format!("{:06}", number)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("namespace").use_db("database").await.unwrap();
    let sq_counter = Counter::new();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        db: Arc::new(RwLock::new(db.clone())),
        counter: Arc::new(RwLock::new(sq_counter)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            get_packet_data,
            get_protocol_stats,
            get_ingress_ip_stats,
            get_egress_ip_stats,
            get_arp_ip_stats,
            network::network_dumper::dump
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            listen_to_event(&app_handle, &db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
