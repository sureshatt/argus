use pnet::{datalink::NetworkInterface, packet::{udp::UdpPacket, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

use crate::network::layers::network::Udp;

pub fn parse<'a>(packet: &'a Udp, interface: &'a NetworkInterface, app_handle: &'a AppHandle, db: &'a Surreal<Db>) -> Result<UdpPacket<'a>, String> {
   
   let udp;
    match packet {
        Udp::UdpIpV4(ipv4_packet) => {
            udp = UdpPacket::new(ipv4_packet.payload());
        }
        Udp::UdpIpV6(ipv6_packet) => {
            udp = UdpPacket::new(ipv6_packet.payload());
        }
    }

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
