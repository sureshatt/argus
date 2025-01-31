use pnet::packet::{udp::UdpPacket, Packet};
use serde_json::json;
use tauri::Emitter;
use crate::network::{layers::network::IpPacket, network_dumper::Context};
use chrono::Utc;

pub fn parse<'a>(
    packet: &'a IpPacket,
    context: &'a Context,
) -> Result<UdpPacket<'a>, String> {
    
    let udp = UdpPacket::new(packet.get_payload());

    if let Some(udp) = udp {
       
        let udp_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "UDP",
            "source": format!("{}:{}", packet.get_source_ip(), udp.get_source()),
            "destination": format!("{}:{}", packet.get_destination_ip(), udp.get_destination()),
            "length": udp.packet().len().to_string(),
            "info": "",
            "interface": context.interface.name.to_string(),
            "udp_source": udp.get_source().to_string(),
            "udp_destination": udp.get_destination().to_string(),
            "checksum": udp.get_checksum().to_string(),
            "payload": udp.payload().to_vec()
        });

        let _ = context.app_handle.emit(
            "update",
            udp_json,
        );

        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
