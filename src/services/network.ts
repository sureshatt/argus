import { invoke } from "@tauri-apps/api/core";
import { NetworkInterface, Packet } from "../types";


async function getAvailableNetworkInterfaces() {
    return await invoke<NetworkInterface[]>("get_network_interfaces")
}

async function getNetworkLogs(selection: string) {
    return await invoke<Packet[]>("dump", { selection })
}

async function stopNetworkLogs() {
    return await invoke("stop_dump")
}

export const Network = {
    getAvailableNetworkInterfaces,
    getNetworkLogs,
    stopNetworkLogs
}