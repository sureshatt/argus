use pnet::{
    datalink::NetworkInterface,
    packet::{tcp::TcpPacket, Packet},
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
    counter: &'a Counter,
    parent_counter: &'a String
) -> Result<TcpPacket<'a>, String> {

    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: {} {} {} TCP Packet: {}:{} > {}:{}; length: {}",
                &interface.name[..],
                parent_counter,
                counter.next(),
                Utc::now().timestamp_millis(),
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
