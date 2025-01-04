use pnet::{datalink::NetworkInterface, packet::arp::ArpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn handle(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    ArpPacket::new(packet).unwrap();
    Ok(())
}
