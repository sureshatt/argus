use pnet::packet::{ethernet::EthernetPacket, Packet};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use chrono::Utc;
use crate::network::network_dumper::Context;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EthernetPacketLog {
    pub npid: String,
    pub parent: String,
    pub timestamp: String,
    pub protocol: String,
    pub interface: String,
    pub length: String,
    pub info: String,
    pub source: String,
    pub destination: String,
    pub ethernet_type: String,
    pub payload: Vec<u8>,
}

pub fn parse<'a>(
    packet: &'a [u8],
    context: &'a Context,
) -> Result<EthernetPacket<'a>, String> {

    let ethernet_frame = EthernetPacket::new(packet);

    if let Some(ethernet) = ethernet_frame {

        let ethernet_payload = EthernetPacketLog {
            npid: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "Ethernet".to_string(),
            source: ethernet.get_source().to_string(),
            destination: ethernet.get_destination().to_string(),
            length: ethernet.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string(),
            ethernet_type: ethernet.get_ethertype().to_string(),
            payload: ethernet.payload().to_vec(),
        };

        let _ = context.app_handle.emit(
            "update",
            ethernet_payload,
        );

        Ok(ethernet)
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
