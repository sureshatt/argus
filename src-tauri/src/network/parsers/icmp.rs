use pnet::packet::{
        icmp::{IcmpPacket, IcmpTypes},
        ipv4::Ipv4Packet,
        Packet,
    };
use chrono::Utc;
use tauri::Emitter;

use crate::{network::network_dumper::Context, NetworkLog};

pub fn parse(
    ipv4_packet: &Ipv4Packet,
    context: &Context,
) -> Result<(), String> {

    let icmp_packet = IcmpPacket::new(ipv4_packet.payload());
    let source = ipv4_packet.get_source();
    let destination = ipv4_packet.get_destination();

    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {

                let netlog = NetworkLog {
                    id: context.counter.next(),
                    parent: context.parent_counter.to_string(),
                    timestamp:  Utc::now().timestamp_millis().to_string(),
                    protocol: "ICMP".to_string(),
                    source: source.to_string(),
                    destination: destination.to_string(),
                    length: icmp_packet.packet().len().to_string(),
                    info: "ICMP Echo Reply".to_string(),
                    interface: (context.interface.name[..]).to_string()
                };
        
                let _ = context.app_handle.emit(
                    "update",
                    netlog,
                );

            }
            IcmpTypes::EchoRequest => {

                let netlog = NetworkLog {
                    id: context.counter.next(),
                    parent: context.parent_counter.to_string(),
                    timestamp:  Utc::now().timestamp_millis().to_string(),
                    protocol: "Ethernet".to_string(),
                    source: source.to_string(),
                    destination: destination.to_string(),
                    length: icmp_packet.packet().len().to_string(),
                    info: "ICMP Echo Request".to_string(),
                    interface: (context.interface.name[..]).to_string()
                };
        
                let _ = context.app_handle.emit(
                    "update",
                    netlog,
                );

            }
            _ => {
                let netlog = NetworkLog {
                    id: context.counter.next(),
                    parent: context.parent_counter.to_string(),
                    timestamp:  Utc::now().timestamp_millis().to_string(),
                    protocol: "ICMP".to_string(),
                    source: source.to_string(),
                    destination: destination.to_string(),
                    length: icmp_packet.packet().len().to_string(),
                    info: "ICMP".to_string(),
                    interface: (context.interface.name[..]).to_string()
                };
        
                let _ = context.app_handle.emit(
                    "update",
                    netlog,
                );
            }
        }
    } else {
        println!("[{}]: Malformed ICMPv6 Packet", context.interface.name.to_string());
    }
    Ok(())
}
