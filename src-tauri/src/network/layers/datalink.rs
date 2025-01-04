use crate::network::parsers::{arp, ethernet};
use pnet::packet::{ethernet::EtherTypes, Packet};
pub enum DatalinkPacketPayload {
    Ipv4(Vec<u8>),
    Ipv6(Vec<u8>),
    Arp(),
}

pub fn process(packet: &[u8]) -> Result<DatalinkPacketPayload, String> {
    let frame = ethernet::parse(packet)?;

    match frame.get_ethertype() {
        EtherTypes::Ipv4 => Ok(DatalinkPacketPayload::Ipv4(frame.payload().to_owned())),
        EtherTypes::Ipv6 => Ok(DatalinkPacketPayload::Ipv6(frame.payload().to_owned())),
        EtherTypes::Arp => {
            let _ = arp::handle(frame.payload());
            Ok(DatalinkPacketPayload::Arp())
        }
        _ => Err(format!("Unsupported Ethertype:{:?}", frame.get_ethertype())),
    }
}
