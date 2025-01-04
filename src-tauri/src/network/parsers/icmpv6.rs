use pnet::packet::icmpv6::Icmpv6Packet;

pub fn parse(packet: &[u8]) -> Result<(), String> {
    let _icmp_packet = Icmpv6Packet::new(packet);
    Ok(())
}
