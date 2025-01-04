use pnet::packet::udp::UdpPacket;

pub fn parse(packet: &[u8]) -> Result<UdpPacket, String> {
    if packet.len() < 8 {
        return Err("UDP datagram too short".to_string());
    }

    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        Ok(udp)
    } else {
        return Err("Malformed UDP Packet".to_string());
    }
}
