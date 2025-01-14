use pnet::datalink::NetworkInterface;
use pnet::packet::ip::IpNextHeaderProtocols::{self, Udp};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::AppHandle;

use crate::network::layers::datalink::DatalinkPacket;
use crate::network::parsers::{icmp, icmpv6, ipv4, ipv6};

pub enum NetworkPacketPayload<'a> {
    Udp(Udp<'a>),
    Tcp(Tcp<'a>),
    Icmp(),
    Icmpv6(),
}

pub enum Udp<'a> {
    UdpIpV4(Ipv4Packet<'a>),
    UdpIpV6(Ipv6Packet<'a>)
}

pub enum Tcp<'a> {
    TcpIpV4(Ipv4Packet<'a>),
    TcpIpV6(Ipv6Packet<'a>)
}

pub fn process<'a>(
    datalink_packet: &'a DatalinkPacket<'a>,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<NetworkPacketPayload<'a>, String> {

    match datalink_packet {
        DatalinkPacket::Ipv4(packet) => {
            let ipv4_packet = ipv4::parse(packet, interface, app_handle, db)?;
            let next_header = ipv4_packet.get_next_level_protocol();
            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(Udp::UdpIpV4(ipv4_packet)))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(Tcp::TcpIpV4(ipv4_packet)))
                }
                IpNextHeaderProtocols::Icmp => {
                    let _ = icmp::parse(ipv4_packet.payload(), interface, app_handle, db);
                    Ok(NetworkPacketPayload::Icmp())
                }
                _ => Err("Not supported".to_string()),
            }
        }

        DatalinkPacket::Ipv6(payload) => {
            let ipv6_packet = ipv6::parse(payload, interface, app_handle, db)?;
            let next_header = ipv6_packet.get_next_header();
            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(Udp::UdpIpV6(ipv6_packet)))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(Tcp::TcpIpV6(ipv6_packet)))
                }
                IpNextHeaderProtocols::Icmpv6 => {
                    let _ = icmpv6::parse(ipv6_packet.payload(), interface, app_handle, db);
                    Ok(NetworkPacketPayload::Icmpv6())
                }
                _ => Err("Not supported".to_string()),
            }
        }

        DatalinkPacket::Arp() => {
            Err("ARP packets are not handled in the network layer".to_string())
        }
    }
}
