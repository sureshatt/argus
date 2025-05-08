use crate::network::network_dumper::Context;
use chrono::Utc;
use pnet::packet::{ethernet::EthernetPacket, Packet};
use serde_json::json;
use tauri::Emitter;

pub fn parse<'a>(packet: &'a [u8], context: &'a Context) -> Result<EthernetPacket<'a>, String> {
    let ethernet_frame = EthernetPacket::new(packet);

    if let Some(ethernet) = ethernet_frame {
        let ethernet_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "Ethernet",
            "source": ethernet.get_source().to_string(),
            "destination": ethernet.get_destination().to_string(),
            "length": ethernet.packet().len().to_string(),
            "info": "",
            "interface": context.interface.name.to_string(),
            "ethernet_type": ethernet.get_ethertype().to_string(),
            "payload": ethernet.payload().to_vec()
        });

        let _ = context.app_handle.emit("all_logs_event", ethernet_json);

        Ok(ethernet)
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
