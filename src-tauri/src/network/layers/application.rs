use pnet::datalink::NetworkInterface;
use surrealdb::{engine::local::Db, Surreal};
use tauri::AppHandle;

use crate::network::layers::transport::TransportSegmentPayload;

pub fn process(
    segment: &TransportSegmentPayload,
    interface: &NetworkInterface,
    app_handle: &AppHandle,
    db: &Surreal<Db>,
) -> Result<(), String> {
    Ok(())
}
