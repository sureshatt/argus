use pnet::packet::tcp::TcpPacket;

pub fn parse(packet: &[u8]) -> Result<TcpPacket, String> {
    if packet.len() < 20 {
        return Err("TCP segment too short".to_string());
    }
    let data_offset = (packet[12] >> 4) * 4;

    if packet.len() < data_offset as usize {
        return Err("TCP header length exceeds packet size".to_string());
    }

    let tcp = TcpPacket::new(packet);

    if let Some(tcp) = tcp {
        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
