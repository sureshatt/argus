mod network;
use std::sync::{Arc, RwLock};
use network::network_interface::{get_net_ifaces, NetIface};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use tauri::{Listener, State, AppHandle};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
struct AppState {
    selected: Arc<RwLock<String>>,
    db: Arc<RwLock<Surreal<Db>>>,
    counter: Arc<RwLock<Counter>>
}

#[tauri::command]
async fn get_packet_data(state: State<'_, AppState>, parent_id: String) -> Result<Vec<serde_json::Value>, String> {
    println!("Looking for packets with parent : {}", parent_id);
    let db = state.db.read().unwrap().clone();
    let query = format!("SELECT * FROM logs WHERE parent = '{}'", parent_id);
    match db.query(query).await {
        Ok(mut response) => {
            let data: Vec<serde_json::Value> = response.take(0).unwrap();
            println!("Data: {:?}", data);
            Ok(data)
        },
        Err(e) => Err(format!("Failed to get packet data: {}", e)),
    }
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

fn listen_to_event(app_handle: &AppHandle, db: &Surreal<Db>) {
    let db_clone = db.clone();
    app_handle.listen("update", move |event| {
        let db_clone = db_clone.clone(); // Clone the db reference to move it into the closure

        tauri::async_runtime::spawn(async move {
            match serde_json::from_str::<serde_json::Value>(event.payload()) {
                Ok(json_payload) => {
                    let _: Vec<serde_json::Value> = db_clone.create("logs").content(&json_payload).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON payload: {}", e);
                }
            }  
        });
    });
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

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("namespace").use_db("database").await.unwrap();
    let sq_counter = Counter::new();

    let app_state = AppState {
        selected: Arc::new(RwLock::new("".to_string())),
        db: Arc::new(RwLock::new(db.clone())),
        counter: Arc::new(RwLock::new(sq_counter))
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_network_interfaces,
            set_selection,
            get_packet_data,
            network::network_dumper::dump
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            listen_to_event(&app_handle, &db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}