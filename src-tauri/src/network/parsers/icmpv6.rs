use crate::network::network_dumper::Context;
use chrono::Utc;
use pnet::packet::{
    icmpv6::{
        echo_reply::EchoReplyPacket, echo_request::EchoRequestPacket, Icmpv6Packet, Icmpv6Types,
    }, ipv6::Ipv6Packet, Packet
};
use serde_json::json;
use tauri::Emitter;

#[derive(Debug)]
enum Icmpv6PacketType {
    EchoReply,
    DestinationUnreachable,
    PacketTooBig,
    TimeExceeded,
    ParameterProblem,
    EchoRequest,
    RouterSolicitation,
    RouterAdvertisement,
    NeighborSolicitation,
    NeighborAdvertisement,
    Redirect,
    RouterRenumbering,
    NodeInformationQuery,
    NodeInformationResponse,
    InverseNeighborDiscoverySolicitation,
    InverseNeighborDiscoveryAdvertisement,
    Version2MulticastListenerReport,
    HomeAgentAddressDiscoveryRequest,
    HomeAgentAddressDiscoveryReply,
    MobilePrefixSolicitation,
    MobilePrefixAdvertisement,
    CertificationPathSolicitation,
    CertificationPathAdvertisement,
    MulticastRouterAdvertisement,
    MulticastRouterSolicitation,
    MulticastRouterTermination,
    FMIPv6Messages,
    RPLControlMessage,
    ILNPv6LocatorUpdateMessage,
    DuplicateAddressRequest,
    DuplicateAddressConfirmation,
    CertificationPathAdvertisementAck,
    HandoverKeyMessage,
    MobileNodeIdentifier,
    RPLMessage,
    Unknown,
}

impl From<u8> for Icmpv6PacketType {
    fn from(value: u8) -> Self {
        match value {
            1 => Icmpv6PacketType::DestinationUnreachable,
            2 => Icmpv6PacketType::PacketTooBig,
            3 => Icmpv6PacketType::TimeExceeded,
            4 => Icmpv6PacketType::ParameterProblem,
            128 => Icmpv6PacketType::EchoRequest,
            129 => Icmpv6PacketType::EchoReply,
            133 => Icmpv6PacketType::RouterSolicitation,
            134 => Icmpv6PacketType::RouterAdvertisement,
            135 => Icmpv6PacketType::NeighborSolicitation,
            136 => Icmpv6PacketType::NeighborAdvertisement,
            137 => Icmpv6PacketType::Redirect,
            138 => Icmpv6PacketType::RouterRenumbering,
            139 => Icmpv6PacketType::NodeInformationQuery,
            140 => Icmpv6PacketType::NodeInformationResponse,
            141 => Icmpv6PacketType::InverseNeighborDiscoverySolicitation,
            142 => Icmpv6PacketType::InverseNeighborDiscoveryAdvertisement,
            143 => Icmpv6PacketType::Version2MulticastListenerReport,
            144 => Icmpv6PacketType::HomeAgentAddressDiscoveryRequest,
            145 => Icmpv6PacketType::HomeAgentAddressDiscoveryReply,
            146 => Icmpv6PacketType::MobilePrefixSolicitation,
            147 => Icmpv6PacketType::MobilePrefixAdvertisement,
            148 => Icmpv6PacketType::CertificationPathSolicitation,
            149 => Icmpv6PacketType::CertificationPathAdvertisement,
            151 => Icmpv6PacketType::MulticastRouterAdvertisement,
            152 => Icmpv6PacketType::MulticastRouterSolicitation,
            153 => Icmpv6PacketType::MulticastRouterTermination,
            154 => Icmpv6PacketType::FMIPv6Messages,
            155 => Icmpv6PacketType::RPLControlMessage,
            156 => Icmpv6PacketType::ILNPv6LocatorUpdateMessage,
            157 => Icmpv6PacketType::DuplicateAddressRequest,
            158 => Icmpv6PacketType::DuplicateAddressConfirmation,
            159 => Icmpv6PacketType::CertificationPathAdvertisementAck,
            160 => Icmpv6PacketType::HandoverKeyMessage,
            161 => Icmpv6PacketType::MobileNodeIdentifier,
            162 => Icmpv6PacketType::RPLMessage,
            _ => Icmpv6PacketType::Unknown,
        }
    }
}

