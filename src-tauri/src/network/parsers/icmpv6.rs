use pnet::packet::{
        icmpv6::{Icmpv6Packet, Icmpv6Types},
        ipv6::Ipv6Packet,
        Packet,
    };
use tauri:: Emitter;
use chrono::Utc;
use crate::{network::network_dumper::Context, NetworkLog};

pub fn parse(
    ipv6_packet: &Ipv6Packet,
    context: &Context,
) -> Result<(), String> {
    let icmpv6_packet = Icmpv6Packet::new(ipv6_packet.payload());
    let source = ipv6_packet.get_source();
    let destination = ipv6_packet.get_destination();

    if let Some(icmpv6_packet) = icmpv6_packet {
        match icmpv6_packet.get_icmpv6_type() {
            Icmpv6Types::EchoReply => {

                let netlog = NetworkLog {
                    id: context.counter.next(),
                    parent: context.parent_counter.to_string(),
                    timestamp:  Utc::now().timestamp_millis().to_string(),
                    protocol: "ICMPv6".to_string(),
                    source: source.to_string(),
                    destination: destination.to_string(),
                    length: icmpv6_packet.packet().len().to_string(),
                    info: "ICMP Echo Reply".to_string(),
                    interface: (context.interface.name[..]).to_string()
                };
        
                let _ = context.app_handle.emit(
                    "update",
                    netlog,
                );
            }
            Icmpv6Types::EchoRequest => {

                let netlog = NetworkLog {
                    id: context.counter.next(),
                    parent: context.parent_counter.to_string(),
                    timestamp:  Utc::now().timestamp_millis().to_string(),
                    protocol: "Ethernet".to_string(),
                    source: source.to_string(),
                    destination: destination.to_string(),
                    length: icmpv6_packet.packet().len().to_string(),
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
                    length: icmpv6_packet.packet().len().to_string(),
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
        println!("[{}]: Malformed ICMPv6 Packet", context.interface.name[..].to_string());
    }

    Ok(())
}
