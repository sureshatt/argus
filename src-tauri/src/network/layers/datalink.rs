use crate::network::parsers::{arp, ethernet};
use pnet::{
    datalink::NetworkInterface,
    packet::{ethernet::EtherTypes, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;
pub enum DatalinkPacketPayload {
    Ipv4(Vec<u8>),
    Ipv6(Vec<u8>),
    Arp(),
}

pub fn process(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<DatalinkPacketPayload, String> {
    let frame = ethernet::parse(packet, interface, app_handle, db)?;

    match frame.get_ethertype() {
        EtherTypes::Ipv4 => Ok(DatalinkPacketPayload::Ipv4(frame.payload().to_owned())),
        EtherTypes::Ipv6 => Ok(DatalinkPacketPayload::Ipv6(frame.payload().to_owned())),
        EtherTypes::Arp => {
            let _ = arp::handle(frame.payload(), interface, app_handle, db);
            Ok(DatalinkPacketPayload::Arp())
        }
        _ => Err(format!("Unsupported Ethertype:{:?}", frame.get_ethertype())),
    }
}
