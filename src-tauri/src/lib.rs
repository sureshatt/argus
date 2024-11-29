mod network;
use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;
use std::thread;
use tauri::Emitter;

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

#[tauri::command]
fn start_loop(selection: String, app_handle: tauri::AppHandle) {
    thread::spawn(move || loop {
        if selection != "" {
            println!("selection: {}", selection);
            let _ = app_handle.emit("update", "hello ".to_owned() + &selection);
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            start_loop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
