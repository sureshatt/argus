mod network;
mod storage;
use log::{error, info};
use lru::LruCache;
use network::ip_utils::load_ip_ranges;
use network::network_dumper::dump;
use network::network_interface::{get_net_ifaces, NetIface};
use std::collections::VecDeque;
use std::num::NonZero;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use storage::log_analytics::{get_packet_data, publish_stats};
use storage::log_entry::BasicLogEntry;
use storage::logger::listen_to_event;
use tauri::{Manager, State};
use tauri_plugin_log::{Builder, RotationStrategy, Target, TargetKind};

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
fn check_capture_permissions() -> bool {
    // Attempt to open /dev/bpf0 for reading.
    // Returns true if the current process has BPF read access (ChmodBPF daemon is active).
    // ENOENT (device not yet created) is treated as no permission.
    std::fs::OpenOptions::new()
        .read(true)
        .open("/dev/bpf0")
        .is_ok()
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
    info!("Starting Argus application...");

    let geo_ip_ranges = match load_ip_ranges() {
        Ok(ranges) => ranges,
        Err(e) => {
            error!("Failed to load IP ranges: {}", e);
            Vec::new()
        }
    };
    info!("Loaded {} IP ranges", geo_ip_ranges.len());

    let max_number_of_logs = 1000;
    let cap = match NonZero::new(max_number_of_logs) {
        Some(c) => c,
        None => {
            error!("Invalid max number of logs");
            return;
        }
    };
    info!(
        "Starting Argus application with max logs: {}",
        max_number_of_logs
    );

    let sq_counter = Counter::new();
    let detailed_logs_store: LruCache<u32, String> = LruCache::new(cap);
    let basic_logs_store: VecDeque<BasicLogEntry> = VecDeque::new();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        counter: Arc::new(RwLock::new(sq_counter)),
        basic_logs_store: Arc::new(RwLock::new(basic_logs_store)),
        detailed_logs_store: Arc::new(RwLock::new(detailed_logs_store)),
        geo_ip_ranges: Arc::new(RwLock::new(geo_ip_ranges)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(app_state)
        .plugin(
            Builder::new() // https://v2.tauri.app/plugin/logging/
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }),
                    Target::new(TargetKind::Stdout),
                ])
                .max_file_size(10 * 1024 * 1024) // 10 MB
                .level(log::LevelFilter::Debug)
                .rotation_strategy(RotationStrategy::KeepSome(5))
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            get_packet_data,
            dump,
            check_capture_permissions
        ])
        .setup(move |app| {
            let window = match app.get_webview_window("main") {
                Some(w) => w,
                None => {
                    error!("Failed to get main webview window");
                    return Err(Box::<dyn std::error::Error>::from("Main window not found"));
                }
            };
            if let Err(e) = window.maximize() {
                error!("Failed to maximize window: {}", e);
            }
            let app_handle = app.handle().clone();
            listen_to_event(&app_handle, app.state::<AppState>(), max_number_of_logs);
            publish_stats(&app_handle, app.state::<AppState>());
            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(|e| {
            error!("Error while running tauri application: {}", e);
        })
        .ok();

    info!("Argus application has started successfully");
}
