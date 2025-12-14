import type { Space } from "@/lib/vecdir/bindings";
import type { AppConfig } from "@/types/config";
import { create } from "zustand";

interface AppState {
    isBackendReady: boolean;
    config: AppConfig;
    spaces: Space[];

    setBackendStatus: (isReady: boolean) => void;
    setConfig: (config: AppConfig) => Promise<void>;
}

// TODO: make persist with automatic syncing with getConfig from backend: https://zustand.docs.pmnd.rs/integrations/persisting-store-data#typescript-simple-example
export const useAppState = create<AppState>()(set => ({
    isBackendReady: false,

    spaces: [],

    config: { version: "" },

    setBackendStatus: (isReady) => {
        set({ isBackendReady: isReady });
    },

    setConfig: async (config) => {
        // making request to backend
        await new Promise(resolve => setTimeout(resolve, 500));

        set({ config });
    },
}));
