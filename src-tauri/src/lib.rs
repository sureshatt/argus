mod network;
mod storage;
use lru::LruCache;
use network::ip_utils::load_ip_ranges;
use network::network_dumper::dump;
use network::network_interface::{get_net_ifaces, NetIface};
use std::collections::VecDeque;
use std::fs;
use std::num::NonZero;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use storage::log_analytics::{get_packet_data, publish_stats};
use storage::log_entry::BasicLogEntry;
use storage::logger::listen_to_event;
use tauri::{Manager, State};
use tracing::{info, error};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};
use dir::home_dir;

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    counter: Arc<RwLock<Counter>>,
    basic_logs_store: Arc<RwLock<VecDeque<BasicLogEntry>>>,
    detailed_logs_store: Arc<RwLock<LruCache<u32, String>>>,
    geo_ip_ranges: Arc<RwLock<Vec<network::ip_utils::IpRange>>>,
}

#[tauri::command]
fn set_selection(state: State<AppState>, selection: String) {
    info!("set_channel called with selection: {}", selection);
    let mut selected = match state.selected.write() {
        Ok(selected) => selected,
        Err(_) => {
            error!("Failed to acquire write lock");
            return;
        }
    };
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
    init_logging();
    
    let max_number_of_logs = 1000;
    let cap = match NonZero::new(max_number_of_logs) {
        Some(c) => c,
        None => {
            error!("Invalid max number of logs");
            return;
        }
    };
    info!("Starting Argus application with max logs: {}", max_number_of_logs);
    
    let detailed_logs_store: LruCache<u32, String> =
    LruCache::new(cap);
    
    
    let sq_counter = Counter::new();
    let geo_ip_ranges = load_ip_ranges().unwrap();
    let basic_logs_store: VecDeque<BasicLogEntry> = VecDeque::new();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        counter: Arc::new(RwLock::new(sq_counter)),
        basic_logs_store: Arc::new(RwLock::new(basic_logs_store)),
        detailed_logs_store: Arc::new(RwLock::new(detailed_logs_store)),
        geo_ip_ranges: Arc::new(RwLock::new(geo_ip_ranges)),
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

fn init_logging() {
    let log_dir = macos_log_dir("argus");
    fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    let file_appender = rolling::daily(log_dir, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Optionally store `_guard` in a global/static so it's not dropped
    std::mem::forget(_guard); // simple way to retain it

    fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    tracing::info!("Argus file logger initialized");
}

fn macos_log_dir(app_name: &str) -> PathBuf {
    let mut path = home_dir().expect("Could not determine home directory");
    path.push("Library");
    path.push("Logs");
    path.push(app_name);
    path
}
