export type NetworkPacket = {
    id: string;
    timestamp: string;
    protocol: string;
    source: string;
    destination: string;
    length: number;
    info: string;
};

export type NetworkInterface = {
    name: string;
    mac: string;
    ipv4_address: string;
    ipv6_addresses: string[];
    is_broadcast: boolean
    is_multicast: boolean
    is_p2p: boolean
    is_running: true
    is_up: boolean
};

// Base type with common fields
export interface BasePacket {
    npid: number;
    parent: string;
    timestamp: string;
    protocol: string;
    source: string;
    destination: string;
    length: string;
    info: string;
    interface: string;
    payload: number[];
}

// Ethernet-specific type
export interface EthernetPacket extends BasePacket {
    ethernet_type: string;
}

// IPv4-specific type
export interface IPv4Packet extends BasePacket {
    version: string;
    header_length: string;
    dscp: string;
    ecn: string;
    total_length: string;
    identification: string;
    flags: string;
    fragment_offset: string;
    ttl: string;
    next_level_protocol: string;
    checksum: string;
    source_ip: string;
    destination_ip: string;
    options: string;
}

// TCP-specific type
export interface TCPPacket extends BasePacket {
    tcp_source: string;
    tcp_destination: string;
    sequence_number: string;
    acknowledgment_number: string;
    data_offset: string;
    reserved: string;
    flags: string;
    window: string;
    checksum: string;
    urgent_pointer: string;
    options: string;
}

// DNS-specific type
export interface DNSPacket extends BasePacket {
    header: string;
    queries: string;
    answers: string;
    name_servers: string;
    additionals: string;
    signature: string;
    edns: string;
}

// IPv6-specific type
export interface IPv6Packet extends BasePacket {
    version: string;
    traffic_class: string;
    flow_label: string;
    payload_length: string;
    next_header: string;
    hop_limit: string;
    source_ip: string;
    destination_ip: string;
}

// ARP-specific type
export interface ARPPacket extends BasePacket {
    hardware_type: string;
    protocol_type: string;
    hardware_addr_length: string;
    protocol_addr_length: string;
    operation: string;
    sender_hw_addr: string;
    sender_proto_addr: string;
    target_hw_addr: string;
    target_proto_addr: string;
}


// Union type for any packet
export type Packet = EthernetPacket | IPv4Packet | TCPPacket | DNSPacket | IPv6Packet | ARPPacket;


export type BarCharData = {
    labels: string[];
    datasets: {
        label?: string;
        data: number[];
        backgroundColor?: string | string[];
        borderRadius?: {
            bottomRight?: number;
            topRight?: number;
            topLeft?: number;
            bottomLeft?: number;
        };
        barThickness?: number;
    }[];
}

export type CountryTraffic = {
    country: string,
    count: number
}

export type AlertData = {
    value: string,
    show: boolean
}

export type NetworkStat = {
    protocol_stats: ProtocolStat[],
    ingress_ip_stats: IpStat[],
    egress_ip_stats: IpStat[],
    country_stats: CountryStat[],
    arp_stats: ArpStat[],

}

export type ProtocolStat = {
    count: number,
    protocol: string
}

export type IpStat = {
    ip: string,
    count: number
}

export type CountryStat = {
    country: string
}

export type ArpStat = {
    source_mac: string,
    source_ip: string
}