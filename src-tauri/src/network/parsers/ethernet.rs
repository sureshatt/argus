use pnet::{datalink::NetworkInterface, packet::{ethernet::EthernetPacket, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

pub fn parse<'a>(
    packet: &'a [u8],
    interface: &'a NetworkInterface,
    app_handle: &'a AppHandle,
    db: &'a Surreal<Db>,
) -> Result<EthernetPacket<'a>, String> {

    if packet.len() < 14 {
        return Err("Packet too short".to_string());
    }

    let ethernet_frame = EthernetPacket::new(packet);

    if let Some(ethernet) = ethernet_frame {
        let _ = app_handle.emit(
            "update",
            format!(
                "[{}]: Ethernet frame: {} > {}; ethertype: {:?} length: {}",
                &interface.name[..],
                ethernet.get_source(),
                ethernet.get_destination(),
                ethernet.get_ethertype(),
                ethernet.packet().len()
            ),
        );
        Ok(ethernet)

    } else {
        return Err("Malformed ARP Packet".to_string());
    }

}
