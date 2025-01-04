use pnet::{datalink::NetworkInterface, packet::udp::UdpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

pub fn parse<'a>(packet: &'a [u8], interface: &'a NetworkInterface, app_handle: &'a AppHandle, db: &'a Surreal<Db>) -> Result<UdpPacket<'a>, String> {
    if packet.len() < 8 {
        return Err("UDP datagram too short".to_string());
    }

    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {

        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: UDP Packet: :{} > :{}; length: {}",
                &interface.name[..],
                udp.get_source(),
                udp.get_destination(),
                udp.get_length()
            ),
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
