use pnet::{
    datalink::NetworkInterface,
    packet::{tcp::TcpPacket, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

use crate::network::layers::network::Tcp;

pub fn parse<'a>(
    packet: &'a Tcp,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<TcpPacket<'a>, String> {
    let tcp;
    let length;

    match packet {
        Tcp::TcpIpV4(ipv4_packet) => {
            tcp = TcpPacket::new(ipv4_packet.payload());
            length = ipv4_packet.get_total_length() - (ipv4_packet.get_header_length() as u16);
        }

        Tcp::TcpIpV6(ipv6_packet) => {
            tcp = TcpPacket::new(ipv6_packet.payload());
            length = ipv6_packet.get_payload_length()
        }
    }

    if let Some(tcp) = tcp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: TCP Packet: :{} > :{}; length: {}",
                &interface.name[..],
                tcp.get_source(),
                tcp.get_destination(),
                length
            ),
        );

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
