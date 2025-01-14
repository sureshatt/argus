use pnet::{datalink::NetworkInterface, packet::{ethernet::EthernetPacket, ipv6::Ipv6Packet, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<Ipv6Packet<'a>, String> {

    let ipv6_packet = Ipv6Packet::new(packet.payload());
    if let Some(ipv6_packet) = ipv6_packet {

        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: IPv6 Packet: {} > {}; length: {}",
                &interface.name[..],
                ipv6_packet.get_source(),
                ipv6_packet.get_destination(),
                ipv6_packet.packet().len()
            ),
        );
        Ok(ipv6_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
