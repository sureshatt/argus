mod network;
use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;
use std::{
    sync::{Arc, RwLock},
    thread,
};
use tauri::{Emitter, State};

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
fn start_loop(state: State<AppState>, app_handle: tauri::AppHandle) {
    let selected = state.selected.clone();
    println!("starting the loop");

    thread::spawn(move || loop {
        let selected_interface = selected.read().unwrap();

        if *selected_interface != "" {
            println!("selection: {}", selected_interface);
            let _ = app_handle.emit("update", "hello ".to_owned() + &*selected_interface);
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            start_loop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
