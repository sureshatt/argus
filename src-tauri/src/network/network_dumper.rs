// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This example shows a basic packet logger using libpnet
extern crate pnet;
use crate::network::layers;
use crate::network::network_interface::NetIface;
use crate::{AppState, Counter};
use log::{error, info};
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, State};

pub struct Context<'a> {
    pub interface: &'a NetIface,
    pub app_handle: &'a AppHandle,
    pub counter: &'a Counter,
    pub parent_counter: &'a String,
    pub geo_ip_ranges: &'a Vec<crate::network::ip_utils::IpRange>,
}

#[tauri::command]
pub fn dump(selection: String, app_handle: tauri::AppHandle, state: State<AppState>) {
    info!("selected interface: {}", selection);

    // Find the network interface with the provided name
    let interface = match datalink::interfaces()
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name.to_lowercase() == selection.to_lowercase())
    {
        Some(iface) => iface,
        None => {
            error!("Error: No such network interface: {}", selection);
            return;
        }
    };

    let selected_clone = state.selected.clone();
    let sequence_generator = state.counter.clone();
    let geo_ip_ranges = state.geo_ip_ranges.clone();
    let net_iface = NetIface::from_network_interface(&interface);
    let basic_logs_store_arc = Arc::clone(&state.basic_logs_store);
    let detailed_logs_store_arc = Arc::clone(&state.detailed_logs_store);

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            error!("Unhandled channel type. Only Ethernet is supported.");
            let _ = app_handle.emit("capture_error", "Unhandled channel type. Only Ethernet is supported.");
            return;
        }
        Err(e) => {
            error!("Failed to create datalink channel: {}", e);
            let is_permission_error = e
                .downcast_ref::<std::io::Error>()
                .map(|io_err| io_err.kind() == std::io::ErrorKind::PermissionDenied)
                .unwrap_or(false);
            if is_permission_error {
                let _ = app_handle.emit("bpf_permission_error", ());
            } else {
                let _ = app_handle.emit("capture_error", e.to_string());
            }
            return;
        }
    };

    info!(" **** Starting packet dump on interface: {}", selection);

    thread::spawn(move || loop {
        // this logic kills the thread if the interface changes
        let read_selected = match selected_clone.read() {
            Ok(selected) => selected.clone(),
            Err(e) => {
                error!("Failed to read selected interface: {}", e);
                return;
            }
        };

        if read_selected != "" && read_selected.to_lowercase() != selection.to_lowercase() {
            let mut basic_logs_store = match basic_logs_store_arc.write() {
                Ok(store) => store,
                Err(e) => {
                    error!("Failed to acquire write lock for basic_logs_store: {}", e);
                    return;
                }
            };
            let mut detailed_logs_store = match detailed_logs_store_arc.write() {
                Ok(store) => store,
                Err(e) => {
                    error!(
                        "Failed to acquire write lock for detailed_logs_store: {}",
                        e
                    );
                    return;
                }
            };
            basic_logs_store.clear();
            detailed_logs_store.clear();
            info!(
                "Quitting the thread for: {} & clearing cache {} {}",
                selection,
                basic_logs_store.len(),
                detailed_logs_store.len()
            );
            return;
        }

        let counter = match sequence_generator.read() {
            Ok(counter) => counter,
            Err(e) => {
                error!("Failed to acquire read lock for sequence_generator: {}", e);
                return;
            }
        };
        let geo_ip_ranges = match geo_ip_ranges.read() {
            Ok(ranges) => ranges,
            Err(e) => {
                error!("Failed to acquire read lock for geo_ip_ranges: {}", e);
                return;
            }
        };

        match rx.next() {
            Ok(packet) => {
                let parent_counter = counter.next();
                let context = Context {
                    interface: &net_iface,
                    app_handle: &app_handle,
                    counter: &counter,
                    parent_counter: &parent_counter,
                    geo_ip_ranges: &geo_ip_ranges,
                };

                let _ = layers::process_packet(packet, &context);
                thread::sleep(Duration::from_millis(500));
            }
            Err(e) => {
                error!("packetdump: unable to receive packet: {}", e);
                return;
            }
        }
    });
}
