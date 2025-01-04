use pnet::{datalink::NetworkInterface, packet::ipv4::Ipv4Packet};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<Ipv4Packet<'a>, String> {
    let ipv4_packet = Ipv4Packet::new(packet);
    if let Some(ipv4_packet) = ipv4_packet {
        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
