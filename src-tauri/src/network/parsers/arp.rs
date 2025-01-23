use pnet::{datalink::NetworkInterface, packet::{arp::ArpPacket, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::{Counter, NetworkLog};

pub fn handle(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
    counter: &Counter,
    parent_counter: &String
) -> Result<(), String> {
    let arp_frame = ArpPacket::new(packet);

    if let Some(arp) = arp_frame {

        let netlog = NetworkLog {
            id: counter.next(),
            parent: parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "ARP".to_string(),
            source: format!("{} ({})", arp.get_sender_hw_addr().to_string(),arp.get_sender_proto_addr()),
            destination: format!("{} ({})", arp.get_target_hw_addr().to_string(),arp.get_target_proto_addr()),
            length: arp.packet().len().to_string(),
            info: "".to_string(),
            interface: (&interface.name[..]).to_string()
        };

        let _ = app_handle.emit(
            "update",
            netlog,
        );

        Ok(())
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
