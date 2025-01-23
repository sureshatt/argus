use crate::network::{layers::transport::TransportSegmentPayload, network_dumper::Context};

pub fn process(
    segment: &TransportSegmentPayload,
    context: &Context,
) -> Result<(), String> {
    Ok(())
}
