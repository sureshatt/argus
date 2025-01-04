use pnet::packet::ipv4::Ipv4Packet;

pub fn parse(packet: &[u8]) -> Result<Ipv4Packet, String> {
    let ipv4_packet = Ipv4Packet::new(packet);
    if let Some(ipv4_packet) = ipv4_packet {
        Ok(ipv4_packet)
    } else {
        Err("Invalid Ipv4".to_string())
    }
}
