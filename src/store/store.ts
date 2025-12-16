import type { EmbeddingConfig, LLMConfig, Space } from "@/lib/vecdir/bindings";
import type { AppConfig } from "@/types/config";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { fileStore, storage } from "@/store/storage";

const APP_STORAGE_NAME = "app-storage";

interface AppState {
    // Runtime state (not saving)
    isBackendReady: boolean;

    // Persisted state
    config: AppConfig;
    spaces: Map<number, Space>;
    selectedSpace: number;

    // Actions
    setBackendStatus: (isReady: boolean) => void;
    syncWithBackend: () => Promise<void>;

    setSpaces: (spaces: Space[]) => void;
    addSpace: (space: Space) => void;
    selectSpace: (spaceId: number) => void;
}

interface PersistedState {
    version: number | undefined; // zustand persist

    config: AppConfig;
    spaces: [number, Space][]; // Serialized Map
    selectedSpace: number;
}

export const useAppState = create<AppState>()(
    persist(
        (set, _get) => ({
            isBackendReady: false,

            spaces: new Map(),
            selectedSpace: 0,
            config: { version: "" } as AppConfig,

            setBackendStatus: (isReady) => {
                set({ isBackendReady: isReady });
            },

            // Smart sync. First, get data from file (hydrated), then from backend.
            syncWithBackend: async () => {
                try {
                    await new Promise(resolve => setTimeout(resolve, 500));

                    // EXAMPLE
                    const remoteConfig: AppConfig = { version: "1.0.1" };

                    // Updating store
                    set({
                        config: remoteConfig,
                        isBackendReady: true,
                    });
                }
                catch (e) {
                    console.error("Sync failed:", e);
                }
            },

            setSpaces: (spacesArr) => {
                const spacesMap = new Map<number, Space>();
                spacesArr.forEach(s => spacesMap.set(s.id, s));
                set({ spaces: spacesMap });
            },

            addSpace: (space) => {
                set((state) => {
                    const newSpaces = new Map(state.spaces);
                    newSpaces.set(space.id, space);
                    return { spaces: newSpaces };
                });
            },

            selectSpace: (spaceId) => {
                set({ selectedSpace: spaceId });
            },
        }),
        {
            name: APP_STORAGE_NAME,
            storage,

            // what to save
            partialize: state => ({
                config: state.config,
                selectedSpace: state.selectedSpace,

                spaces: Array.from(state.spaces.entries()), // map -> array
            }),

            // restoring data
            merge: (persistedState, currentState) => {
                const typedPersisted = persistedState as PersistedState;

                // fallback on when there are some error on reading file
                if (!typedPersisted) {
                    return currentState;
                }

                return {
                    ...currentState,
                    ...typedPersisted,

                    spaces: new Map(typedPersisted.spaces || []), // array -> map

                    // resetting runtime flags
                    isBackendReady: false,
                };
            },
        },
    ),
);
