use chrono::Utc;
use pnet::packet::{ethernet::EthernetPacket, ipv6::Ipv6Packet, Packet};
use serde_json::json;
use tauri::Emitter;

use crate::network::network_dumper::Context;

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    context: &'a Context,
) -> Result<Ipv6Packet<'a>, String> {
    let ipv6_packet = Ipv6Packet::new(packet.payload());

    if let Some(ipv6_packet) = ipv6_packet {
        let ipv6_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "IPv6",
            "source": ipv6_packet.get_source().to_string(),
            "destination": ipv6_packet.get_destination().to_string(),
            "length": ipv6_packet.packet().len().to_string(),
            "info": "",
            "interface": context.interface.name.to_string(),
            "version": ipv6_packet.get_version().to_string(),
            "traffic_class": ipv6_packet.get_traffic_class().to_string(),
            "flow_label": ipv6_packet.get_flow_label().to_string(),
            "payload_length": ipv6_packet.get_payload_length().to_string(),
            "next_header": ipv6_packet.get_next_header().to_string(),
            "hop_limit": ipv6_packet.get_hop_limit().to_string(),
            "source_ip": ipv6_packet.get_source().to_string(),
            "destination_ip": ipv6_packet.get_destination().to_string(),
            "payload": ipv6_packet.payload().to_vec()
        });

        let _ = context.app_handle.emit("update", ipv6_json);

        Ok(ipv6_packet)
    } else {
        Err("Invalid Ipv6".to_string())
    }
}
