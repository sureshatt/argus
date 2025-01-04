use pnet::packet::ethernet::EthernetPacket;

pub fn parse(packet: &[u8]) -> Result<EthernetPacket, String> {
    if packet.len() < 14 {
        return Err("Packet too short".to_string());
    }

    Ok(EthernetPacket::new(packet).unwrap())
}
