mod network;
use network::network_interface::{get_net_ifaces, NetIface};

#[tauri::command]
fn get_network_interfaces() -> Vec<NetIface> {
  get_net_ifaces().into_iter().filter(|iface| iface.has_ipv4()).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_network_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
