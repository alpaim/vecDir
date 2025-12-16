import type { StateStorage } from "zustand/middleware";
import { LazyStore } from "@tauri-apps/plugin-store";
import { createJSONStorage } from "zustand/middleware";

// TODO: maybe combine localstore + tauri for immediate initialization

const store = new LazyStore("app-state.json");

const tauriStorageAdapter: StateStorage = {
    getItem: async (name: string): Promise<string | null> => {
        try {
            await store.init();

            const value = await store.get(name);
            return value as string | null;
        }
        catch (e) {
            console.error("Storage read error:", e);
            return null;
        }
    },
    setItem: async (name: string, value: string): Promise<void> => {
        try {
            await store.set(name, value);

            await store.save();
        }
        catch (e) {
            console.error("Storage write error:", e);
        }
    },
    removeItem: async (name: string): Promise<void> => {
        await store.delete(name);
        await store.save();
    },
};

export const storage = createJSONStorage(() => tauriStorageAdapter);
