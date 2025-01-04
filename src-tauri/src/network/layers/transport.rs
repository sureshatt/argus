use pnet::datalink::NetworkInterface;
use pnet::packet::Packet;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tauri::AppHandle;

use crate::network::layers::network::NetworkPacketPayload;
use crate::network::parsers::{tcp, udp};

pub enum TransportSegmentPayload {
    Dns(Vec<u8>),
    Dhcp(Vec<u8>),
    Tftp(Vec<u8>),
    Ntp(Vec<u8>),
    Snmp(Vec<u8>),
    Rtsp(Vec<u8>),
    Rtp(Vec<u8>),

    Ftp(Vec<u8>),
    Ssh(Vec<u8>),
    Telnet(Vec<u8>),
    Smtp(Vec<u8>),
    Http(Vec<u8>),
    Pop3(Vec<u8>),
    Imap(Vec<u8>),
    Https(Vec<u8>),
    SmtpSub(Vec<u8>),
    Imaps(Vec<u8>),
    Pop3s(Vec<u8>),
}

pub fn process(
    packet: &NetworkPacketPayload,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<TransportSegmentPayload, String> {
    match packet {
        NetworkPacketPayload::Udp(payload) => {
            let udp = udp::parse(payload, interface, app_handle, db)?;
            match udp.get_destination() {
                53 => Ok(TransportSegmentPayload::Dns(udp.payload().to_owned())),
                67 | 68 => Ok(TransportSegmentPayload::Dhcp(udp.payload().to_owned())),
                69 => Ok(TransportSegmentPayload::Tftp(udp.payload().to_owned())),
                123 => Ok(TransportSegmentPayload::Ntp(udp.payload().to_owned())),
                161 | 162 => Ok(TransportSegmentPayload::Snmp(udp.payload().to_owned())),
                554 => Ok(TransportSegmentPayload::Rtsp(udp.payload().to_owned())),
                5004 | 5005 => Ok(TransportSegmentPayload::Rtp(udp.payload().to_owned())),
                _ => Err("Not supported".to_string()),
            }
        }
        NetworkPacketPayload::Tcp(paylod) => {
            let tcp = tcp::parse(paylod, interface, app_handle, db)?;
            match tcp.get_destination() {
                20 | 21 => Ok(TransportSegmentPayload::Ftp(tcp.payload().to_owned())),
                22 => Ok(TransportSegmentPayload::Ssh(tcp.payload().to_owned())),
                23 => Ok(TransportSegmentPayload::Telnet(tcp.payload().to_owned())),
                25 => Ok(TransportSegmentPayload::Smtp(tcp.payload().to_owned())),
                53 => Ok(TransportSegmentPayload::Dns(tcp.payload().to_owned())),
                80 => Ok(TransportSegmentPayload::Http(tcp.payload().to_owned())),
                110 => Ok(TransportSegmentPayload::Pop3(tcp.payload().to_owned())),
                143 => Ok(TransportSegmentPayload::Imap(tcp.payload().to_owned())),
                443 => Ok(TransportSegmentPayload::Https(tcp.payload().to_owned())),
                993 => Ok(TransportSegmentPayload::Imaps(tcp.payload().to_owned())),
                995 => Ok(TransportSegmentPayload::Pop3s(tcp.payload().to_owned())),
                _ => Err("Not supported".to_string()),
            }
        }

        _ => Err(format!("Unsupported Protocol")),
    }
}
