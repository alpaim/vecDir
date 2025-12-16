import type { EmbeddingConfig, LLMConfig, Space } from "@/lib/vecdir/bindings";
import type { AppConfig } from "@/types/config";
import { create } from "zustand";

interface AppState {
    isBackendReady: boolean;

    config: AppConfig;

    spaces: Map<number, Space>;
    selectedSpace: number;

    setBackendStatus: (isReady: boolean) => void;

    setConfig: (config: AppConfig) => Promise<void>;

    setSpaces: (spaces: Space[]) => void;
    addSpace: (space: Space) => void;
    selectSpace: (spaceId: number) => void;
}

// TODO: make persist with automatic syncing with getConfig from backend: https://zustand.docs.pmnd.rs/integrations/persisting-store-data#typescript-simple-example
export const useAppState = create<AppState>()(set => ({
    isBackendReady: false,

    spaces: new Map(),
    selectedSpace: 0,

    config: { version: "" },

    setBackendStatus: (isReady) => {
        set({ isBackendReady: isReady });
    },

    setConfig: async (config) => {
        // making request to backend
        await new Promise(resolve => setTimeout(resolve, 500));

        set({ config });
    },

    setSpaces: (spaces) => {
        const spacesMap = new Map<number, Space>();

        for (const space of spaces) {
            spacesMap.set(space.id, space);
        }

        set({ spaces: spacesMap });
    },

    addSpace: (space) => {
        set(state => ({
            spaces: {
                ...state.spaces,
                [space.id]: space,
            },
        }));
    },

    selectSpace: (spaceId) => {
        set({ selectedSpace: spaceId });
    },
}));
