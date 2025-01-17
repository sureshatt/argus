use pnet::{
    datalink::NetworkInterface,
    packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::Counter;

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &Counter,
    parent_counter: &String
) -> Result<Ipv4Packet<'a>, String> {
    let ipv4_packet = Ipv4Packet::new(packet.payload());
    if let Some(ipv4_packet) = ipv4_packet {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: {} {} {} IPv4 Packet: {} > {}; length: {}",
                &interface.name[..],
                parent_counter,
                counter.next(),
                Utc::now().timestamp_millis(),
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
