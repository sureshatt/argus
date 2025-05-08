use crate::network::layers::transport::TransportSegment;
use crate::network::network_dumper::Context;
use chrono::Utc;
use hickory_proto::op::Message;
use hickory_proto::serialize::binary::{BinDecodable, BinDecoder};
use serde_json::json;
use tauri::Emitter;

pub fn parse<'a>(packet: &'a TransportSegment, context: &'a Context) -> Result<(), String> {

    let decoder = BinDecoder::new(packet.get_payload());

    match Message::read(&mut decoder.clone(0)) {
        Ok(dns_message) => {

            let dns_json = json!({
                "npid": context.counter.next(),
                "parent": context.parent_counter.to_string(),
                "timestamp": Utc::now().timestamp_millis().to_string(),
                "protocol": "DNS",
                "source": packet.get_source(),
                "destination": packet.get_destination(),
                "length": packet.get_payload().len().to_string(),
                "info": format!("{}({})", dns_message.message_type().to_string(), dns_message.op_code().to_string()),
                "interface": context.interface.name.to_string(),
                "payload": packet.get_payload().to_vec(),
                "header": dns_message.header().to_string(),
                "queries": dns_message.queries().iter().map(|q| q.to_string()).collect::<Vec<_>>(),
                "answers": dns_message.answers().iter().map(|a| a.to_string()).collect::<Vec<_>>(),
                "name_servers": dns_message.name_servers().iter().map(|ns| ns.to_string()).collect::<Vec<_>>(),
                "additionals": dns_message.additionals().iter().map(|a| a.to_string()).collect::<Vec<_>>(),
                "signature": dns_message.signature().iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "),
                "edns": dns_message.extensions().as_ref().map_or("None".to_string(), |edns| format!("{:?}", edns)),
            });

            let _ = context.app_handle.emit("all_logs_event", dns_json);

            Ok(())
        }
        Err(e) => {
            return Err("Failed to parse DNS message: ".to_string() + &e.to_string());
        }
    }
}
