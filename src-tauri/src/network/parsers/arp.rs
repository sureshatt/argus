use pnet::{datalink::NetworkInterface, packet::arp::ArpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};
use chrono::Utc;

use crate::Counter;

pub fn handle(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
    conter: &Counter,
    parent_counter: &String
) -> Result<(), String> {
    let arp_frame = ArpPacket::new(packet);

    if let Some(arp) = arp_frame {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: {} {} {} ARP packet: {}({}) > {}({}); operation: {:?}",
                interface,
                parent_counter,
                conter.next(),
                Utc::now().timestamp_millis(),
                arp.get_sender_hw_addr(),
                arp.get_sender_proto_addr(),
                arp.get_target_hw_addr(),
                arp.get_target_proto_addr(),
                arp.get_operation()
            ),
        );
        Ok(())
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
