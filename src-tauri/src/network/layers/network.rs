use pnet::datalink::NetworkInterface;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::AppHandle;

use crate::network::layers::datalink::DatalinkPacketPayload;
use crate::network::parsers::{icmp, icmpv6, ipv4, ipv6};

pub enum NetworkPacketPayload {
    Udp(Vec<u8>),
    Tcp(Vec<u8>),
    Icmp(),
    Icmpv6(),
}

pub fn process(
    packet: &DatalinkPacketPayload,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<NetworkPacketPayload, String> {
    match packet {
        DatalinkPacketPayload::Ipv4(payload) => {
            let ipv4_packet = ipv4::parse(payload, interface, app_handle, db)?;
            let next_header = ipv4_packet.get_next_level_protocol();
            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(ipv4_packet.payload().to_owned()))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(ipv4_packet.payload().to_owned()))
                }
                IpNextHeaderProtocols::Icmp => {
                    let _ = icmp::parse(ipv4_packet.payload(), interface, app_handle, db);
                    Ok(NetworkPacketPayload::Icmp())
                }
                _ => Err("Not supported".to_string()),
            }
        }

        DatalinkPacketPayload::Ipv6(payload) => {
            let ipv6_packet = ipv6::parse(payload, interface, app_handle, db)?;
            let next_header = ipv6_packet.get_next_header();
            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(ipv6_packet.payload().to_owned()))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(ipv6_packet.payload().to_owned()))
                }
                IpNextHeaderProtocols::Icmpv6 => {
                    let _ = icmpv6::parse(ipv6_packet.payload(), interface, app_handle, db);
                    Ok(NetworkPacketPayload::Icmpv6())
                }
                _ => Err("Not supported".to_string()),
            }
        }

        DatalinkPacketPayload::Arp() => {
            Err("ARP packets are not handled in the network layer".to_string())
        }
    }
}
