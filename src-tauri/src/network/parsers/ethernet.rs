use pnet::{
    datalink::NetworkInterface,
    packet::{ethernet::EthernetPacket, Packet},
};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::{Counter, NetworkLog};


pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
    counter: &'a Counter,
    parent_counter: &'a String
) -> Result<EthernetPacket<'a>, String> {

    let ethernet_frame = EthernetPacket::new(packet);

    if let Some(ethernet) = ethernet_frame {

        let netlog = NetworkLog {
            id: counter.next(),
            parent: parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "Ethernet".to_string(),
            source: ethernet.get_source().to_string(),
            destination: ethernet.get_destination().to_string(),
            length: ethernet.packet().len().to_string(),
            info: "".to_string(),
            interface: (&interface.name[..]).to_string()
        };

        let _ = app_handle.emit(
            "update",
            netlog,
        );

        Ok(ethernet)
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
