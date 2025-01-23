use pnet::packet::{tcp::TcpPacket, Packet};
use tauri::Emitter;
use crate::{network::{layers::network::IpPacket, network_dumper::Context},NetworkLog};
use chrono::Utc;

pub fn parse<'a>(
    packet: &'a IpPacket,
    context: &'a Context,
) -> Result<TcpPacket<'a>, String> {

    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        
        let netlog = NetworkLog {
            id: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "TCP".to_string(),
            source: format!("{}:{}", packet.get_source_ip(), tcp.get_source()),
            destination: format!("{}:{}", packet.get_destination_ip(), tcp.get_destination()),
            length: tcp.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string()
        };

        let _ = context.app_handle.emit(
            "update",
            netlog,
        );

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
