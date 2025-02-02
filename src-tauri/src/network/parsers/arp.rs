use crate::network::network_dumper::Context;
use chrono::Utc;
use pnet::packet::{arp::ArpPacket, Packet};
use tauri::Emitter;

pub fn handle(packet: &[u8], context: &Context) -> Result<(), String> {
    let arp_frame = ArpPacket::new(packet);

    if let Some(arp) = arp_frame {

        let arp_info = match arp.get_operation().0 {
            1 => "ARP Request",
            2 => "ARP Reply",
            _ => "Unknown",
        };

        use serde_json::json;

        let arp_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "ARP",
            "source": arp.get_sender_proto_addr().to_string(),
            "destination": arp.get_target_proto_addr().to_string(),
            "length": arp.packet().len().to_string(),
            "info": arp_info,
            "interface": context.interface.name.to_string(),
            "hardware_type": arp.get_hardware_type().0.to_string(),
            "protocol_type": arp.get_protocol_type().to_string(),
            "hardware_addr_length": arp.get_hw_addr_len().to_string(),
            "protocol_addr_length": arp.get_proto_addr_len().to_string(),
            "operation": arp.get_operation().0.to_string(),
            "sender_hw_addr": arp.get_sender_hw_addr().to_string(),
            "sender_proto_addr": arp.get_sender_proto_addr().to_string(),
            "target_hw_addr": arp.get_target_hw_addr().to_string(),
            "target_proto_addr": arp.get_target_proto_addr().to_string(),
            "payload": arp.payload().to_vec()
        });

        let _ = context.app_handle.emit("update", arp_json);

        Ok(())
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
