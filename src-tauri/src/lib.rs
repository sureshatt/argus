mod network;
use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;
use std::sync::RwLock;
use tauri::State;

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

struct AppState {
    kill_channel: RwLock<bool>, // Shared, thread-safe boolean
}

#[tauri::command]
fn toggle_kill_current_channel_flag(state: State<'_, AppState>) -> bool {
    println!("channel toggeling");
    let mut kill_channel = state.kill_channel.write().unwrap(); // Lock the Mutex to access the boolean
    *kill_channel = !*kill_channel;
    *kill_channel // Return the updated value
}

#[tauri::command]
async fn listen_to_event(
    interface: String,
    channel: tauri::ipc::Channel<NetLogEvent>,
    state: State<'_, AppState>,
) -> Result<bool, ()> {
    println!("channel called: {}", interface);
    
    loop {
        let kill_channel_read = state.kill_channel.read().unwrap();

        if *kill_channel_read {
            println!("channel killded");
            break;
        }

        let event = NetLogEvent {
            log: "hello from: ".to_string() + &interface + "-" + &channel.id().to_string(),
        };

        if let Err(e) = channel.send(event.clone()) {
            eprintln!("Failed to send message: {:?}", e);
        }

        // Sleep to simulate real-time intervals
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(true)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            kill_channel: RwLock::new(false), // Initialize state
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            listen_to_event,
            toggle_kill_current_channel_flag
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
