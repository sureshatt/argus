use pnet::packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet};
use tauri::Emitter;
use chrono::Utc;

use crate::{network::network_dumper::Context, NetworkLog};

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    context: &'a Context,
) -> Result<Ipv4Packet<'a>, String> {

    let ipv4_packet = Ipv4Packet::new(packet.payload());

    if let Some(ipv4_packet) = ipv4_packet {
       
        let netlog = NetworkLog {
            npid: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "IPv4".to_string(),
            source: ipv4_packet.get_source().to_string(),
            destination: ipv4_packet.get_destination().to_string(),
            length: ipv4_packet.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string()
        };

        let _ = context.app_handle.emit(
            "update",
            netlog,
        );

        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
