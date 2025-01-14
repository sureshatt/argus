use crate::network::parsers::{arp, ethernet};
use pnet::{
    datalink::NetworkInterface,
    packet::{ethernet::{EtherTypes, EthernetPacket}, Packet},
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
) -> Result<DatalinkPacket<'a>, String> {
    
    let frame = ethernet::parse(packet, interface, app_handle, db)?;

    match frame.get_ethertype() {
        EtherTypes::Ipv4 => Ok(DatalinkPacket::Ipv4(frame)),
        EtherTypes::Ipv6 => Ok(DatalinkPacket::Ipv6(frame)),
        EtherTypes::Arp => {
            let _ = arp::handle(frame.payload(), interface, app_handle, db);
            Ok(DatalinkPacket::Arp())
        }
        _ => Err(format!("Unsupported Ethertype:{:?}", frame.get_ethertype())),
    }
}
