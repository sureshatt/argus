use pnet::{datalink::NetworkInterface, packet::icmpv6::Icmpv6Packet};
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

pub fn parse(
    packet: &[u8],
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    let _icmp_packet = Icmpv6Packet::new(packet);
    Ok(())
}
