use crate::network::{layers::transport::TransportSegmentPayload, network_dumper::Context};

pub fn process(
    segment: &TransportSegmentPayload,
    context: &Context,
) -> Result<(), String> {
    match segment {
        TransportSegmentPayload::Dns(udp) => {
                        let _ = crate::network::parsers::dns::parse(udp, context);
                        Ok(())
            }
       _ => {
            Ok(())
        }
    }
}
