use pnet::packet::icmp::IcmpPacket;

pub fn parse(packet: &[u8]) -> Result<(), String> {
    let _icmp_packet = IcmpPacket::new(packet);
    Ok(())
}
