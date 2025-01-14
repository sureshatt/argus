use pnet::{
    datalink::NetworkInterface,
    packet::{
        icmpv6::{echo_reply, echo_request, Icmpv6Packet, Icmpv6Types},
        ipv6::Ipv6Packet,
        Packet,
    },
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

pub fn parse(
    ipv6_packet: &Ipv6Packet,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    let icmpv6_packet = Icmpv6Packet::new(ipv6_packet.payload());
    let source = ipv6_packet.get_source();
    let destination = ipv6_packet.get_destination();

    if let Some(icmpv6_packet) = icmpv6_packet {
        match icmpv6_packet.get_icmpv6_type() {
            Icmpv6Types::EchoReply => {
                let echo_reply_packet =
                    echo_reply::EchoReplyPacket::new(ipv6_packet.payload()).unwrap();

                let _ = app_handle.emit(
                    "update",
                    format!(
                        "[{}]: ICMPv6 echo reply {} -> {} (seq={:?}, id={:?})",
                        &interface.name[..],
                        source,
                        destination,
                        echo_reply_packet.get_sequence_number(),
                        echo_reply_packet.get_identifier()
                    ),
                );
            }
            Icmpv6Types::EchoRequest => {
                let echo_request_packet =
                    echo_request::EchoRequestPacket::new(ipv6_packet.payload()).unwrap();

                let _ = app_handle.emit(
                    "update",
                    format!(
                        "[{}]: ICMPv6 echo request {} -> {} (seq={:?}, id={:?})",
                        &interface.name[..],
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
                        "[{}]: ICMPv6 packet {} -> {} (type={:?})",
                        &interface.name[..],
                        source,
                        destination,
                        icmpv6_packet.get_icmpv6_type()
                    ),
                );
            }
        }
    } else {
        println!("[{}]: Malformed ICMPv6 Packet", &interface.name[..]);
    }

    Ok(())
}
