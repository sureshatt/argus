use crate::network::network_dumper::Context;
use chrono::Utc;
use pnet::packet::{
    icmp::{
        destination_unreachable::DestinationUnreachablePacket, echo_reply::EchoReplyPacket,
        echo_request::EchoRequestPacket, IcmpPacket, IcmpTypes,
    },
    ipv4::Ipv4Packet,
    Packet,
};
use serde_json::json;
use tauri::Emitter;

pub fn parse(ipv4_packet: &Ipv4Packet, context: &Context) -> Result<(), String> {
    let icmp_packet = IcmpPacket::new(ipv4_packet.payload());
    let source = ipv4_packet.get_source();
    let destination = ipv4_packet.get_destination();

    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let icmp_echo_reply = EchoReplyPacket::new(icmp_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmp_packet.packet().len().to_string(),
                    "info": "ICMP Echo Reply",
                    "interface": context.interface.name.to_string(),
                    "icmp_type": icmp_echo_reply.get_icmp_type().0.to_string(),
                    "icmp_code": icmp_echo_reply.get_icmp_code().0.to_string(),
                    "checksum": icmp_echo_reply.get_checksum().to_string(),
                    "identifier": icmp_echo_reply.get_identifier().to_string(),
                    "sequence_number": icmp_echo_reply.get_sequence_number().to_string(),
                    "payload": icmp_echo_reply.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
            IcmpTypes::EchoRequest => {
                let icmp_echo_request = EchoRequestPacket::new(icmp_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmp_packet.packet().len().to_string(),
                    "info": "ICMP Echo Request",
                    "interface": context.interface.name.to_string(),
                    "icmp_type": icmp_echo_request.get_icmp_type().0.to_string(),
                    "icmp_code": icmp_echo_request.get_icmp_code().0.to_string(),
                    "checksum": icmp_echo_request.get_checksum().to_string(),
                    "identifier": icmp_echo_request.get_identifier().to_string(),
                    "sequence_number": icmp_echo_request.get_sequence_number().to_string(),
                    "payload": icmp_echo_request.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
            IcmpTypes::DestinationUnreachable => {
                let icmp_destination_unreachable =
                    DestinationUnreachablePacket::new(icmp_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmp_packet.packet().len().to_string(),
                    "info": "ICMP Destination Unreachable",
                    "interface": context.interface.name.to_string(),
                    "icmp_type": icmp_destination_unreachable.get_icmp_type().0.to_string(),
                    "icmp_code": icmp_destination_unreachable.get_icmp_code().0.to_string(),
                    "checksum": icmp_destination_unreachable.get_checksum().to_string(),
                    "unused": icmp_destination_unreachable.get_unused().to_string(),
                    "next_hop_mtu": icmp_destination_unreachable.get_next_hop_mtu().to_string(),
                    "payload": icmp_destination_unreachable.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
            IcmpTypes::TimeExceeded => {
                let icmp_time_exceeded =
                    DestinationUnreachablePacket::new(icmp_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmp_packet.packet().len().to_string(),
                    "info": "ICMP Time Exceeded",
                    "interface": context.interface.name.to_string(),
                    "icmp_type": icmp_time_exceeded.get_icmp_type().0.to_string(),
                    "icmp_code": icmp_time_exceeded.get_icmp_code().0.to_string(),
                    "checksum": icmp_time_exceeded.get_checksum().to_string(),
                    "unused": icmp_time_exceeded.get_unused().to_string(),
                    "next_hop_mtu": icmp_time_exceeded.get_next_hop_mtu().to_string(),
                    "payload": icmp_time_exceeded.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
            _ => {
                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmp_packet.packet().len().to_string(),
                    "info": "ICMP",
                    "interface": context.interface.name.to_string(),
                    "icmp_type": icmp_packet.get_icmp_type().0.to_string(),
                    "icmp_code": icmp_packet.get_icmp_code().0.to_string(),
                    "checksum": icmp_packet.get_checksum().to_string(),
                    "payload": icmp_packet.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
        }
    } else {
        println!(
            "[{}]: Malformed ICMPv6 Packet",
            context.interface.name.to_string()
        );
    }
    Ok(())
}
