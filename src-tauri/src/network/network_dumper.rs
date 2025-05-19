// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// This example shows a basic packet logger using libpnet
extern crate pnet;

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use std::thread;
use std::time::Duration;
use tauri::{State, AppHandle};
use crate::network::network_interface::NetIface;
use crate::{AppState, Counter};
use crate::network::layers;

pub struct Context<'a> {
    pub interface: &'a NetIface,
    pub app_handle: &'a AppHandle,
    pub counter: &'a Counter,
    pub parent_counter: &'a String,
    pub geo_ip_ranges: &'a Vec<crate::network::ip_utils::IpRange>,
}

#[tauri::command]
pub fn dump(selection: String, app_handle: tauri::AppHandle, state: State<AppState>) {
    println!("selected interface: {}", selection);

    // Find the network interface with the provided name
    let interface = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == selection)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", selection));

    let selected_clone = state.selected.clone();
    let sequence_generator = state.counter.clone();
    let geo_ip_ranges = state.geo_ip_ranges.clone();
    let net_iface = NetIface::from_network_interface(&interface); 

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    thread::spawn(move || loop {

        // this logic kills the thread if the interface changes
        let read_selected = selected_clone.read().unwrap().clone();
        if read_selected != "" && read_selected!= selection {
            println!("Quitting the thread for: {}", selection);
            return ;
        }

        let counter = sequence_generator.read().unwrap();
        let geo_ip_ranges = geo_ip_ranges.read().unwrap();

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
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    });
}