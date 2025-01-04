use pnet::packet::arp::ArpPacket;

pub fn handle(packet: &[u8]) -> Result<(), String> {
    ArpPacket::new(packet).unwrap();
    Ok(())
}
