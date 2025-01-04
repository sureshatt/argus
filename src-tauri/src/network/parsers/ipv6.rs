use pnet::{datalink::NetworkInterface, packet::ipv6::Ipv6Packet};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<Ipv6Packet<'a>, String> {
    let ipv6_packet = Ipv6Packet::new(packet);
    if let Some(ipv6_packet) = ipv6_packet {
        Ok(ipv6_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
