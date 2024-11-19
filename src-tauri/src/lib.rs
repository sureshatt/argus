mod network;
use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;

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
fn listen_to_event(interface: String, channel: tauri::ipc::Channel<NetLogEvent>) {
    println!("channel called: {}", interface);

    for i in 0..5 {

        let event = NetLogEvent {
            log: "hello".to_string() + &&i.to_string(),
        };

        channel.send(event).unwrap();
        println!("channel ended: {}", interface);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            listen_to_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
