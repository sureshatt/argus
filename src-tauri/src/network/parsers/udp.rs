use pnet::{
    datalink::NetworkInterface,
    packet::udp::UdpPacket,
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use crate::{network::layers::network::IpPacket, Counter};
use chrono::Utc;

pub fn parse<'a>(
    packet: &'a IpPacket,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter
) -> Result<UdpPacket<'a>, String> {
    
    let udp = UdpPacket::new(packet.get_payload());

    if let Some(udp) = udp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: {} {} UDP Packet: {}:{} > {}:{}; length: {}",
                &interface.name[..],
                counter.next(),
                Utc::now().timestamp_millis(),
                packet.get_source_ip(),
                udp.get_source(),
                packet.get_destination_ip(),
                udp.get_destination(),
                udp.get_length()
            ),
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
