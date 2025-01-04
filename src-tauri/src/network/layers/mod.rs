pub mod datalink;
pub mod network;
pub mod transport;
pub mod application;


pub fn process_packet(packet: &[u8]) -> Result<(), String> {
    let datalink_frame_payload = datalink::process(packet)?;
    let network_packet_payload = network::process(&datalink_frame_payload)?;
    let transport_segment_payload = transport::process(&network_packet_payload)?;
    application::process(&transport_segment_payload)
}
