use pnet::{datalink::NetworkInterface, packet::ethernet::EthernetPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<EthernetPacket<'a>, String> {
    if packet.len() < 14 {
        return Err("Packet too short".to_string());
    }

    Ok(EthernetPacket::new(packet).unwrap())
}
