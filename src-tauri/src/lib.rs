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

    std::thread::spawn(move || {

        loop {
            let event = NetLogEvent {
                log: "hello from: ".to_string() + &interface + "-" + &channel.id().to_string(),
            };

            if let Err(e) = channel.send(event.clone()) {
                eprintln!("Failed to send message: {:?}", e);
            }

            // Sleep to simulate real-time intervals
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
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
