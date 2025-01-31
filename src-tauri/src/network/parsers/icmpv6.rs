use crate::network::network_dumper::Context;
use chrono::Utc;
use pnet::packet::{
    icmpv6::{
        echo_reply::EchoReplyPacket, echo_request::EchoRequestPacket, Icmpv6Packet, Icmpv6Types,
    },
    ipv6::Ipv6Packet,
    Packet,
};
use serde_json::json;
use tauri::Emitter;

pub fn parse(ipv6_packet: &Ipv6Packet, context: &Context) -> Result<(), String> {
    let icmpv6_packet = Icmpv6Packet::new(ipv6_packet.payload());
    let source = ipv6_packet.get_source();
    let destination = ipv6_packet.get_destination();

    if let Some(icmpv6_packet) = icmpv6_packet {
        match icmpv6_packet.get_icmpv6_type() {
            Icmpv6Types::EchoReply => {
                let icmpv6_echo_reply = EchoReplyPacket::new(icmpv6_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": "ICMP Echo Reply",
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_echo_reply.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_echo_reply.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_echo_reply.get_checksum().to_string(),
                    "identifier": icmpv6_echo_reply.get_identifier().to_string(),
                    "sequence_number": icmpv6_echo_reply.get_sequence_number().to_string(),
                    "payload": icmpv6_echo_reply.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
            Icmpv6Types::EchoRequest => {
                let icmpv6_echo_request = EchoRequestPacket::new(icmpv6_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMP",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": "ICMP Echo Request",
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_echo_request.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_echo_request.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_echo_request.get_checksum().to_string(),
                    "identifier": icmpv6_echo_request.get_identifier().to_string(),
                    "sequence_number": icmpv6_echo_request.get_sequence_number().to_string(),
                    "payload": icmpv6_echo_request.payload().to_vec()
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
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": "ICMPv6 Packet",
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_packet.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_packet.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_packet.get_checksum().to_string(),
                    "payload": icmpv6_packet.payload().to_vec()
                });

                let _ = context.app_handle.emit("update", icmp_json);
            }
        }
    } else {
        println!(
            "[{}]: Malformed ICMPv6 Packet",
            context.interface.name[..].to_string()
        );
    }

    Ok(())
}
