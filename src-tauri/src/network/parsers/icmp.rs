use pnet::{
    datalink::NetworkInterface,
    packet::{
        icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes},
        ipv4::Ipv4Packet,
        Packet,
    },
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::Counter;

pub fn parse(
    ipv4_packet: &Ipv4Packet,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
    counter: &Counter,
    parent_counter: &String
) -> Result<(), String> {
    let icmp_packet = IcmpPacket::new(ipv4_packet.payload());
    let source = ipv4_packet.get_source();
    let destination = ipv4_packet.get_destination();

    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let echo_reply_packet =
                    echo_reply::EchoReplyPacket::new(ipv4_packet.payload()).unwrap();

                let _ = app_handle.emit(
                    "update",
                    format!(
                        "[{}]: {} {} {} ICMP echo reply {} -> {} (seq={:?}, id={:?})",
                        &interface.name[..],
                        parent_counter,
                        counter.next(),
                        Utc::now().timestamp_millis(),
                        source,
                        destination,
                        echo_reply_packet.get_sequence_number(),
                        echo_reply_packet.get_identifier()
                    ),
                );
            }
            IcmpTypes::EchoRequest => {
                let echo_request_packet =
                    echo_request::EchoRequestPacket::new(ipv4_packet.payload()).unwrap();

                let _ = app_handle.emit(
                    "update",
                    format!(
                        "[{}]: {} {} {} ICMP echo request {} -> {} (seq={:?}, id={:?})",
                        &interface.name[..],
                        parent_counter,
                        counter.next(),
                        Utc::now().timestamp_millis(),
                        source,
                        destination,
                        echo_request_packet.get_sequence_number(),
                        echo_request_packet.get_identifier()
                    ),
                );
            }
            _ => {
                let _ = app_handle.emit(
                    "update",
                    format!(
                        "[{}]: {} {} {} ICMP packet {} -> {} (type={:?})",
                        &interface.name[..],
                        parent_counter,
                        counter.next(),
                        Utc::now().timestamp_millis(),
                        source,
                        destination,
                        icmp_packet.get_icmp_type()
                    ),
                );
            }
        }
    } else {
        println!("[{}]: Malformed ICMPv6 Packet", &interface.name[..]);
    }
    Ok(())
}
