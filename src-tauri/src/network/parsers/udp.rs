use pnet::packet::{udp::UdpPacket, Packet};
use tauri::Emitter;
use crate::{network::{layers::network::IpPacket, network_dumper::Context}, NetworkLog};
use chrono::Utc;

pub fn parse<'a>(
    packet: &'a IpPacket,
    context: &'a Context,
) -> Result<UdpPacket<'a>, String> {
    
    let udp = UdpPacket::new(packet.get_payload());

    if let Some(udp) = udp {
       
        let netlog = NetworkLog {
            npid: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "TCP".to_string(),
            source: format!("{}:{}", packet.get_source_ip(), udp.get_source()),
            destination: format!("{}:{}", packet.get_destination_ip(), udp.get_destination()),
            length: udp.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string()
        };

        let _ = context.app_handle.emit(
            "update",
            netlog,
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
