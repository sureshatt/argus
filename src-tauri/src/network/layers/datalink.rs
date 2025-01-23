use crate::network::{network_dumper::Context, parsers::{arp, ethernet}};
use pnet::packet::{
        ethernet::{EtherTypes, EthernetPacket},
        Packet,
    };
pub enum DatalinkPacket<'a> {
    Ipv4(EthernetPacket<'a>),
    Ipv6(EthernetPacket<'a>),
    Arp(),
}

pub fn process<'a>(
    packet: &'a [u8],
    context: &'a Context
) -> Result<DatalinkPacket<'a>, String> {
    let frame = ethernet::parse(packet, context)?;

    match frame.get_ethertype() {
        EtherTypes::Ipv4 => Ok(DatalinkPacket::Ipv4(frame)),
        EtherTypes::Ipv6 => Ok(DatalinkPacket::Ipv6(frame)),
        EtherTypes::Arp => {
            let _ = arp::handle(frame.payload(), context);
            Ok(DatalinkPacket::Arp())
        }
        _ => Err(format!("Unsupported Ethertype:{:?}", frame.get_ethertype())),
    }
}
