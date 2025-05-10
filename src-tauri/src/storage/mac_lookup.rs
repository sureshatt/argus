use std::collections::HashMap;

pub struct Device {
    pub mac: String,
    pub ip_address: String,
    pub vendor: String,
    pub device_type: String,
}

impl Device {
    pub fn new(mac: String, ip_address: String) -> Self {
        let vendor = lookup_vendor(&mac).unwrap_or("Unknown Vendor").to_string();
        let device_type = guess_device_type(&vendor).to_string();
        Self {
            mac,
            ip_address,
            vendor,
            device_type,
        }
    }
    
}

/// Normalize MAC prefix and return vendor if known.
pub(crate) fn lookup_vendor(mac: &str) -> Option<&'static str> {
    let mac_prefix = mac.to_lowercase().replace(":", "")[..6].to_string();
    let vendor_map: HashMap<&str, &str> = [
        ("3ca62f", "Foxconn (Apple devices)"),
        ("a4b121", "Samsung Electronics"),
        ("f4ec38", "TP-Link Technologies"),
        ("b827eb", "Raspberry Pi Foundation"),
        ("fcfc48", "Dell Inc."),
        ("001e65", "Cisco Systems"),
    ].iter().cloned().collect();

    vendor_map.get(mac_prefix.as_str()).copied()
}

/// Heuristically determine device type based on vendor.
pub(crate) fn guess_device_type(vendor: &str) -> &'static str {
    match vendor.to_lowercase().as_str() {
        v if v.contains("apple") => "Laptop or Mobile",
        v if v.contains("samsung") => "Mobile Phone",
        v if v.contains("tp-link") => "Router",
        v if v.contains("raspberry") => "IoT or Hobbyist Device",
        v if v.contains("dell") => "Laptop or Desktop",
        v if v.contains("cisco") => "Router or Switch",
        _ => "Unknown",
    }
}