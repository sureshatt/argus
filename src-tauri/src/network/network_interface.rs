use pnet::{datalink::NetworkInterface, ipnetwork::IpNetwork};
use serde::Serialize;

#[derive(Serialize)]
pub struct NetIface {
    name: String,
    mac: String,
    ipv4_address: String,
    ipv6_addresses: Vec<String>,
    is_up: bool,
    is_running: bool,
    is_loopback: bool,
    is_broadcast: bool,
    is_multicast: bool,
    is_p2p: bool
}

impl NetIface {
    pub fn has_ipv4(&self) -> bool {
        if self.ipv4_address.is_empty() {
            return false;
        } else {
            return true;
        }
    }
}

pub fn get_net_ifaces() -> Vec<NetIface> {

    let mut netface_list: Vec<NetIface> = Vec::new();

    let interfaces = pnet::datalink::interfaces();    

    for intfce in &interfaces {

        let netface = NetIface {
            name: intfce.name.clone(),
            mac: get_mac_address(intfce),
            ipv4_address: get_ipv4_address(&intfce.ips),
            ipv6_addresses: get_ipv6_address(&intfce.ips),
            is_up: intfce.is_up(),
            is_running: intfce.is_running(),
            is_loopback: intfce.is_loopback(),
            is_broadcast: intfce.is_broadcast(),
            is_multicast: intfce.is_multicast(),
            is_p2p: intfce.is_point_to_point()
        };

        netface_list.push(netface);

    }

    return netface_list;
    
}

fn get_mac_address(interface: &NetworkInterface) -> String {
    interface
    .mac
    .map(|mac| mac.to_string())
    .unwrap_or("".to_owned())
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