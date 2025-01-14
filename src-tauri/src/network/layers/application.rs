use pnet::{datalink::NetworkInterface, packet::{dns::Dns, Packet}};
use surrealdb::{engine::local::Db, Surreal};
use tauri::{AppHandle, Emitter};

use crate::network::layers::transport::TransportSegmentPayload;

pub fn process(
    segment: &TransportSegmentPayload,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    Ok(())
}
