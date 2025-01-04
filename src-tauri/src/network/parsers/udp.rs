use pnet::{datalink::NetworkInterface, packet::udp::UdpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse<'a>(packet: &'a [u8], interface: &'a NetworkInterface, app_handle: &'a AppHandle, db: &'a Surreal<Db>) -> Result<UdpPacket<'a>, String> {
    if packet.len() < 8 {
        return Err("UDP datagram too short".to_string());
    }

    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
