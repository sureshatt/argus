use pnet::{datalink::NetworkInterface, ipnetwork::IpNetwork};
use serde::{Deserialize, Serialize};
use log::{error, info};

#[derive(Debug,Deserialize, Serialize)]
pub struct NetIface {
    pub name: String,
    pub mac: String,
    pub ipv4_address: String,
    pub ipv6_addresses: Vec<String>,
    pub is_up: bool,
    pub is_running: bool,
    pub is_loopback: bool,
    pub is_broadcast: bool,
    pub is_multicast: bool,
    pub is_p2p: bool
}

impl NetIface {
    pub fn has_ipv4(&self) -> bool {
        if self.ipv4_address.is_empty() {
            return false;
        } else {
            return true;
        }
    }

    pub fn from_network_interface(interface: &NetworkInterface) -> Self {
        NetIface {
            name: interface.name.clone(),
            mac: get_mac_address(interface),
            ipv4_address: get_ipv4_address(&interface.ips),
            ipv6_addresses: get_ipv6_address(&interface.ips),
            is_up: interface.is_up(),
            is_running: interface.is_running(),
            is_loopback: interface.is_loopback(),
            is_broadcast: interface.is_broadcast(),
            is_multicast: interface.is_multicast(),
            is_p2p: interface.is_point_to_point(),
        }
    }
}

pub fn get_net_ifaces() -> Vec<NetIface> {
    let mut netface_list: Vec<NetIface> = Vec::new();
    let interfaces = pnet::datalink::interfaces();    

    for intfce in &interfaces {
        let netface = NetIface::from_network_interface(intfce);
        netface_list.push(netface);
    }

    if netface_list.is_empty() {
        error!("No network interfaces found");
    } else {
        info!("Found {} network interfaces", netface_list.len());
    }

    return netface_list;
}

pub fn get_net_iface_by_name(name: &str) -> Option<NetIface> {
    let interfaces = pnet::datalink::interfaces();
    let netface = interfaces.iter().find(|iface| iface.name == name);
    match netface {
        Some(interface) => Some(NetIface::from_network_interface(interface)),
        None => None,
    }
}

fn get_mac_address(interface: &NetworkInterface) -> String {
    match interface.mac {
        Some(mac) => mac.to_string(),
        None => {
            error!("MAC address not found for interface: {}", interface.name);
            String::new()
        }
    }
}

fn get_ipv4_address(ips: &Vec<IpNetwork>) -> String {
    if ips.len() < 1 {
        return "".to_string();
    } else {
        return ips.iter().find(|ip| ip.is_ipv4()).map_or(String::new(), |ip| ip.to_string());
    }
}

fn get_ipv6_address(ips: &Vec<IpNetwork>) -> Vec<String> {
    if ips.len() < 1 {
        return Vec::new();
    } else {
        return ips.iter().filter(|ip| ip.is_ipv6()).map(|ip| ip.to_string()).collect::<Vec<String>>();
    }
}