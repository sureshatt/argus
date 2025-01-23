use pnet::{
    datalink::NetworkInterface,
    packet::{tcp::TcpPacket, Packet},
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
) -> Result<TcpPacket<'a>, String> {

    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        
        let netlog = NetworkLog {
            id: counter.next(),
            parent: parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "TCP".to_string(),
            source: format!("{}:{}", packet.get_source_ip(), tcp.get_source()),
            destination: format!("{}:{}", packet.get_destination_ip(), tcp.get_destination()),
            length: tcp.packet().len().to_string(),
            info: "".to_string(),
            interface: (&interface.name[..]).to_string()
        };

        let _ = app_handle.emit(
            "update",
            netlog,
        );

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
