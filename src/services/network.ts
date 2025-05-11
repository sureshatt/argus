import { invoke } from "@tauri-apps/api/core";
import { ArpStat, CountryTraffic, IpStat, NetworkInterface, Packet, ProtocolStat } from "../types";


async function getAvailableNetworkInterfaces() {
    return await invoke<NetworkInterface[]>("get_network_interfaces")
}

async function getProtocolsStats(netIface: string) {
    return await invoke<ProtocolStat[]>("get_protocol_stats", { netIface })
}

async function getNetworkLogs(selection: string) {
    return await invoke<Packet[]>("dump", { selection })
}

async function stopNetworkLogs() {
    return await invoke("stop_dump")
}

async function getArpIpStats(netIface: NetworkInterface) {
    return await invoke<ArpStat[]>("get_arp_ip_stats", { netiface: netIface })
}

async function getIngressIpStats(netIface: NetworkInterface) {
    return await invoke<IpStat[]>("get_ingress_ip_stats", { netiface: netIface })
}

async function getEngressIpStats(netIface: NetworkInterface) {
    return await invoke<IpStat[]>("get_egress_ip_stats", { netiface: netIface })
}

async function getEgressCountryStats(netIface: NetworkInterface) {
    return await invoke<CountryTraffic[]>("get_egress_country_stats", { netiface: netIface })
}

async function getIngressCountryStats(netIface: NetworkInterface) {
    return await invoke<CountryTraffic[]>("get_ingress_country_stats", { netiface: netIface })
}

export const Network = {
    getAvailableNetworkInterfaces,
    getProtocolsStats,
    getNetworkLogs,
    getArpIpStats,
    getIngressIpStats,
    getEngressIpStats,
    getEgressCountryStats,
    getIngressCountryStats,
    stopNetworkLogs
}