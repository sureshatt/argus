use super::network_dumper::Context;
pub mod application;
pub mod datalink;
pub mod network;
pub mod transport;

pub fn process_packet(
    packet: &[u8],
    context: &Context,
) -> Result<(), String> {

    let datalink_frame_payload = datalink::process(packet, context)?;

    // only the network layer processing copies & owns the packet data
    let network_packet_payload =
        network::process(&datalink_frame_payload, context)?;

    let transport_segment_payload =
        transport::process(&network_packet_payload, context)?;

    application::process(&transport_segment_payload, context)
}
