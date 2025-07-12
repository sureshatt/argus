use ipnetwork::IpNetwork;
use pnet::ipnetwork;
use serde::Deserialize;
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub const LOCAL_ORIGIN: &str = "local";
pub const UNKNOWN_ORIGIN: &str = "unknown";
const CSV_DATA: &str = include_str!("../../assets/data.csv"); // https://db-ip.com/db/

pub fn ip_in_any_cidr(ip_str: &str, cidr_list: &[String]) -> bool {
    let ip: IpAddr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => return false,
    };

    for cidr_str in cidr_list {
        if let Ok(cidr) = cidr_str.parse::<IpNetwork>() {
            if cidr.contains(ip) {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Deserialize)]
struct RawIpRange {
    start_ip: String,
    end_ip: String,
    country: String,
}

#[derive(Debug)]
pub struct IpRange {
    start: u32,
    end: u32,
    country: String,
}

fn ip_to_u32(ip: &str) -> Option<u32> {
    Ipv4Addr::from_str(ip).ok().map(u32::from)
}

pub fn load_ip_ranges() -> Result<Vec<IpRange>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(CSV_DATA.as_bytes());
    let mut ranges = Vec::new();

    for result in rdr.deserialize() {
        let raw: RawIpRange = result?;
        if let (Some(start), Some(end)) = (ip_to_u32(&raw.start_ip), ip_to_u32(&raw.end_ip)) {
            ranges.push(IpRange {
                start,
                end,
                country: raw.country,
            });
        }
    }

    // Sort ranges by start IP for binary search
    ranges.sort_by_key(|r| r.start);
    Ok(ranges)
}

pub fn lookup_country<'a>(ip_str: &str, ranges: &'a [IpRange]) -> Option<&'a str> {
    let ip = ip_to_u32(ip_str)?;

    // Binary search for the IP range that contains the IP
    let mut low = 0;
    let mut high = ranges.len();

    while low < high {
        let mid = (low + high) / 2;
        let range = &ranges[mid];

        if ip < range.start {
            high = mid;
        } else if ip > range.end {
            low = mid + 1;
        } else {
            return Some(&range.country);
        }
    }

    None
}

pub fn get_ip_origin<'a>(ip_str: &str, cidr_list: &[String], ranges: &'a [IpRange]) -> String {
    if ip_in_any_cidr(ip_str, cidr_list) {
        return LOCAL_ORIGIN.to_string();
    }

    match lookup_country(ip_str, ranges).map(|country| country.to_string()) {
        Some(country) => country,
        None => UNKNOWN_ORIGIN.to_string(),
    }
}
