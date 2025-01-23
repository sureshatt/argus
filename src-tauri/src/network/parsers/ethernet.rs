use pnet::packet::{ethernet::EthernetPacket, Packet};
use tauri::Emitter;
use chrono::Utc;
use crate::{network::network_dumper::Context, NetworkLog};

pub fn parse<'a>(
    packet: &'a [u8],
    context: &'a Context,
) -> Result<EthernetPacket<'a>, String> {

    let ethernet_frame = EthernetPacket::new(packet);

    if let Some(ethernet) = ethernet_frame {

        let netlog = NetworkLog {
            id: context.counter.next(),
            parent: context.parent_counter.to_string(),
            timestamp:  Utc::now().timestamp_millis().to_string(),
            protocol: "Ethernet".to_string(),
            source: ethernet.get_source().to_string(),
            destination: ethernet.get_destination().to_string(),
            length: ethernet.packet().len().to_string(),
            info: "".to_string(),
            interface: (context.interface.name[..]).to_string()
        };

        let _ = context.app_handle.emit(
            "update",
            netlog,
        );

        Ok(ethernet)
    } else {
        return Err("Malformed ARP Packet".to_string());
    }
}
