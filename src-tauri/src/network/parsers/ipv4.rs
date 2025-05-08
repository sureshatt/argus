use chrono::Utc;
use pnet::packet::{
    ethernet::EthernetPacket,
    ipv4::{Ipv4OptionPacket, Ipv4Packet},
    Packet,
};
use serde_json::json;
use tauri::Emitter;

use crate::network::network_dumper::Context;

pub fn parse<'a>(
    packet: &'a EthernetPacket<'a>,
    context: &'a Context,
) -> Result<Ipv4Packet<'a>, String> {
    let ipv4_packet_op = Ipv4Packet::new(packet.payload());

    if let Some(ipv4_packet) = ipv4_packet_op {
        let ipv4_options_packet = Ipv4OptionPacket::new(ipv4_packet.payload());
        let mut ipv4_options_json = json!({});
        ipv4_options_packet.map(|ipv4_options_packet| {
            ipv4_options_json = json!({
                "copied": ipv4_options_packet.get_copied().to_string(),
                "class": ipv4_options_packet.get_class().to_string(),
                "option_number": ipv4_options_packet.get_number().0.to_string(),
                "length": ipv4_options_packet.get_length().to_vec(),
                "data": ipv4_options_packet.payload().to_vec()
            })
        });

        let ipv4_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "IPv4",
            "source": ipv4_packet.get_source().to_string(),
            "destination": ipv4_packet.get_destination().to_string(),
            "length": ipv4_packet.packet().len().to_string(),
            "info": "",
            "interface": context.interface.name.to_string(),
            "version": ipv4_packet.get_version().to_string(),
            "header_length": ipv4_packet.get_header_length().to_string(),
            "dscp": ipv4_packet.get_dscp().to_string(),
            "ecn": ipv4_packet.get_ecn().to_string(),
            "total_length": ipv4_packet.get_total_length().to_string(),
            "identification": ipv4_packet.get_identification().to_string(),
            "flags": ipv4_packet.get_flags().to_string(),
            "fragment_offset": ipv4_packet.get_fragment_offset().to_string(),
            "ttl": ipv4_packet.get_ttl().to_string(),
            "next_level_protocol": ipv4_packet.get_next_level_protocol().to_string(),
            "checksum": ipv4_packet.get_checksum().to_string(),
            "source_ip": ipv4_packet.get_source().to_string(),
            "destination_ip": ipv4_packet.get_destination().to_string(),
            "options": ipv4_options_json,
            "payload": ipv4_packet.payload().to_vec()
        });

        let _ = context.app_handle.emit("all_logs_event", ipv4_json);

        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
