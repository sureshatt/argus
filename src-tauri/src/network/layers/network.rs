use pnet::datalink::NetworkInterface;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::AppHandle;

use crate::network::layers::datalink::DatalinkPacket;
use crate::network::parsers::{icmp, icmpv6, ipv4, ipv6};
use crate::Counter;

// define a struct to provide consistant interface to the transport layer for ipv4 & ipv6
// Owns data instead of borrowing to prevent lifetime complexity. 
//i.e. have to create this struct from a local variable, hence have to copy
pub struct IpPacket {
    source_ip: String,
    destination_ip: String,
    payload: Vec<u8> // this requires cloning
}

impl IpPacket {

    pub fn from_ip_v4(ipv4_packet: Ipv4Packet) ->  Self {
        IpPacket {
            source_ip: ipv4_packet.get_source().to_string(),
            destination_ip: ipv4_packet.get_destination().to_string(),
            payload: ipv4_packet.payload().to_vec(),
        }
    }

    pub fn from_ip_v6(ipv6_packet: Ipv6Packet) ->  Self {
        IpPacket {
            source_ip: ipv6_packet.get_source().to_string(),
            destination_ip: ipv6_packet.get_destination().to_string(),
            payload: ipv6_packet.payload().to_vec(),
        }
    }
    
    pub fn get_source_ip(&self) -> &String {
        &self.source_ip
    }

    pub fn get_destination_ip(&self) -> &String {
        &self.destination_ip
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
}

pub enum NetworkPacketPayload {
    Udp(IpPacket),
    Tcp(IpPacket),
    Icmp(),
    Icmpv6(),
}

pub fn process<'a>(
    datalink_packet: &'a DatalinkPacket<'a>,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter,
    parent_counter: &'a String
) -> Result<NetworkPacketPayload, String> {
    match datalink_packet {
        DatalinkPacket::Ipv4(packet) => {
            let ipv4_packet = ipv4::parse(packet, interface, app_handle, db, counter, parent_counter)?;
            let next_header = ipv4_packet.get_next_level_protocol();
            
            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(IpPacket::from_ip_v4(ipv4_packet)))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(IpPacket::from_ip_v4(ipv4_packet)))
                }
                IpNextHeaderProtocols::Icmp => {
                    let _ = icmp::parse(&ipv4_packet, interface, app_handle, db, counter, parent_counter);
                    Ok(NetworkPacketPayload::Icmp())
                }
                _ => Err("Not supported".to_string()),
            }
        }

        DatalinkPacket::Ipv6(payload) => {
            let ipv6_packet = ipv6::parse(payload, interface, app_handle, db, counter, parent_counter)?;
            let next_header = ipv6_packet.get_next_header();

            match next_header {
                IpNextHeaderProtocols::Udp => {
                    Ok(NetworkPacketPayload::Udp(IpPacket::from_ip_v6(ipv6_packet)))
                }
                IpNextHeaderProtocols::Tcp => {
                    Ok(NetworkPacketPayload::Tcp(IpPacket::from_ip_v6(ipv6_packet)))
                }
                IpNextHeaderProtocols::Icmpv6 => {
                    let _ = icmpv6::parse(&ipv6_packet, interface, app_handle, db, counter, parent_counter);
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
