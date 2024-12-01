mod network;
use std::sync::{Arc, RwLock};

use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;
use tauri::State;

#[derive(Default)]
struct AppState {
    selected: Arc<RwLock<String>>,
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

#[derive(Clone, Serialize)]
pub struct NetLogEvent {
    log: String,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            network::network_dumper::dump
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
