mod network;
use std::sync::{Arc, RwLock};
use network::network_interface::{get_net_ifaces, NetIface};
use serde::Serialize;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use tauri::State;

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    db: Arc<RwLock<Surreal<Db>>>,
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

#[derive(Clone, Serialize)]
pub struct NetLogEvent {
    log: String,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("namespace").use_db("database").await.unwrap();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        db: Arc::new(RwLock::new(db))
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            network::network_dumper::dump
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}