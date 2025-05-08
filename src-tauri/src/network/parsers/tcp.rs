use crate::network::{layers::network::IpPacket, network_dumper::Context};
use chrono::Utc;
use pnet::packet::{
    tcp::{TcpOptionPacket, TcpPacket},
    Packet,
};
use serde_json::json;
use tauri::Emitter;

pub fn parse<'a>(packet: &'a IpPacket, context: &'a Context) -> Result<TcpPacket<'a>, String> {
    let tcp = TcpPacket::new(packet.get_payload());

    if let Some(tcp) = tcp {
        let tcp_options_packet = TcpOptionPacket::new(tcp.payload());
        let mut tcp_options_json = json!({});
        tcp_options_packet.map(|tcp_options_packet| {
            tcp_options_json = json!({
                "number": tcp_options_packet.get_number().0.to_string(),
                "length": tcp_options_packet.get_length().to_vec(),
                "data": tcp_options_packet.payload().to_vec()
            })
        });

        let tcp_json = json!({
            "npid": context.counter.next(),
            "parent": context.parent_counter.to_string(),
            "timestamp": Utc::now().timestamp_millis().to_string(),
            "protocol": "TCP",
            "source": format!("{}:{}", packet.get_source_ip(), tcp.get_source()),
            "destination": format!("{}:{}", packet.get_destination_ip(), tcp.get_destination()),
            "length": tcp.packet().len().to_string(),
            "info": "",
            "interface": context.interface.name.to_string(),
            "tcp_source": tcp.get_source().to_string(),
            "tcp_destination": tcp.get_destination().to_string(),
            "sequence_number": tcp.get_sequence().to_string(),
            "acknowledgment_number": tcp.get_acknowledgement().to_string(),
            "data_offset": tcp.get_data_offset().to_string(),
            "reserved": tcp.get_reserved().to_string(),
            "flags": tcp.get_flags().to_string(),
            "window": tcp.get_window().to_string(),
            "checksum": tcp.get_checksum().to_string(),
            "urgent_pointer": tcp.get_urgent_ptr().to_string(),
            "options": tcp_options_json,
            "payload": tcp.payload().to_vec()
        });

        let _ = context.app_handle.emit("all_logs_event", tcp_json);

        Ok(tcp)
    } else {
        return Err("Malformed TCP Packet".to_string());
    }
}
