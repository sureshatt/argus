import { invoke } from '@tauri-apps/api/core'
import { create } from 'zustand'
import { NetworkInterface, Packet } from '../types'

interface NetState {
    currentInterface?: NetworkInterface,
    currentLogInterface?: NetworkInterface,
    selectedLog?: Packet,
    autoViewNewLog: boolean
}

interface NetActions {
    setCurrentInterface: (value: NetworkInterface) => void,
    setSelectedLog: (value: Packet) => void,
    setAutoViewNewLog: (value: boolean) => void,
}



export const useNetStore = create<NetState & NetActions>((set, get) => ({
    autoViewNewLog: true,
    currentLogInterface: undefined,
    async setCurrentInterface(value) {
        await invoke("set_selection", { selection: value.name });
        set({ currentInterface: value })
    },
    setSelectedLog(value) {
        set({ selectedLog: value })
    },
    setAutoViewNewLog(value) {
        if (get().autoViewNewLog != value) {
            set({ autoViewNewLog: value })
        }
    },
}))
