use pnet::packet::ipv6::Ipv6Packet;

pub fn parse(packet: &[u8]) -> Result<Ipv6Packet, String> {
    let ipv6_packet = Ipv6Packet::new(packet);
    if let Some(ipv6_packet) = ipv6_packet {
        Ok(ipv6_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
