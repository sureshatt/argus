use pnet::{
    datalink::NetworkInterface,
    packet::{udp::UdpPacket, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

use crate::network::layers::network::Udp;

pub fn parse<'a>(
    packet: &'a Udp,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<UdpPacket<'a>, String> {
    let udp;
    let source_ip;
    let destination_ip;

    match packet {
        Udp::UdpIpV4(ipv4_packet) => {
            udp = UdpPacket::new(ipv4_packet.payload());
            source_ip = ipv4_packet.get_source().to_string();
            destination_ip = ipv4_packet.get_destination().to_string();
        }
        Udp::UdpIpV6(ipv6_packet) => {
            udp = UdpPacket::new(ipv6_packet.payload());
            source_ip = ipv6_packet.get_source().to_string();
            destination_ip = ipv6_packet.get_destination().to_string();
        }
    }

    if let Some(udp) = udp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: UDP Packet: {}:{} > {}:{}; length: {}",
                &interface.name[..],
                source_ip,
                udp.get_source(),
                destination_ip,
                udp.get_destination(),
                udp.get_length()
            ),
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
