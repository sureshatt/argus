mod network;
mod storage;
use lru::LruCache;
use storage::log_entry::BasicLogEntry;
use storage::logger::listen_to_event;
use storage::log_analytics::{publish_stats, get_packet_data};
use network::network_interface::{get_net_ifaces, NetIface};
use network::network_dumper::dump;
use std::collections::VecDeque;
use std::num::NonZero;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use tauri::{Manager, State};

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    counter: Arc<RwLock<Counter>>,
    basic_logs_store: Arc<RwLock<VecDeque<BasicLogEntry>>>,
    detailed_logs_store: Arc<RwLock<LruCache<u32, String>>>,
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
    let max_number_of_logs = 1000;

    let basic_logs_store: VecDeque<BasicLogEntry> = VecDeque::new();
    let detailed_logs_store: LruCache<u32, String> = LruCache::new(NonZero::new(max_number_of_logs).unwrap());

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        counter: Arc::new(RwLock::new(sq_counter)),
        basic_logs_store: Arc::new(RwLock::new(basic_logs_store)),
        detailed_logs_store: Arc::new(RwLock::new(detailed_logs_store)),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            get_packet_data,
            dump
        ])
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            window.maximize().unwrap();
            let app_handle = app.handle().clone();
            listen_to_event(&app_handle, app.state::<AppState>(), max_number_of_logs);
            publish_stats(&app_handle, app.state::<AppState>());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
