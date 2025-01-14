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
    let tcp_length;
    let source_ip;
    let destination_ip;

    match packet {
        Tcp::TcpIpV4(ipv4_packet) => {
            tcp = TcpPacket::new(ipv4_packet.payload());
            tcp_length = ipv4_packet.get_total_length() - (ipv4_packet.get_header_length() as u16);
            source_ip = ipv4_packet.get_source().to_string();
            destination_ip = ipv4_packet.get_destination().to_string();
        }

        Tcp::TcpIpV6(ipv6_packet) => {
            tcp = TcpPacket::new(ipv6_packet.payload());
            tcp_length = ipv6_packet.get_payload_length();
            source_ip = ipv6_packet.get_source().to_string();
            destination_ip = ipv6_packet.get_destination().to_string();
        }
    }

    if let Some(tcp) = tcp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
                &interface.name[..],
                source_ip,
                tcp.get_source(),
                destination_ip,
                tcp.get_destination(),
                tcp_length
            ),
        );

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
