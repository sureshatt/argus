use pnet::{datalink::NetworkInterface, packet::{ipv4::Ipv4Packet, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<Ipv4Packet<'a>, String> {

    let ipv4_packet = Ipv4Packet::new(packet);
    if let Some(ipv4_packet) = ipv4_packet {

        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: IPv4 Packet: {} > {}; length: {}",
                &interface.name[..],
                ipv4_packet.get_source(),
                ipv4_packet.get_destination(),
                ipv4_packet.packet().len()
            ),
        );

        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
