use pnet::datalink::NetworkInterface;
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub mod application;
pub mod datalink;
pub mod network;
pub mod transport;

pub fn process_packet(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    
    let datalink_frame_payload = datalink::process(packet, interface, app_handle, db)?;
    let network_packet_payload =
        network::process(&datalink_frame_payload, interface, app_handle, db)?;
    let transport_segment_payload =
        transport::process(&network_packet_payload, interface, app_handle, db)?;
    application::process(&transport_segment_payload, interface, app_handle, db)
}
