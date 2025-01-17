use pnet::{
    datalink::NetworkInterface,
    packet::{tcp::TcpPacket, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

use crate::network::layers::network::IpPacket;

pub fn parse<'a>(
    packet: &'a IpPacket,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<TcpPacket<'a>, String> {

    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
                &interface.name[..],
                packet.get_source_ip(),
                tcp.get_source(),
                packet.get_destination_ip(),
                tcp.get_destination(),
                tcp.packet().len()
            ),
        );

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
