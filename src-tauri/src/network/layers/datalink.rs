use crate::{network::parsers::{arp, ethernet}, Counter};
use pnet::{
    datalink::NetworkInterface,
    packet::{
        ethernet::{EtherTypes, EthernetPacket},
        Packet,
    },
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;
pub enum DatalinkPacket<'a> {
    Ipv4(EthernetPacket<'a>),
    Ipv6(EthernetPacket<'a>),
    Arp(),
}

pub fn process<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter,
    parent_counter: &'a String,
) -> Result<DatalinkPacket<'a>, String> {
    let frame = ethernet::parse(packet, interface, app_handle, db, counter, parent_counter)?;

    match frame.get_ethertype() {
        EtherTypes::Ipv4 => Ok(DatalinkPacket::Ipv4(frame)),
        EtherTypes::Ipv6 => Ok(DatalinkPacket::Ipv6(frame)),
        EtherTypes::Arp => {
            let _ = arp::handle(frame.payload(), interface, app_handle, db, counter, parent_counter);
            Ok(DatalinkPacket::Arp())
        }
        _ => Err(format!("Unsupported Ethertype:{:?}", frame.get_ethertype())),
    }
}