impl ToString for Icmpv6PacketType {
    fn to_string(&self) -> String {
        match self {
            Icmpv6PacketType::EchoReply => "Echo Reply",
            Icmpv6PacketType::DestinationUnreachable => "Destination Unreachable",
            Icmpv6PacketType::PacketTooBig => "Packet Too Big",
            Icmpv6PacketType::TimeExceeded => "Time Exceeded",
            Icmpv6PacketType::ParameterProblem => "Parameter Problem",
            Icmpv6PacketType::EchoRequest => "Echo Request",
            Icmpv6PacketType::RouterSolicitation => "Router Solicitation",
            Icmpv6PacketType::RouterAdvertisement => "Router Advertisement",
            Icmpv6PacketType::NeighborSolicitation => "Neighbor Solicitation",
            Icmpv6PacketType::NeighborAdvertisement => "Neighbor Advertisement",
            Icmpv6PacketType::Redirect => "Redirect",
            Icmpv6PacketType::RouterRenumbering => "Router Renumbering",
            Icmpv6PacketType::NodeInformationQuery => "Node Information Query",
            Icmpv6PacketType::NodeInformationResponse => "Node Information Response",
            Icmpv6PacketType::InverseNeighborDiscoverySolicitation => {
                "Inverse Neighbor Discovery Solicitation"
            }
            Icmpv6PacketType::InverseNeighborDiscoveryAdvertisement => {
                "Inverse Neighbor Discovery Advertisement"
            }
            Icmpv6PacketType::Version2MulticastListenerReport => {
                "Version 2 Multicast Listener Report"
            }
            Icmpv6PacketType::HomeAgentAddressDiscoveryRequest => {
                "Home Agent Address Discovery Request"
            }
            Icmpv6PacketType::HomeAgentAddressDiscoveryReply => {
                "Home Agent Address Discovery Reply"
            }
            Icmpv6PacketType::MobilePrefixSolicitation => "Mobile Prefix Solicitation",
            Icmpv6PacketType::MobilePrefixAdvertisement => "Mobile Prefix Advertisement",
            Icmpv6PacketType::CertificationPathSolicitation => "Certification Path Solicitation",
            Icmpv6PacketType::CertificationPathAdvertisement => "Certification Path Advertisement",
            Icmpv6PacketType::MulticastRouterAdvertisement => "Multicast Router Advertisement",
            Icmpv6PacketType::MulticastRouterSolicitation => "Multicast Router Solicitation",
            Icmpv6PacketType::MulticastRouterTermination => "Multicast Router Termination",
            Icmpv6PacketType::FMIPv6Messages => "FMIPv6 Messages",
            Icmpv6PacketType::RPLControlMessage => "RPL Control Message",
            Icmpv6PacketType::ILNPv6LocatorUpdateMessage => "ILNPv6 Locator Update Message",
            Icmpv6PacketType::DuplicateAddressRequest => "Duplicate Address Request",
            Icmpv6PacketType::DuplicateAddressConfirmation => "Duplicate Address Confirmation",
            Icmpv6PacketType::CertificationPathAdvertisementAck => {
                "Certification Path Advertisement Ack"
            }
            Icmpv6PacketType::HandoverKeyMessage => "Handover Key Message",
            Icmpv6PacketType::MobileNodeIdentifier => "Mobile Node Identifier",
            Icmpv6PacketType::RPLMessage => "RPL Message",
            Icmpv6PacketType::Unknown => "Unknown", 
    }.to_string()
}
}

pub fn parse(ipv6_packet: &Ipv6Packet, context: &Context) -> Result<(), String> {
    let icmpv6_packet = Icmpv6Packet::new(ipv6_packet.payload());
    let source = ipv6_packet.get_source();
    let destination = ipv6_packet.get_destination();

    if let Some(icmpv6_packet) = icmpv6_packet {
        match icmpv6_packet.get_icmpv6_type() {
            Icmpv6Types::EchoReply => {
                let icmpv6_echo_reply = EchoReplyPacket::new(icmpv6_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMPv6",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": "ICMP Echo Reply",
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_echo_reply.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_echo_reply.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_echo_reply.get_checksum().to_string(),
                    "identifier": icmpv6_echo_reply.get_identifier().to_string(),
                    "sequence_number": icmpv6_echo_reply.get_sequence_number().to_string(),
                    "payload": icmpv6_echo_reply.payload().to_vec()
                });

                let _ = context.app_handle.emit("all_logs_event", icmp_json);
            }
            Icmpv6Types::EchoRequest => {
                let icmpv6_echo_request = EchoRequestPacket::new(icmpv6_packet.packet()).unwrap();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMPv6",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": "ICMP Echo Request",
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_echo_request.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_echo_request.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_echo_request.get_checksum().to_string(),
                    "identifier": icmpv6_echo_request.get_identifier().to_string(),
                    "sequence_number": icmpv6_echo_request.get_sequence_number().to_string(),
                    "payload": icmpv6_echo_request.payload().to_vec()
                });

                let _ = context.app_handle.emit("all_logs_event", icmp_json);
            }
            _ => {

                let icmpv6_type: Icmpv6PacketType = icmpv6_packet.get_icmpv6_type().0.into();

                let icmp_json = json!({
                    "npid": context.counter.next(),
                    "parent": context.parent_counter.to_string(),
                    "timestamp": Utc::now().timestamp_millis().to_string(),
                    "protocol": "ICMPv6",
                    "source": source.to_string(),
                    "destination": destination.to_string(),
                    "length": icmpv6_packet.packet().len().to_string(),
                    "info": icmpv6_type.to_string(),
                    "interface": (context.interface.name[..]).to_string(),
                    "icmp_type": icmpv6_packet.get_icmpv6_type().0.to_string(),
                    "icmp_code": icmpv6_packet.get_icmpv6_code().0.to_string(),
                    "checksum": icmpv6_packet.get_checksum().to_string(),
                    "payload": icmpv6_packet.payload().to_vec()
                });

                let _ = context.app_handle.emit("all_logs_event", icmp_json);
            }
        }
    } else {
        println!(
            "[{}]: Malformed ICMPv6 Packet",
            context.interface.name[..].to_string()
        );
    }

    Ok(())
}
