use pnet::{
    datalink::NetworkInterface,
    packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::{Counter, NetworkLog};

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
       
        let netlog = NetworkLog {
            id: counter.next(),
            parent: parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "IPv4".to_string(),
            source: ipv4_packet.get_source().to_string(),
            destination: ipv4_packet.get_destination().to_string(),
            length: ipv4_packet.packet().len().to_string(),
            info: "".to_string(),
            interface: (&interface.name[..]).to_string()
        };

        let _ = app_handle.emit(
            "update",
            netlog,
        );

        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
