use pnet::{datalink::NetworkInterface, packet::tcp::TcpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<TcpPacket<'a>, String> {
    if packet.len() < 20 {
        return Err("TCP segment too short".to_string());
    }
    let data_offset = (packet[12] >> 4) * 4;

    if packet.len() < data_offset as usize {
        return Err("TCP header length exceeds packet size".to_string());
    }

    let tcp = TcpPacket::new(packet);

    if let Some(tcp) = tcp {
        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
