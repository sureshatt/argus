mod network;
mod storage;
use storage::logger::listen_to_event;
use network::network_interface::{get_net_ifaces, NetIface};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use tauri::State;

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    counter: Arc<RwLock<Counter>>,
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
    let sq_counter = Counter::new();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        counter: Arc::new(RwLock::new(sq_counter)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            network::network_dumper::dump
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            listen_to_event(&app_handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
