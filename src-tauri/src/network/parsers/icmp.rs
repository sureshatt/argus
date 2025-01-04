use pnet::{datalink::NetworkInterface, packet::icmp::IcmpPacket};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {

    let _icmp_packet = IcmpPacket::new(packet);
    Ok(())
    
}
