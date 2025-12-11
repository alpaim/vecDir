import type { AppConfig } from "@/types/config";

import { create } from "zustand";

interface AppState {
    config: AppConfig;

    setConfig: (config: AppConfig) => Promise<void>;
}

// TODO: make persist with automatic syncing with getConfig from backend: https://zustand.docs.pmnd.rs/integrations/persisting-store-data#typescript-simple-example
export const useAppState = create<AppState>()(set => ({
    config: { version: "" },

    setConfig: async (config) => {
        // making request to backend
        await new Promise(resolve => setTimeout(resolve, 500));

        set({ config });
    },
}));
