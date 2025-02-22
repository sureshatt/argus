use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use crate::network::layers::network::NetworkPacketPayload;
use crate::network::network_dumper::Context;
use crate::network::parsers::{tcp, udp};

use super::network::IpPacket;

pub struct TransportSegment {
    source: String,
    destination: String,
    payload: Vec<u8>,
}

impl TransportSegment {

    pub fn from_udp(udp: UdpPacket, ip_packet: &IpPacket) -> Self {
        TransportSegment {
            source: format!("{}:{}", ip_packet.get_source_ip(), udp.get_source()),
            destination: format!("{}:{}", ip_packet.get_destination_ip(), udp.get_source()),
            payload: udp.payload().to_vec(),
        }
    }

    pub fn from_tcp(tcp: TcpPacket, ip_packet: &IpPacket) -> Self {
        TransportSegment {
            source: format!("{}:{}", ip_packet.get_source_ip(), tcp.get_source()),
            destination: format!("{}:{}", ip_packet.get_destination_ip(), tcp.get_source()),
            payload: tcp.payload().to_vec(),
        }
    }

    pub fn get_source(&self) -> &String {
        &self.source
    }

    pub fn get_destination(&self) -> &String {
        &self.destination
    }

    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
}

pub enum TransportSegmentPayload {
    Dns(TransportSegment),
    Dhcp(TransportSegment),
    Tftp(TransportSegment),
    Ntp(TransportSegment),
    Snmp(TransportSegment),
    Rtsp(TransportSegment),
    Rtp(TransportSegment),

    Ftp(TransportSegment),
    Ssh(TransportSegment),
    Telnet(TransportSegment),
    Smtp(TransportSegment),
    DnsTcp(TransportSegment),
    Http(TransportSegment),
    Pop3(TransportSegment),
    Imap(TransportSegment),
    Https(TransportSegment),
    Imaps(TransportSegment),
    Pop3s(TransportSegment),
}

pub fn process<'a>(
    packet: &'a NetworkPacketPayload,
    context: &'a Context,
) -> Result<TransportSegmentPayload, String> {
    match packet {
        NetworkPacketPayload::Udp(ip_packet) => {
            let udp = udp::parse(ip_packet, context)?;
            match udp.get_destination() {
                53 => Ok(TransportSegmentPayload::Dns(TransportSegment::from_udp(udp, ip_packet))),
                67 | 68 => Ok(TransportSegmentPayload::Dhcp(TransportSegment::from_udp(udp, ip_packet))),
                69 => Ok(TransportSegmentPayload::Tftp(TransportSegment::from_udp(udp, ip_packet))),
                123 => Ok(TransportSegmentPayload::Ntp(TransportSegment::from_udp(udp, ip_packet))),
                161 | 162 => Ok(TransportSegmentPayload::Snmp(TransportSegment::from_udp(udp, ip_packet))),
                554 => Ok(TransportSegmentPayload::Rtsp(TransportSegment::from_udp(udp, ip_packet))),
                5004 | 5005 => Ok(TransportSegmentPayload::Rtp(TransportSegment::from_udp(udp, ip_packet))),
                _ => Err("Not supported".to_string()),
            }
        }

        NetworkPacketPayload::Tcp(ip_packet) => {
            let tcp = tcp::parse(ip_packet, context)?;
            match tcp.get_destination() {
                20 | 21 => Ok(TransportSegmentPayload::Ftp(TransportSegment::from_tcp(tcp, ip_packet))),
                22 => Ok(TransportSegmentPayload::Ssh(TransportSegment::from_tcp(tcp, ip_packet))),
                23 => Ok(TransportSegmentPayload::Telnet(TransportSegment::from_tcp(tcp, ip_packet))),
                25 => Ok(TransportSegmentPayload::Smtp(TransportSegment::from_tcp(tcp, ip_packet))),
                53 => Ok(TransportSegmentPayload::DnsTcp(TransportSegment::from_tcp(tcp, ip_packet))),
                80 => Ok(TransportSegmentPayload::Http(TransportSegment::from_tcp(tcp, ip_packet))),
                110 => Ok(TransportSegmentPayload::Pop3(TransportSegment::from_tcp(tcp, ip_packet))),
                143 => Ok(TransportSegmentPayload::Imap(TransportSegment::from_tcp(tcp, ip_packet))),
                443 => Ok(TransportSegmentPayload::Https(TransportSegment::from_tcp(tcp, ip_packet))),
                993 => Ok(TransportSegmentPayload::Imaps(TransportSegment::from_tcp(tcp, ip_packet))),
                995 => Ok(TransportSegmentPayload::Pop3s(TransportSegment::from_tcp(tcp, ip_packet))),
                _ => Err("Not supported".to_string()),
            }
        }

        _ => Err(format!("Unsupported Protocol")),
    }
}
