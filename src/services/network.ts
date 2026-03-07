import { invoke } from "@tauri-apps/api/core";
import { NetworkInterface, Packet } from "../types";
import { info } from "@tauri-apps/plugin-log";


async function getAvailableNetworkInterfaces() {
    info("Fetching available network interfaces");
    return await invoke<NetworkInterface[]>("get_network_interfaces")
}

async function getNetworkLogs(selection: string) {
    info("Fetching network logs for selection: " + selection);
    return await invoke<Packet[]>("dump", { selection })
}

async function stopNetworkLogs() {
    info("Stopping network logs");
    return await invoke("stop_dump")
}

async function checkCapturePermissions(): Promise<boolean> {
    info("Checking BPF capture permissions");
    return await invoke<boolean>("check_capture_permissions");
}

export const Network = {
    getAvailableNetworkInterfaces,
    getNetworkLogs,
    stopNetworkLogs,
    checkCapturePermissions
}