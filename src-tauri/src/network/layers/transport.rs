use pnet::datalink::NetworkInterface;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::AppHandle;

use crate::network::layers::network::NetworkPacketPayload;
use crate::network::parsers::{tcp, udp};
use crate::Counter;

pub enum TransportSegmentPayload<'a> {
    Dns(UdpPacket<'a>),
    Dhcp(UdpPacket<'a>),
    Tftp(UdpPacket<'a>),
    Ntp(UdpPacket<'a>),
    Snmp(UdpPacket<'a>),
    Rtsp(UdpPacket<'a>),
    Rtp(UdpPacket<'a>),

    Ftp(TcpPacket<'a>),
    Ssh(TcpPacket<'a>),
    Telnet(TcpPacket<'a>),
    Smtp(TcpPacket<'a>),
    DnsTcp(TcpPacket<'a>),
    Http(TcpPacket<'a>),
    Pop3(TcpPacket<'a>),
    Imap(TcpPacket<'a>),
    Https(TcpPacket<'a>),
    Imaps(TcpPacket<'a>),
    Pop3s(TcpPacket<'a>),
}

pub fn process<'a>(
    packet: &'a NetworkPacketPayload,
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter
) -> Result<TransportSegmentPayload<'a>, String> {
    match packet {
        NetworkPacketPayload::Udp(ip_packet) => {
            let udp = udp::parse(ip_packet, interface, app_handle, db, counter)?;
            match udp.get_destination() {
                53 => Ok(TransportSegmentPayload::Dns(udp)),
                67 | 68 => Ok(TransportSegmentPayload::Dhcp(udp)),
                69 => Ok(TransportSegmentPayload::Tftp(udp)),
                123 => Ok(TransportSegmentPayload::Ntp(udp)),
                161 | 162 => Ok(TransportSegmentPayload::Snmp(udp)),
                554 => Ok(TransportSegmentPayload::Rtsp(udp)),
                5004 | 5005 => Ok(TransportSegmentPayload::Rtp(udp)),
                _ => Err("Not supported".to_string()),
            }
        }

        NetworkPacketPayload::Tcp(ip_packet) => {
            let tcp = tcp::parse(ip_packet, interface, app_handle, db, counter)?;
            match tcp.get_destination() {
                20 | 21 => Ok(TransportSegmentPayload::Ftp(tcp)),
                22 => Ok(TransportSegmentPayload::Ssh(tcp)),
                23 => Ok(TransportSegmentPayload::Telnet(tcp)),
                25 => Ok(TransportSegmentPayload::Smtp(tcp)),
                53 => Ok(TransportSegmentPayload::DnsTcp(tcp)),
                80 => Ok(TransportSegmentPayload::Http(tcp)),
                110 => Ok(TransportSegmentPayload::Pop3(tcp)),
                143 => Ok(TransportSegmentPayload::Imap(tcp)),
                443 => Ok(TransportSegmentPayload::Https(tcp)),
                993 => Ok(TransportSegmentPayload::Imaps(tcp)),
                995 => Ok(TransportSegmentPayload::Pop3s(tcp)),
                _ => Err("Not supported".to_string()),
            }
        }

        _ => Err(format!("Unsupported Protocol")),
    }
}
