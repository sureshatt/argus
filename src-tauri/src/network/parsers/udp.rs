use pnet::{
    datalink::NetworkInterface,
    packet::{udp::UdpPacket, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use crate::{network::layers::network::IpPacket, Counter, NetworkLog};
use chrono::Utc;

pub fn parse<'a>(
    packet: &'a IpPacket,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter,
    parent_counter: &'a String
) -> Result<UdpPacket<'a>, String> {
    
    let udp = UdpPacket::new(packet.get_payload());

    if let Some(udp) = udp {
       
        let netlog = NetworkLog {
            id: counter.next(),
            parent: parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "TCP".to_string(),
            source: format!("{}:{}", packet.get_source_ip(), udp.get_source()),
            destination: format!("{}:{}", packet.get_destination_ip(), udp.get_destination()),
            length: udp.packet().len().to_string(),
            info: "".to_string(),
            interface: (&interface.name[..]).to_string()
        };

        let _ = app_handle.emit(
            "update",
            netlog,
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
