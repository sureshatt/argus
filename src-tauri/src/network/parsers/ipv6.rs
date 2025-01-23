use pnet::packet::{ethernet::EthernetPacket, ipv6::Ipv6Packet, Packet};
use tauri::Emitter;
use chrono::Utc;

use crate::{network::network_dumper::Context, NetworkLog};

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    context: &'a Context,
) -> Result<Ipv6Packet<'a>, String> {

    let ipv6_packet = Ipv6Packet::new(packet.payload());

    if let Some(ipv6_packet) = ipv6_packet {
        
        let netlog = NetworkLog {
            id: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "IPv6".to_string(),
            source: ipv6_packet.get_source().to_string(),
            destination: ipv6_packet.get_destination().to_string(),
            length: ipv6_packet.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string()
        };

        let _ = context.app_handle.emit(
            "update",
            netlog,
        );

        Ok(ipv6_packet)
    } else {
        Err("Invalid Ipv6".to_string())
    }
}
